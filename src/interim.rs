#[cfg(feature = "serde")]
use serde::Deserialize;

#[cfg(feature = "nanoserde")]
use nanoserde::DeJson;

use crate::entities::Entity;

/// Intermediate format to be deserialized from which the concrete structure is then made.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "nanoserde", derive(DeJson))]
pub struct Interim {
    pub ents: Vec<Entity>,
    pub tiles: TileInterim,
}

/// Intermediate format for the tiles.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "nanoserde", derive(DeJson))]
pub struct TileInterim {
    pub width: u32,
    pub height: u32,
    pub length: u32,
    pub textures: Vec<String>,
    pub models: Vec<String>,
    pub data: String,
}
