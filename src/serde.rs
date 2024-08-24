use std::{io, path::Path};

use base64::DecodeError;
use thiserror::Error;

use crate::{assets::Assets, entities::Entity, interim::Interim, tiles::raw::RawTiles};

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("could not read file")]
    IoError(#[from] io::Error),
    #[error("could not deserialize file")]
    DeError(#[from] serde_json::Error),
    #[error("could not decode tiles (invalid base64)")]
    TileError(#[from] DecodeError)
}

pub(crate) fn interim_from_path(path: impl AsRef<Path>) -> Result<Interim, LoadError> {
    //load file 
    let file = std::fs::File::open(path)?;
    //deserialize 
    let interim = serde_json::de::from_reader(file)?;
    //return interim
    Ok(interim)
}

pub(crate) fn interim_from_str(str: &str) -> Result<Interim, LoadError> {
    //deserialize 
    let interim = serde_json::de::from_str(str)?;
    //return interim
    Ok(interim)
}

pub(crate) fn interim_from_slice(slice: &[u8]) -> Result<Interim, LoadError> {
    //deserialize 
    let interim = serde_json::de::from_slice(slice)?;
    //return interim
    Ok(interim)
}

/// Gives you all information about the level from file at `path`.
pub fn from_path(path: impl AsRef<Path>) -> Result<(Vec<Entity>, RawTiles, Assets), LoadError> {
    //get interim 
    let interim = interim_from_path(path)?;
    //convert interim 
    let entities = interim.ents;
    let raw_tiles = RawTiles::from_interim(&interim.tiles)?;
    let assets = Assets::from_interim(&interim.tiles);
    //compose them into a tuple 
    Ok((entities, raw_tiles, assets))
}

/// Gives you all information about the level from the &str provided.
pub fn from_str(str: &str) -> Result<(Vec<Entity>, RawTiles, Assets), LoadError> {
    //get interim 
    let interim = interim_from_str(str)?;
    //convert interim 
    let entities = interim.ents;
    let raw_tiles = RawTiles::from_interim(&interim.tiles)?;
    let assets = Assets::from_interim(&interim.tiles);
    //compose them into a tuple 
    Ok((entities, raw_tiles, assets))
}

/// Gives you all information about the level from the slice provided.
pub fn from_slice(slice: &[u8]) -> Result<(Vec<Entity>, RawTiles, Assets), LoadError> {
    //get interim 
    let interim = interim_from_slice(slice)?;
    //convert interim 
    let entities = interim.ents;
    let raw_tiles = RawTiles::from_interim(&interim.tiles)?;
    let assets = Assets::from_interim(&interim.tiles);
    //compose them into a tuple 
    Ok((entities, raw_tiles, assets))
}
