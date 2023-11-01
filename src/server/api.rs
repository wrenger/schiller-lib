use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

use axum::extract::{FromRef, Path, Query, State};
use axum::middleware::from_extractor_with_state;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use hyper::StatusCode;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::error;

use super::auth::{Auth, Login};
use crate::db::{self, UserSearch};
use crate::error::{Error, Result};
use crate::mail::{self, account_is_valid};
use crate::provider;

#[derive(Debug, Clone)]
pub struct Project {
    db: Arc<Mutex<db::Database>>,
    user_file: Arc<PathBuf>,
    user_delimiter: u8,
    client: Client,
    auth: Auth,
}

impl FromRef<Project> for Auth {
    fn from_ref(input: &Project) -> Self {
        input.auth.clone()
    }
}

impl Project {
    pub fn new(db: db::Database, user_file: PathBuf, user_delimiter: u8, auth: Auth) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            user_file: Arc::new(user_file),
            user_delimiter,
            client: Client::new(),
            auth,
        }
    }

    fn db<'a>(&'a self) -> MutexGuard<'a, db::Database> {
        self.db.lock().unwrap()
    }
}

pub fn routes(state: Project) -> Router {
    Router::new()
        // general
        .route("/about", get(about))
        .route("/settings", get(settings_get).post(settings_update))
        .route("/stats", get(stats))
        .route("/session", get(session))
        // books
        .route("/book", get(book_search).post(book_add))
        .route(
            "/book/:id",
            get(book_fetch).patch(book_update).delete(book_delete),
        )
        .route("/book-search", get(book_search_advanced))
        .route("/book-id", post(book_generate_id))
        .route("/book-fetch/:isbn", get(book_fetch_data))
        // user
        .route("/user", get(user_search).post(user_add))
        .route(
            "/user/:account",
            get(user_fetch).patch(user_update).delete(user_delete),
        )
        .route("/user-search", get(user_search_advanced))
        .route("/user-fetch/:account", get(user_fetch_data))
        .route("/user-update-roles", patch(user_update_roles))
        // category
        .route("/category", get(category_list).post(category_add))
        .route(
            "/category/:id",
            patch(category_update).delete(category_delete),
        )
        .route("/category-refs/:id", get(category_references))
        // lending
        .route("/lending/lend", patch(lending_lend))
        .route("/lending/return", patch(lending_return))
        .route("/lending/reserve", patch(lending_reserve))
        .route("/lending/release", patch(lending_release))
        .route("/overdues", get(lending_overdues))
        // mail
        .route("/notify", post(mail_notify))
        // all routes require authorization
        .route_layer(from_extractor_with_state::<Login, Auth>(state.auth.clone()))
        .fallback(|| async { (StatusCode::NOT_FOUND, Json(Error::NothingFound)) })
        .with_state(state)
}

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
async fn about() -> Json<About> {
    use crate::*;
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
async fn settings_get(State(project): State<Project>) -> Result<Json<db::Settings>> {
    Ok(Json(db::settings::fetch(&project.db())?))
}

/// Updates project settings.
async fn settings_update(
    State(project): State<Project>,
    Json(settings): Json<db::Settings>,
) -> Result<()> {
    let db = project.db();
    db::settings::update(&db, &settings)?;
    Ok(())
}

/// Returns the project statistics.
async fn stats(State(project): State<Project>) -> Result<Json<db::Stats>> {
    Ok(Json(db::stats::fetch(&project.db())?))
}

/// Returns the project statistics.
async fn session(login: Login) -> Result<Json<Login>> {
    Ok(Json(login))
}

// Book

/// Returns the book with the given `id`.
async fn book_fetch(
    State(project): State<Project>,
    Path(id): Path<String>,
) -> Result<Json<db::Book>> {
    Ok(Json(db::book::fetch(&project.db(), &id)?))
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct SimpleSearch {
    query: String,
    offset: usize,
    limit: usize,
}

impl Default for SimpleSearch {
    fn default() -> Self {
        Self {
            query: Default::default(),
            offset: 0,
            limit: 100,
        }
    }
}

/// Preforms a simple media search with the given `query`.
async fn book_search(
    State(project): State<Project>,
    Query(params): Query<db::BookSearch>,
) -> Result<Json<Vec<db::book::Book>>> {
    Ok(Json(db::book::search(&project.db(), &params)?))
}

// /// Performs an advanced media search with the given search parameters.
async fn book_search_advanced(
    State(project): State<Project>,
    Query(params): Query<db::BookAdvancedSearch>,
) -> Result<Json<Vec<db::book::Book>>> {
    Ok(Json(db::book::search_advanced(&project.db(), &params)?))
}

/// Adds a new book.
async fn book_add(State(project): State<Project>, Json(book): Json<db::Book>) -> Result<()> {
    Ok(db::book::add(&project.db(), &book)?)
}

/// Updates the book and all references if its id changes.
async fn book_update(
    State(project): State<Project>,
    Path(id): Path<String>,
    Json(book): Json<db::Book>,
) -> Result<()> {
    Ok(db::book::update(&project.db(), &id, &book)?)
}

/// Deletes the book including the its authors.
/// Also borrowers & reservations for this book are removed.
async fn book_delete(State(project): State<Project>, Path(id): Path<String>) -> Result<()> {
    Ok(db::book::delete(&project.db(), &id)?)
}

/// Generates a new book id.
async fn book_generate_id(
    State(project): State<Project>,
    Json(book): Json<db::Book>,
) -> Result<Json<String>> {
    Ok(Json(db::book::generate_id(&project.db(), &book)?))
}

/// Fetch the data of the book from the DNB an their like.
async fn book_fetch_data(
    State(project): State<Project>,
    Path(isbn): Path<String>,
) -> Result<Json<provider::dnb::BookData>> {
    let settings = db::settings::fetch(&project.db())?;

    Ok(Json(
        provider::dnb::fetch(project.client, &settings.dnb_token, &isbn).await?,
    ))
}

// User

/// Returns the user with the given `account`.
async fn user_fetch(
    State(project): State<Project>,
    Path(account): Path<String>,
) -> Result<Json<db::User>> {
    Ok(Json(db::user::fetch(&project.db(), &account)?))
}

/// Performs a simple user search with the given `text`.
async fn user_search(
    State(project): State<Project>,
    Query(params): Query<UserSearch>,
) -> Result<Json<Vec<db::user::User>>> {
    Ok(Json(db::user::search(&project.db(), &params)?))
}

/// Performs a simple user search with the given `text`.
async fn user_search_advanced(
    State(project): State<Project>,
    Query(params): Query<db::UserAdvancedSearch>,
) -> Result<Json<Vec<db::user::User>>> {
    Ok(Json(db::user::search_advanced(&project.db(), &params)?))
}

/// Adds a new user.
async fn user_add(State(project): State<Project>, Json(user): Json<db::User>) -> Result<()> {
    Ok(db::user::add(&project.db(), &user)?)
}

/// Updates the user and all references if its account changes.
async fn user_update(
    State(project): State<Project>,
    Path(account): Path<String>,
    Json(user): Json<db::User>,
) -> Result<()> {
    Ok(db::user::update(&project.db(), &account, &user)?)
}

/// Deletes the user.
/// This includes all its borrows & reservations.
async fn user_delete(State(project): State<Project>, Path(account): Path<String>) -> Result<()> {
    Ok(db::user::delete(&project.db(), &account)?)
}

/// Fetch the data of the book from the DNB an their like.
async fn user_fetch_data(
    State(project): State<Project>,
    Path(account): Path<String>,
) -> Result<Json<db::User>> {
    Ok(Json(super::provider::user::search(
        &project.user_file,
        project.user_delimiter,
        &account,
    )?))
}

/// Deletes the roles from all users and inserts the new roles.
///
/// The roles of all users not contained in the given list are cleared.
async fn user_update_roles(State(project): State<Project>) -> Result<()> {
    let users = super::provider::user::load_roles(&project.user_file, project.user_delimiter)?;
    db::user::update_roles(&project.db(), &users)
}

// Category

/// Fetches and returns all categories.
async fn category_list(
    State(project): State<Project>,
) -> Result<Json<Vec<db::category::Category>>> {
    Ok(Json(db::category::list(&project.db())?))
}

/// Adds a new category.
async fn category_add(
    State(project): State<Project>,
    Json(category): Json<db::Category>,
) -> Result<()> {
    Ok(db::category::add(&project.db(), &category)?)
}

/// Updates the category and all references.
async fn category_update(
    State(project): State<Project>,
    Path(id): Path<String>,
    Json(category): Json<db::Category>,
) -> Result<()> {
    Ok(db::category::update(&project.db(), &id, &category)?)
}

/// Removes the category or returns a `Error::Logic` if it is still in use.
async fn category_delete(State(project): State<Project>, Path(id): Path<String>) -> Result<()> {
    Ok(db::category::delete(&project.db(), &id)?)
}

/// Returns the number of books in this category.
async fn category_references(
    State(project): State<Project>,
    Path(id): Path<String>,
) -> Result<Json<i64>> {
    Ok(Json(db::category::references(&project.db(), &id)?))
}

// Lending

#[derive(Debug, Deserialize)]
struct LendParams {
    id: String,
    account: String,
    /// ISO date format: YYYY-MM-DD
    deadline: String,
}

/// Lends the book to the specified user.
async fn lending_lend(
    State(project): State<Project>,
    Query(params): Query<LendParams>,
) -> Result<Json<db::Book>> {
    Ok(Json(db::lending::lend(
        &project.db(),
        &params.id,
        &params.account,
        &params.deadline,
    )?))
}

#[derive(Debug, Deserialize)]
struct ReturnParams {
    id: String,
}

/// Returns the book.
async fn lending_return(
    State(project): State<Project>,
    Query(params): Query<ReturnParams>,
) -> Result<Json<db::Book>> {
    Ok(Json(db::lending::return_back(&project.db(), &params.id)?))
}

#[derive(Debug, Deserialize)]
struct ReserveParams {
    id: String,
    account: String,
}

/// Creates a reservation for the borrowed book.
async fn lending_reserve(
    State(project): State<Project>,
    Query(params): Query<ReserveParams>,
) -> Result<Json<db::Book>> {
    Ok(Json(db::lending::reserve(
        &project.db(),
        &params.id,
        &params.account,
    )?))
}

/// Removes the reservation from the specified book.
async fn lending_release(
    State(project): State<Project>,
    Query(params): Query<ReturnParams>,
) -> Result<Json<db::Book>> {
    Ok(Json(db::lending::release(&project.db(), &params.id)?))
}

/// Returns the list of expired borrowing periods.
async fn lending_overdues(
    State(project): State<Project>,
) -> Result<Json<Vec<(db::book::Book, db::user::User)>>> {
    Ok(Json(db::lending::overdues(&project.db())?))
}

// Mail Notifications

#[derive(Debug, Deserialize)]
struct Message {
    account: String,
    subject: String,
    body: String,
}

async fn mail_notify(
    State(project): State<Project>,
    Json(Message {
        account,
        subject,
        body,
    }): Json<Message>,
) -> Result<()> {
    let settings = db::settings::fetch(&project.db())?;

    if !account_is_valid(&settings.mail_from) {
        error!("Invalid sender {}", settings.mail_from);
        return Err(Error::Logic);
    }
    let account = account.trim();
    if !account_is_valid(account) {
        error!("Invalid recipient {}", settings.mail_from);
        return Err(Error::InvalidUser);
    }

    let from = format!("{}@{}", settings.mail_from, settings.mail_host);
    let to = format!("{account}@{}", settings.mail_host);

    mail::send(
        &settings.mail_host,
        &settings.mail_password,
        &from,
        &to,
        &subject,
        &body,
    )
}
