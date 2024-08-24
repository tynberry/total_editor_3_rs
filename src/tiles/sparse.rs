use super::raw::RawTiles;

/// Tiled used inside the dense tile representation.
///
/// While similar to [RawTiles], the types of the properties has slightly changed to be clearer on
/// intent.
#[derive(Debug, Clone, Copy)]
pub struct SparseTile {
    /// Position of the tile in world.
    pub position: [usize; 3],
    /// Index of the model used for this tile inside [crate::assets::Assets].
    pub model: usize,
    /// Index of the texture used for this tile inside [crate::assets::Assets].
    pub texture: usize,
    /// Yaw angle in degrees.
    pub yaw: f32,
    /// Pitch angle in degrees.
    pub pitch: f32,
}

/// Sparse collection of tiles.
///
/// This collection is an unrolled version of the RawTiles which only saves non-empty tiles.
///
/// This is useful for quick iterations over all tiles, like rendering, however it may be slower
/// for random access as the tile must be found.
#[derive(Debug, Clone)]
pub struct SparseTiles {
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
    pub tiles: Vec<SparseTile>,
}

impl SparseTiles {
    pub fn from_raw(raw_tiles: &RawTiles) -> Self {
        //helper function
        let coords_from_index = move |index| {
            [
                index % raw_tiles.width,
                index / (raw_tiles.width * raw_tiles.length),
                (index / raw_tiles.width) % raw_tiles.length,
            ]
        };
        //unroll tiles from raw_tiles
        let mut tiles = vec![];
        //unroll tiles from raw tiles
        let mut cursor = 0;
        for tile in &raw_tiles.tiles {
            //is it unroll tile
            if tile.model < 0 {
                //unroll it
                cursor += -tile.model as i64;
            } else {
                //convert it
                tiles.push(SparseTile {
                    position: coords_from_index(cursor as usize),
                    model: tile.model as usize,
                    texture: tile.texture as usize,
                    yaw: tile.angle as f32,
                    pitch: tile.pitch as f32,
                });
                //move cursor
                cursor += 1;
            }
        }
        //compose struct
        Self {
            width: raw_tiles.width,
            height: raw_tiles.height,
            length: raw_tiles.length,
            tiles,
        }
    }
}

// Getters
impl SparseTiles {
    //Retrieves tile at coordinates.
    #[inline]
    pub fn get_tile(&self, x: usize, y: usize, z: usize) -> Option<SparseTile> {
        //find the sparse tile
        self.tiles
            .iter()
            .find(|tile| tile.position == [x, y, z])
            .copied()
    }
}
