use std::num::NonZeroU64;

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
use chrono::Utc;
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    DynErrorResult,
    bot::create_agent,
    database::{get_db_connection, models::ImageDescriptor},
};

pub async fn bot_post() -> DynErrorResult<()> {
    let agent = create_agent().await?;

    let now = Utc::now().fixed_offset();

    println!("New post at: {:#?}", &now);

    let mut conn = get_db_connection().await?;

    let descriptors = ImageDescriptor::get_random(&mut conn).await?;

    let ImageDescriptor {
        descriptor_id,
        file_name,
        alt_text,
        width,
        height,
        weight: _,
    } = descriptors;

    let mut images = Vec::new();
    if let Ok(mut file) = File::open(format!("images/{}", &file_name)).await {
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).await.expect("read image file");

        let output = agent.api.com.atproto.repo.upload_blob(buf).await?;
        images.push(
            ImageData {
                alt: alt_text,
                aspect_ratio: Some(
                    AspectRatioData {
                        height: NonZeroU64::new(height as u64).unwrap(),
                        width: NonZeroU64::new(width as u64).unwrap(),
                    }
                    .into(),
                ),
                image: output.data.blob,
            }
            .into(),
        );
    } else {
        panic!("No file found for {file_name:#?}");
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

    ImageDescriptor::update_weights(&mut conn, descriptor_id).await?;

    Ok(())
}
