use std::{env, fs};

use axum::http::StatusCode;

use sqlx::migrate::Migrator;
use tokio_cron_scheduler::JobScheduler;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{app::create_router, bot::ScheduleAll, database::new_db_pool};

mod app;
mod bot;
mod database;
mod editor;
mod error;

const AUTH_FILE: &str = "agent_auth.json";
const IMAGES_DIR: &str = "images";

static MIGRATOR: Migrator = sqlx::migrate!();

type DynErrorResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> DynErrorResult<()> {
    dotenvy::dotenv()?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let file_exists: bool = fs::exists(IMAGES_DIR).unwrap_or(false);

    if !file_exists {
        fs::create_dir_all(IMAGES_DIR)?;
    }

    let file_exists: bool = fs::exists("db").unwrap_or(false);

    if !file_exists {
        fs::create_dir_all("db")?;
    }

    let main_db_file = create_db_file("main");

    if let Err(e) = main_db_file {
        panic!("Could not create main db file, cause: {:?}", e);
    }

    let pool = new_db_pool().await?;

    #[cfg(not(debug_assertions))]
    MIGRATOR.run(&pool).await?;

    let scheduler = JobScheduler::new().await?;
    scheduler.add_bot_jobs().await?;
    scheduler.start().await?;

    let router = create_router(pool).await;

    let listener = tokio::net::TcpListener::bind(env::var("LISTENER_ADDR")?)
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub fn create_db_file(path: &str) -> std::io::Result<()> {
    fs::File::options()
        .create(true)
        .write(true)
        .open(format!("db/{}.db", path))?;

    Ok(())
}
