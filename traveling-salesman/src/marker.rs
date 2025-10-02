use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Marker {
    pub(crate) coordinates: (f32, f32),
    pub(crate) label: String,
}