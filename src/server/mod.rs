use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::routing::*;
use axum::{extract::State, Json, Router};
use hyper::Server;
use serde::{Deserialize, Serialize};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing::debug;

use crate::db;
use crate::error::Result;

pub async fn start(host: SocketAddr, db: db::Database) {
    let app = Router::new()
        // general
        .route("/api/about", get(about))
        .route("/api/settings", get(settings_get).post(settings_update))
        .route("/api/stats", get(stats))
        // books
        .route("/api/book", get(book_search).post(book_add))
        .route(
            "/api/book/:id",
            get(book_fetch).patch(book_update).delete(book_delete),
        )
        .route("/api/book-search", get(book_search_advanced))
        .route("/api/book-id", get(book_generate_id))
        // user
        .route("/api/user", get(user_search).post(user_add))
        .route(
            "/api/user/:account",
            get(user_fetch).patch(user_update).delete(user_delete),
        )
        .route("/api/user-search", get(user_search_advanced))
        // category
        .route("/api/category", get(category_list).post(category_add))
        .route(
            "/api/category/:id",
            patch(category_update).delete(category_delete),
        )
        .route("/api/category-refs/:id", get(category_references))
        // lending
        .route("/api/lending/lend", patch(lending_lend))
        .route("/api/lending/return", patch(lending_return))
        .route("/api/lending/reserve", patch(lending_reserve))
        .route("/api/lending/release", patch(lending_release))
        .route("/api/overdues", get(lending_overdues))
        // TODO: Mail, DNB Requests, User Role Updating
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(Arc::new(Mutex::new(db)));

    debug!("Listening on {host}");
    Server::bind(&host)
        .serve(app.into_make_service())
        .await
        .unwrap();
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
async fn settings_get(State(db): State<Arc<Mutex<db::Database>>>) -> Result<Json<db::Settings>> {
    Ok(Json(db::settings::fetch(&db.lock().unwrap())?))
}

/// Updates project settings.
async fn settings_update(
    State(db): State<Arc<Mutex<db::Database>>>,
    Json(settings): Json<db::Settings>,
) -> Result<()> {
    let db = db.lock().unwrap();
    db::settings::update(&db, &settings)?;
    Ok(())
}

/// Returns the project statistics.
async fn stats(State(db): State<Arc<Mutex<db::Database>>>) -> Result<Json<db::Stats>> {
    Ok(Json(db::stats::fetch(&db.lock().unwrap())?))
}

// Book

/// Returns the book with the given `id`.
async fn book_fetch(
    State(db): State<Arc<Mutex<db::Database>>>,
    Path(id): Path<String>,
) -> Result<Json<db::Book>> {
    Ok(Json(db::book::fetch(&db.lock().unwrap(), &id)?))
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
    State(db): State<Arc<Mutex<db::Database>>>,
    Query(params): Query<SimpleSearch>,
) -> Result<Json<Vec<db::book::Book>>> {
    Ok(Json(db::book::search(&db.lock().unwrap(), &params.query)?))
}

// /// Performs an advanced media search with the given search parameters.
async fn book_search_advanced(
    State(db): State<Arc<Mutex<db::Database>>>,
    Query(params): Query<db::BookSearch>,
) -> Result<Json<Vec<db::book::Book>>> {
    Ok(Json(db::book::search_advanced(
        &db.lock().unwrap(),
        &params,
    )?))
}

/// Adds a new book.
async fn book_add(
    State(db): State<Arc<Mutex<db::Database>>>,
    Json(book): Json<db::Book>,
) -> Result<()> {
    Ok(db::book::add(&db.lock().unwrap(), &book)?)
}

/// Updates the book and all references if its id changes.
async fn book_update(
    State(db): State<Arc<Mutex<db::Database>>>,
    Path(id): Path<String>,
    Json(book): Json<db::Book>,
) -> Result<()> {
    Ok(db::book::update(&db.lock().unwrap(), &id, &book)?)
}

/// Deletes the book including the its authors.
/// Also borrowers & reservations for this book are removed.
async fn book_delete(
    State(db): State<Arc<Mutex<db::Database>>>,
    Path(id): Path<String>,
) -> Result<()> {
    Ok(db::book::delete(&db.lock().unwrap(), &id)?)
}

/// Generates a new book id.
async fn book_generate_id(
    State(db): State<Arc<Mutex<db::Database>>>,
    Json(book): Json<db::Book>,
) -> Result<Json<String>> {
    Ok(Json(db::book::generate_id(&db.lock().unwrap(), &book)?))
}

// User

/// Returns the user with the given `account`.
async fn user_fetch(
    State(db): State<Arc<Mutex<db::Database>>>,
    Path(account): Path<String>,
) -> Result<Json<db::User>> {
    Ok(Json(db::user::fetch(&db.lock().unwrap(), &account)?))
}

/// Performs a simple user search with the given `text`.
async fn user_search(
    State(db): State<Arc<Mutex<db::Database>>>,
    Query(params): Query<SimpleSearch>,
) -> Result<Json<Vec<db::user::User>>> {
    Ok(Json(db::user::search(&db.lock().unwrap(), &params.query)?))
}

/// Performs a simple user search with the given `text`.
async fn user_search_advanced(
    State(db): State<Arc<Mutex<db::Database>>>,
    Query(params): Query<db::UserSearch>,
) -> Result<Json<Vec<db::user::User>>> {
    Ok(Json(db::user::search_advanced(
        &db.lock().unwrap(),
        &params,
    )?))
}

/// Adds a new user.
async fn user_add(
    State(db): State<Arc<Mutex<db::Database>>>,
    Json(user): Json<db::User>,
) -> Result<()> {
    Ok(db::user::add(&db.lock().unwrap(), &user)?)
}

/// Updates the user and all references if its account changes.
async fn user_update(
    State(db): State<Arc<Mutex<db::Database>>>,
    Path(account): Path<String>,
    Json(user): Json<db::User>,
) -> Result<()> {
    Ok(db::user::update(&db.lock().unwrap(), &account, &user)?)
}

/// Deletes the user.
/// This includes all its borrows & reservations.
async fn user_delete(
    State(db): State<Arc<Mutex<db::Database>>>,
    Path(account): Path<String>,
) -> Result<()> {
    Ok(db::user::delete(&db.lock().unwrap(), &account)?)
}

// /// Deletes the roles from all users and inserts the new roles.
// ///
// /// The roles of all users not contained in the given list are cleared.
// async fn user_update_roles(
//     State(db): State<Arc<Mutex<db::Database>>>,
//     users: Vec<(String, String)>,
// ) -> Result<()> {
//     let db = &db.lock().unwrap();
//     let users: Vec<(&str, &str)> = users
//         .iter()
//         .map(|(u, r)| (u.as_str(), r.as_str()))
//         .collect();
//     db::user::update_roles(db, &users)
// }

// // Category

/// Fetches and returns all categories.
async fn category_list(
    State(db): State<Arc<Mutex<db::Database>>>,
) -> Result<Json<Vec<db::category::Category>>> {
    Ok(Json(db::category::list(&db.lock().unwrap())?))
}

/// Adds a new category.
async fn category_add(
    State(db): State<Arc<Mutex<db::Database>>>,
    Json(category): Json<db::Category>,
) -> Result<()> {
    Ok(db::category::add(&db.lock().unwrap(), &category)?)
}

/// Updates the category and all references.
async fn category_update(
    State(db): State<Arc<Mutex<db::Database>>>,
    Path(id): Path<String>,
    Json(category): Json<db::Category>,
) -> Result<()> {
    Ok(db::category::update(&db.lock().unwrap(), &id, &category)?)
}

/// Removes the category or returns a `Error::Logic` if it is still in use.
async fn category_delete(
    State(db): State<Arc<Mutex<db::Database>>>,
    Path(id): Path<String>,
) -> Result<()> {
    Ok(db::category::delete(&db.lock().unwrap(), &id)?)
}

/// Returns the number of books in this category.
async fn category_references(
    State(db): State<Arc<Mutex<db::Database>>>,
    Path(id): Path<String>,
) -> Result<Json<i64>> {
    Ok(Json(db::category::references(&db.lock().unwrap(), &id)?))
}

// Lending

#[derive(Debug, Deserialize)]
struct LendParams {
    id: String,
    account: String,
    days: usize,
}

/// Lends the book to the specified user.
async fn lending_lend(
    State(db): State<Arc<Mutex<db::Database>>>,
    Query(params): Query<LendParams>,
) -> Result<Json<db::Book>> {
    Ok(Json(db::lending::lend(
        &db.lock().unwrap(),
        &params.id,
        &params.account,
        params.days,
    )?))
}

#[derive(Debug, Deserialize)]
struct ReturnParams {
    id: String,
}

/// Returns the book.
async fn lending_return(
    State(db): State<Arc<Mutex<db::Database>>>,
    Query(params): Query<ReturnParams>,
) -> Result<Json<db::Book>> {
    Ok(Json(db::lending::return_back(
        &db.lock().unwrap(),
        &params.id,
    )?))
}

#[derive(Debug, Deserialize)]
struct ReserveParams {
    id: String,
    account: String,
}

/// Creates a reservation for the borrowed book.
async fn lending_reserve(
    State(db): State<Arc<Mutex<db::Database>>>,
    Query(params): Query<ReserveParams>,
) -> Result<Json<db::Book>> {
    Ok(Json(db::lending::reserve(
        &db.lock().unwrap(),
        &params.id,
        &params.account,
    )?))
}

/// Removes the reservation from the specified book.
async fn lending_release(
    State(db): State<Arc<Mutex<db::Database>>>,
    Query(params): Query<ReturnParams>,
) -> Result<Json<db::Book>> {
    Ok(Json(db::lending::release(&db.lock().unwrap(), &params.id)?))
}

/// Returns the list of expired borrowing periods.
async fn lending_overdues(
    State(db): State<Arc<Mutex<db::Database>>>,
) -> Result<Json<Vec<(db::book::Book, db::user::User)>>> {
    Ok(Json(db::lending::overdues(&db.lock().unwrap())?))
}
