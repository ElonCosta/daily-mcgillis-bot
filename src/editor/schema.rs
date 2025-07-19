use axum_typed_multipart::{FieldData, TryFromMultipart};
use tempfile::NamedTempFile;

#[derive(Debug, TryFromMultipart)]
pub struct NewImage {
    pub alt_desc: String,
    #[form_data(limit = "unlimited")]
    pub image: FieldData<NamedTempFile>,
}

#[derive(Debug, TryFromMultipart)]
pub struct EditImage {
    pub new_alt_desc: String,
}
