use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ImageDescriptor {
    pub descriptor_id: i64,
    pub file_name: String,
    pub alt_text: String,
    pub width: i64,
    pub height: i64,
    pub weight: f64,
}
