use std::{
    io::{self, Read},
    path::Path,
};

use base64::DecodeError;
use nanoserde::DeJson;
use thiserror::Error;

use crate::{assets::Assets, entities::Entity, interim::Interim, tiles::raw::RawTiles};

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("could not read file")]
    IoError(#[from] io::Error),
    #[error("could not deserialize file")]
    DeError(#[from] nanoserde::DeJsonErr),
    #[error("could not decode tiles (invalid base64)")]
    TileError(#[from] DecodeError),
}

pub(crate) fn interim_from_path(path: impl AsRef<Path>) -> Result<Interim, LoadError> {
    //load file
    let mut file = std::fs::File::open(path)?;
    //load file into string buffer
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    //deserialize
    let interim = Interim::deserialize_json(&buffer)?;
    //return interim
    Ok(interim)
}

pub(crate) fn interim_from_str(str: &str) -> Result<Interim, LoadError> {
    //deserialize
    let interim = Interim::deserialize_json(str)?;
    //return interim
    Ok(interim)
}

/// Gives you all information about the level from file at `path`.
pub fn from_path(path: impl AsRef<Path>) -> Result<(Vec<Entity>, RawTiles, Assets), LoadError> {
    //get interim
    let interim = interim_from_path(path)?;
    //convert interim
    let mut entities = interim.ents;
    entities.iter_mut().for_each(|ent| {
        ent.position[0] = ent.position[0] * 0.5;
        ent.position[1] = ent.position[1] * 0.5;
        ent.position[2] = ent.position[2] * 0.5;
    });
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
    let mut entities = interim.ents;
    entities.iter_mut().for_each(|ent| {
        ent.position[0] = ent.position[0] * 0.5;
        ent.position[1] = ent.position[1] * 0.5;
        ent.position[2] = ent.position[2] * 0.5;
    });
    let raw_tiles = RawTiles::from_interim(&interim.tiles)?;
    let assets = Assets::from_interim(&interim.tiles);
    //compose them into a tuple
    Ok((entities, raw_tiles, assets))
}
