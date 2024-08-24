use base64::{DecodeError, Engine};
use bytemuck::{Pod, Zeroable};

use crate::interim::TileInterim;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RawTile {
    pub model: i32,
    pub angle: i32,
    pub texture: i32,
    pub pitch: i32,
}

//SAFETY:
//You can fill it.
//All zeroes is valid configuration (empty tile).
unsafe impl Zeroable for RawTile {}
//SAFETY:
//You can fill it.
//All possible combinations are valid. Sort of.
//No padding bytes. (only i32)
//All fields are Pod.
//It is repr C.
//No pointers what so ever.
unsafe impl Pod for RawTile {}

/// Collection of raw representation of tiles inside the format.
///
/// The tiles are retrieved as is. Since the editor uses Run Length Encoding for empty
/// tiles, this struct will have very few tiles.
#[derive(Debug, Clone)]
pub struct RawTiles {
    /// Width of the grid.
    ///
    /// This is the size along X axis in editor.
    pub width: usize,
    /// Height of the grid.
    ///
    /// This is the size along Y axis in editor.
    pub height: usize,
    /// Length of the grid.
    ///
    /// This is the size along Z axis in editor.
    pub length: usize,
    /// All tiles of the grid.
    pub tiles: Vec<RawTile>,
}

// Constructors
impl RawTiles {
    /// Creates [RawTiles] from intermediate tile representation.
    pub(crate) fn from_interim(interim: &TileInterim) -> Result<Self, DecodeError> {
        //unload base64
        let bytes = base64::prelude::BASE64_STANDARD.decode(&interim.data)?;
        Ok(Self {
            width: interim.width as usize,
            height: interim.height as usize,
            length: interim.length as usize,
            tiles: bytemuck::cast_slice(&bytes[..]).to_vec(),
        })
    }
}
