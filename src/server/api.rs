use std::path::PathBuf;
use std::sync::Arc;

use axum::extract::{FromRef, Path, Query, State};
use axum::middleware::from_extractor_with_state;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::NaiveDate;
use gluer::{generate, metadata};
use hyper::StatusCode;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::error;

use super::auth::{Auth, Login};
use crate::db::*;
use crate::error::{Error, Result};
use crate::mail::{self, account_is_valid};
use crate::provider;
use crate::provider::dnb::BookData;

/// User configuration.
#[derive(Debug, Clone)]
pub struct UserConfig {
    pub file: PathBuf,
    pub delimiter: u8,
}

/// Project state.
#[derive(Debug, Clone)]
pub struct Project {
    db: Arc<AtomicDatabase>,
    user: Option<Arc<UserConfig>>,
    client: Client,
    auth: Auth,
}

impl FromRef<Project> for Auth {
    fn from_ref(input: &Project) -> Self {
        input.auth.clone()
    }
}

impl Project {
    pub fn new(db: AtomicDatabase, user: Option<UserConfig>, auth: Auth) -> Self {
        Self {
            db: Arc::new(db),
            user: user.map(Arc::new),
            client: Client::new(),
            auth,
        }
    }
}

pub fn routes(state: Project) -> Router {
    generate! {
        prefix = "/api",
        routes = {
            // general
            "/about" = get(about),
            "/settings" = get(settings_get).post(settings_update),
            "/stats" = get(stats),
            "/session" = get(session),
            // books
            "/book" = get(book_search).post(book_add),
            "/book/{id}" = get(book_fetch).post(book_update).delete(book_delete),
            "/book-id" = post(book_generate_id),
            "/book-fetch/{isbn}" = get(book_fetch_data),
            // user
            "/user" = get(user_search).post(user_add),
            "/user/{account}" = get(user_fetch).post(user_update).delete(user_delete),
            "/user-fetch/{account}" = get(user_fetch_data),
            "/user-update-roles" = post(user_update_roles),
            // category
            "/category" = get(category_list).post(category_add),
            "/category/{id}" = post(category_update).delete(category_delete),
            "/category-refs/{id}" = get(category_references),
            // lending
            "/lending/lend" = post(lending_lend),
            "/lending/return" = post(lending_return),
            "/lending/reserve" = post(lending_reserve),
            "/lending/release" = post(lending_release),
            "/overdues" = get(lending_overdues),
            // mail
            "/notify" = post(mail_notify),
        },
        files = [
            "src/db",
            "src/server",
            "src/error.rs",
            "src/provider/dnb.rs"
        ],
        output = "lib-view/src/lib/api.ts",
    }
    // all routes require authorization
    .route_layer(from_extractor_with_state::<Login, Auth>(state.auth.clone()))
    .fallback(|| async { (StatusCode::NOT_FOUND, Json(Error::NothingFound)) })
    .with_state(state)
}

#[metadata]
#[derive(Debug, Serialize)]
struct About {
    name: &'static str,
    version: &'static str,
    repository: &'static str,
    authors: Vec<&'static str>,
    description: &'static str,
    license: &'static str,
}

/// Returns info about this project.
#[metadata]
async fn about() -> Json<About> {
    use crate::util::*;
    Json(About {
        name: PKG_NAME,
        version: PKG_VERSION,
        repository: PKG_REPOSITORY,
        authors: PKG_AUTHORS.split(':').collect(),
        description: PKG_DESCRIPTION,
        license: PKG_LICENSE,
    })
}

/// Returns the project settings.
/// They are fetched when opening a project, so that this function only
/// returns copies of the cached version.
#[metadata(custom = [Result])]
async fn settings_get(State(project): State<Project>) -> Result<Json<Settings>> {
    Ok(Json(project.db.read().settings()))
}

/// Updates project settings.
#[metadata(custom = [Result])]
async fn settings_update(
    State(project): State<Project>,
    Json(settings): Json<Settings>,
) -> Result<()> {
    project.db.write().settings_update(settings)?;
    Ok(())
}

/// Returns the project statistics.
#[metadata(custom = [Result])]
async fn stats(State(project): State<Project>) -> Result<Json<Stats>> {
    Ok(Json(project.db.read().stats()?))
}

/// Returns the project statistics.
#[metadata(custom = [Result])]
async fn session(login: Login) -> Result<Json<Login>> {
    Ok(Json(login))
}

// Book

/// Returns the book with the given `id`.
#[metadata(custom = [Result])]
async fn book_fetch(State(project): State<Project>, Path(id): Path<String>) -> Result<Json<Book>> {
    Ok(Json(project.db.read().books.fetch(&id)?))
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct Search {
    query: String,
    offset: usize,
    limit: usize,
}

impl Default for Search {
    fn default() -> Self {
        Self {
            query: Default::default(),
            offset: 0,
            limit: 100,
        }
    }
}

/// Search result containing the total number of found records.
#[metadata]
#[derive(Serialize)]
struct Limited<T: Serialize> {
    /// Total number of results (without limit)
    total: usize,
    rows: Vec<T>,
}
impl<T: Serialize> From<(usize, Vec<T>)> for Limited<T> {
    fn from(value: (usize, Vec<T>)) -> Self {
        Limited {
            total: value.0,
            rows: value.1,
        }
    }
}

/// Preforms a simple media search with the given `query`.
#[metadata(custom = [Result])]
async fn book_search(
    State(project): State<Project>,
    Query(params): Query<BookSearch>,
) -> Result<Json<Limited<Book>>> {
    Ok(Json(project.db.read().books.search(&params)?.into()))
}

/// Adds a new book.
#[metadata(custom = [Result])]
async fn book_add(State(project): State<Project>, Json(book): Json<Book>) -> Result<Json<Book>> {
    let db = &mut *project.db.write();
    Ok(Json(db.books.add(book, &db.categories)?))
}

/// Updates the book and all references if its id changes.
#[metadata(custom = [Result])]
async fn book_update(
    State(project): State<Project>,
    Path(id): Path<String>,
    Json(book): Json<Book>,
) -> Result<Json<Book>> {
    let db = &mut *project.db.write();
    Ok(Json(db.books.update(&id, book, &db.categories)?))
}

/// Deletes the book including the its authors.
/// Also borrowers & reservations for this book are removed.
#[metadata(custom = [Result])]
async fn book_delete(State(project): State<Project>, Path(id): Path<String>) -> Result<()> {
    project.db.write().books.delete(&id)
}

/// Generates a new book id.
#[metadata(custom = [Result])]
async fn book_generate_id(
    State(project): State<Project>,
    Json(book): Json<Book>,
) -> Result<Json<String>> {
    Ok(Json(project.db.write().books.generate_id(&book)?))
}

/// Fetch the data of the book from the DNB an their like.
#[metadata(custom = [Result])]
async fn book_fetch_data(
    State(project): State<Project>,
    Path(isbn): Path<String>,
) -> Result<Json<BookData>> {
    Ok(Json(provider::dnb::fetch(&project.client, &isbn).await?))
}

// User

/// Returns the user with the given `account`.
#[metadata(custom = [Result])]
async fn user_fetch(
    State(project): State<Project>,
    Path(account): Path<String>,
) -> Result<Json<User>> {
    Ok(Json(project.db.read().users.fetch(&account)?))
}

/// Performs a simple user search with the given `text`.
#[metadata(custom = [Result])]
async fn user_search(
    State(project): State<Project>,
    Query(params): Query<UserSearch>,
) -> Result<Json<Limited<User>>> {
    Ok(Json(project.db.read().users.search(&params)?.into()))
}

/// Adds a new user.
#[metadata(custom = [Result])]
async fn user_add(State(project): State<Project>, Json(user): Json<User>) -> Result<Json<User>> {
    Ok(Json(project.db.write().users.add(user)?))
}

/// Updates the user and all references if its account changes.
#[metadata(custom = [Result])]
async fn user_update(
    State(project): State<Project>,
    Path(account): Path<String>,
    Json(user): Json<User>,
) -> Result<Json<User>> {
    let db = &mut *project.db.write();
    Ok(Json(db.users.update(&account, user, &mut db.books)?))
}

/// Deletes the user.
///
/// Returns a `Error::StillReferenced` if there are any borrows or reservations left.
#[metadata(custom = [Result])]
async fn user_delete(State(project): State<Project>, Path(account): Path<String>) -> Result<()> {
    let db = &mut *project.db.write();
    db.users.delete(&account, &db.books)
}

/// Fetch the data of the user from the specified user file.
#[metadata(custom = [Result])]
async fn user_fetch_data(
    State(project): State<Project>,
    Path(account): Path<String>,
) -> Result<Json<User>> {
    if let Some(user) = &project.user {
        Ok(Json(super::provider::user::search(
            &user.file,
            user.delimiter,
            &account,
        )?))
    } else {
        Err(Error::NothingFound)
    }
}

/// Deletes the roles from all users and inserts the new roles.
///
/// The roles of all users not contained in the given list are cleared.
#[metadata(custom = [Result])]
async fn user_update_roles(State(project): State<Project>) -> Result<()> {
    if let Some(user) = &project.user {
        let users = super::provider::user::load_roles(&user.file, user.delimiter)?;
        project.db.write().users.update_roles(&users)
    } else {
        Err(Error::NothingFound)
    }
}

// Category

/// Fetches and returns all categories.
#[metadata(custom = [Result])]
async fn category_list(State(project): State<Project>) -> Result<Json<Vec<Category>>> {
    Ok(Json(project.db.read().categories.list()?))
}

/// Adds a new category.
#[metadata(custom = [Result])]
async fn category_add(
    State(project): State<Project>,
    Json(category): Json<Category>,
) -> Result<Json<Category>> {
    Ok(Json(project.db.write().categories.add(category)?))
}

/// Updates the category and all references.
#[metadata(custom = [Result])]
async fn category_update(
    State(project): State<Project>,
    Path(id): Path<String>,
    Json(category): Json<Category>,
) -> Result<Json<Category>> {
    let db = &mut *project.db.write();
    Ok(Json(db.categories.update(&id, category, &mut db.books)?))
}

/// Removes the category or returns a `Error::StillReferenced` if it is still in use.
#[metadata(custom = [Result])]
async fn category_delete(State(project): State<Project>, Path(id): Path<String>) -> Result<()> {
    let db = &mut *project.db.write();
    db.categories.delete(&id, &db.books)
}

/// Returns the number of books in this category.
#[metadata(custom = [Result])]
async fn category_references(
    State(project): State<Project>,
    Path(id): Path<String>,
) -> Result<Json<usize>> {
    Ok(Json(project.db.read().books.in_category(&id)?))
}

// Lending
#[metadata]
#[derive(Debug, Deserialize)]
struct LendParams {
    id: String,
    account: String,
    #[meta(into = String)]
    /// ISO date format: YYYY-MM-DD
    deadline: NaiveDate,
}

/// Lends the book to the specified user.
#[metadata(custom = [Result])]
async fn lending_lend(
    State(project): State<Project>,
    Query(params): Query<LendParams>,
) -> Result<Json<Book>> {
    Ok(Json(project.db.write().lend(
        &params.id,
        &params.account,
        params.deadline,
    )?))
}
#[metadata]
#[derive(Debug, Deserialize)]
struct ReturnParams {
    id: String,
}

/// Returns the book.
#[metadata(custom = [Result])]
async fn lending_return(
    State(project): State<Project>,
    Query(params): Query<ReturnParams>,
) -> Result<Json<Book>> {
    Ok(Json(project.db.write().return_back(&params.id)?))
}
#[metadata]
#[derive(Debug, Deserialize)]
struct ReserveParams {
    id: String,
    account: String,
}

/// Creates a reservation for the borrowed book.
#[metadata(custom = [Result])]
async fn lending_reserve(
    State(project): State<Project>,
    Query(params): Query<ReserveParams>,
) -> Result<Json<Book>> {
    Ok(Json(
        project.db.write().reserve(&params.id, &params.account)?,
    ))
}

/// Removes the reservation from the specified book.
#[metadata(custom = [Result])]
async fn lending_release(
    State(project): State<Project>,
    Query(params): Query<ReturnParams>,
) -> Result<Json<Book>> {
    Ok(Json(project.db.write().release(&params.id)?))
}

/// Returns the list of expired borrowing periods.
#[metadata(custom = [Result])]
async fn lending_overdues(State(project): State<Project>) -> Result<Json<Vec<Overdue>>> {
    Ok(Json(project.db.read().overdues()?))
}

// Mail Notifications
#[metadata]
#[derive(Debug, Deserialize)]
struct Message {
    account: String,
    subject: String,
    body: String,
}

#[metadata(custom = [Result])]
async fn mail_notify(
    State(project): State<Project>,
    Json(messages): Json<Vec<Message>>,
) -> Result<()> {
    let settings = project.db.read().settings();

    for Message {
        account,
        subject,
        body,
    } in messages
    {
        if !account_is_valid(&settings.mail_from) {
            error!("Invalid sender {}", settings.mail_from);
            return Err(Error::Arguments);
        }
        let account = account.trim();
        if !account_is_valid(account) {
            error!("Invalid recipient {account}");
            return Err(Error::InvalidUser);
        }

        mail::send(
            &settings.mail_host,
            &settings.mail_password,
            &settings.mail_from,
            account,
            &subject,
            &body,
        )
        .await?;
    }
    Ok(())
}
