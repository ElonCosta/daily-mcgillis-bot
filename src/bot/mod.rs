use std::env;

use bsky_sdk::{
    BskyAgent,
    agent::config::{Config, FileStore},
};
use tokio_cron_scheduler::{Job, JobBuilder, JobScheduler};

use crate::{AUTH_FILE, DynErrorResult, bot::jobs::bot_post};

mod jobs;

impl ScheduleAll for JobScheduler {
    async fn add_bot_jobs(&self) -> DynErrorResult<()> {
        self.add(create_post_job()?).await?;

        Ok(())
    }
}

pub trait ScheduleAll {
    async fn add_bot_jobs(&self) -> DynErrorResult<()>;
}

fn create_post_job() -> DynErrorResult<Job> {
    let schedule = &env::var("CRON_SCHEDULE")?;
    let job = JobBuilder::new()
        .with_timezone(chrono_tz::Etc::UTC)
        .with_cron_job_type()
        .with_schedule(schedule)?
        .with_run_async(Box::new(|_uuid, mut _l| {
            Box::pin(async move {
                if let Err(error) = bot_post().await {
                    println!("{error:#?}");
                };
            })
        }))
        .build()?;

    Ok(job)
}

async fn create_agent() -> DynErrorResult<BskyAgent> {
    if let Ok(true) = std::fs::exists(AUTH_FILE) {
        let config = Config::load(&FileStore::new(AUTH_FILE)).await?;

        let agent_response = BskyAgent::builder().config(config).build().await;

        if let Ok(agent) = agent_response {
            return Ok(agent);
        }
    }

    let login: String = env::var("BSKY_LOGIN")?;
    let password: String = env::var("BSKY_PASS")?;

    let agent = BskyAgent::builder().build().await?;
    agent.login(login, password).await?;
    agent
        .to_config()
        .await
        .save(&FileStore::new(AUTH_FILE))
        .await?;

    Ok(agent)
}
