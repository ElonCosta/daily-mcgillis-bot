use std::num::NonZero;

use atrium_api::{
    app::bsky::{
        embed::{
            defs::AspectRatioData,
            images::{ImageData, MainData},
        },
        feed::post::{RecordData, RecordEmbedRefs},
    },
    types::{Union, string::Datetime},
};
use bsky_sdk::{
    BskyAgent,
    agent::config::{Config, FileStore},
};
use chrono::Utc;
use image::{GenericImageView, ImageReader};
use rand::seq::IndexedRandom;
use text_io::read;
use tokio::time::{Duration, Instant, sleep_until};
use tokio::{fs::File, io::AsyncReadExt};
use tokio_cron_scheduler::{Job, JobBuilder, JobScheduler};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ImageDescriptor {
    name: String,
    alt_desc: String,
}

const AUTH_FILE: &str = "agent_auth.json";

type DynErrorResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> DynErrorResult<()> {
    let _ = create_agent().await?;

    let scheduler = JobScheduler::new().await?;

    scheduler.add(create_post_job()?).await?;

    scheduler.start().await?;

    loop {
        sleep_until(Instant::now() + Duration::from_secs(60)).await;
    }
}

async fn create_agent() -> DynErrorResult<BskyAgent> {
    if let Ok(true) = std::fs::exists(AUTH_FILE) {
        let config = Config::load(&FileStore::new(AUTH_FILE)).await?;

        let agent = BskyAgent::builder().config(config).build().await?;

        return Ok(agent);
    }

    print!("Login: ");
    let login: String = read!();
    print!("Password: ");
    let password: String = read!();

    let agent = BskyAgent::builder().build().await?;
    agent.login(login, password).await?;
    agent
        .to_config()
        .await
        .save(&FileStore::new(AUTH_FILE))
        .await?;

    Ok(agent)
}

fn create_post_job() -> DynErrorResult<Job> {
    let job = JobBuilder::new()
        .with_timezone(chrono_tz::Etc::UTC)
        .with_cron_job_type()
        .with_schedule("0 30 19 * * *")?
        .with_run_async(Box::new(|_uuid, mut _l| {
            Box::pin(async move {
                if let Err(error) = periodic_post().await {
                    println!("{error:#?}");
                };
            })
        }))
        .build()?;

    Ok(job)
}

async fn periodic_post() -> DynErrorResult<()> {
    let agent = create_agent().await?;

    let now = Utc::now().fixed_offset();
    //    let text = format!("Testing random image {}", &now.format("%Y-%m-%d %H:%M:%S"));

    println!("New post at: {:#?}", &now);
    let descriptors = load_image_descriptors()?;
    let ImageDescriptor { name, alt_desc } = descriptors.choose(&mut rand::rng()).unwrap();

    let mut images = Vec::new();
    if let Ok(mut file) = File::open(format!("images/{}", &name)).await {
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await.expect("read image file");

        let img = ImageReader::open(format!("images/{}", &name))?.decode()?;
        let (width, height) = img.dimensions();

        let output = agent.api.com.atproto.repo.upload_blob(buf).await?;
        images.push(
            ImageData {
                alt: alt_desc.into(),
                aspect_ratio: Some(
                    AspectRatioData {
                        height: NonZero::new(height).unwrap().into(),
                        width: NonZero::new(width).unwrap().into(),
                    }
                    .into(),
                ),
                image: output.data.blob,
            }
            .into(),
        );
    }

    let embed = Some(Union::Refs(RecordEmbedRefs::AppBskyEmbedImagesMain(
        Box::new(MainData { images }.into()),
    )));

    let _ = agent
        .create_record(RecordData {
            created_at: Datetime::new(now),
            embed,
            entities: None,
            facets: None,
            labels: None,
            langs: None,
            reply: None,
            tags: None,
            text: "".into(),
        })
        .await?;

    Ok(())
}

fn load_image_descriptors() -> DynErrorResult<Vec<ImageDescriptor>> {
    let file_content =
        std::fs::read_to_string("image_descriptors.json").expect("Missing image descriptors");

    let descriptors: Vec<ImageDescriptor> = serde_json::from_str(&file_content)?;

    Ok(descriptors)
}
