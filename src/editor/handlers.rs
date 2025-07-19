use std::{fs::File, io::Write, path::Path};

use axum::response::IntoResponse;
use axum_typed_multipart::TypedMultipart;
use http::StatusCode;
use image::{GenericImageView, ImageReader};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    database::{models::ImageDescriptor, DatabaseConnection},
    editor::schema::EditImage,
    error::{AppError, AppJson}, IMAGES_DIR,
};

use super::schema::NewImage;

#[derive(Debug, Deserialize, Serialize)]
pub struct ImagePath {
    descriptor_id: i64,
}

pub async fn new_image(
    DatabaseConnection(mut conn): DatabaseConnection,
    TypedMultipart(NewImage { image, alt_desc }): TypedMultipart<NewImage>,
) -> Result<impl IntoResponse, AppError> {
    println!("Image Name: {:#?}\n alt: {:#?}", image.metadata, alt_desc);
    let uuid = Uuid::new_v4();

    let new_name = format!("{uuid:#?}.jpg");
    let new_path = Path::new(IMAGES_DIR).join(&new_name);
    let original_path = image.contents.path();

    let img = ImageReader::open(original_path)?.with_guessed_format()?.decode()?;

    let (width, height) = img.dimensions();

    let rgb_img = img.into_rgb8();

    let query = sqlx::query_as!(
        ImageDescriptor,
        "INSERT INTO image_descriptors(file_name, alt_text, width, height) VALUES (?, ?, ?, ?) RETURNING *",
        new_name,
        alt_desc,
        width,
        height
    )
    .fetch_one(&mut *conn)
    .await?;

    let jpeg_data = turbojpeg::compress_image(&rgb_img, 80, turbojpeg::Subsamp::None)?;

    File::create(new_path)?.write_all(&jpeg_data)?;

    Ok((StatusCode::CREATED, AppJson(query)))
}

pub async fn fetch_all_images(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<impl IntoResponse, AppError> {
    let query = sqlx::query_as!(ImageDescriptor, "SELECT * FROM image_descriptors")
        .fetch_all(&mut *conn)
        .await?;

    Ok((StatusCode::OK, AppJson(query)))
}

pub async fn fetch_image(
    axum::extract::Path(ImagePath { descriptor_id }): axum::extract::Path<ImagePath>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<impl IntoResponse, AppError> {
    let query = sqlx::query_as!(
        ImageDescriptor,
        "SELECT * FROM image_descriptors WHERE descriptor_id = ?",
        descriptor_id
    )
    .fetch_one(&mut *conn)
    .await?;

    Ok((StatusCode::OK, AppJson(query)))
}

pub async fn update_image(
    axum::extract::Path(ImagePath { descriptor_id }): axum::extract::Path<ImagePath>,
    DatabaseConnection(mut conn): DatabaseConnection,
    TypedMultipart(EditImage { new_alt_desc }): TypedMultipart<EditImage>,
) -> Result<impl IntoResponse, AppError> {
    let query = sqlx::query_as!(
        ImageDescriptor,
        "UPDATE image_descriptors SET alt_text = ? WHERE descriptor_id = ? RETURNING *",
        new_alt_desc,
        descriptor_id,
    )
    .fetch_one(&mut *conn)
    .await?;

    Ok((StatusCode::OK, AppJson(query)))
}
