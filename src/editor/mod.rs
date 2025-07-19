use axum::{routing::{get, post}, Router};
use sqlx::{Pool, Sqlite};

mod handlers;
mod schema;

pub fn config_handlers() -> Router<Pool<Sqlite>> {
    Router::new()
        .route("/editor/image/new", post(handlers::new_image))
        .route("/editor/image/all", get(handlers::fetch_all_images))
        .route("/editor/image/{descriptor_id}", get(handlers::fetch_image))
        .route("/editor/image/{descriptor_id}/update", post(handlers::update_image))
}
