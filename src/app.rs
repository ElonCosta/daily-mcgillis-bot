use axum::{
    Router,
    extract::{DefaultBodyLimit, MatchedPath, Request},
};
use http::{HeaderValue, Method};
use sqlx::{Pool, Sqlite};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

use crate::{IMAGES_DIR, database::new_db_pool, editor};

pub async fn create_router(pool: Pool<Sqlite>) -> Router {

    Router::new()
        .merge(editor::config_handlers())
        .nest_service("/images", ServeDir::new(IMAGES_DIR))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|req: &Request| {
                    let method = req.method();
                    let uri = req.uri();

                    let matched_path = req
                        .extensions()
                        .get::<MatchedPath>()
                        .map(|matched_path| matched_path.as_str());

                    tracing::debug_span!("request", %method, %uri, matched_path)
                })
                .on_failure(()),
        )
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([http::header::CONTENT_TYPE]),
        )
        .layer(DefaultBodyLimit::max(1024 * 1024 * 30))
        .with_state(pool)
}
