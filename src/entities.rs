use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::Deserialize;

#[cfg(feature = "nanoserde")]
use nanoserde::DeJson;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "nanoserde", derive(DeJson))]
pub struct Entity {
    pub position: [f32; 3],
    pub color: [u8; 3],
    pub angles: [f32; 3],
    pub radius: f32,
    pub display: u8,
    pub model: String,
    pub texture: String,
    pub properties: HashMap<String, String>,
}
