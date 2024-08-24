use super::raw::RawTiles;

/// Tiled used inside the dense tile representation.
///
/// While similar to [RawTiles], the types of the properties has slightly changed to be clearer on
/// intent.
#[derive(Debug, Clone, Copy)]
pub enum DenseTile {
    /// Empty tile
    Empty,
    /// Full tile
    Full {
        /// Index of the model used for this tile inside [crate::assets::Assets].
        model: usize,
        /// Index of the texture used for this tile inside [crate::assets::Assets].
        texture: usize,
        /// Yaw angle in degrees.
        yaw: f32,
        /// Pitch angle in degrees.
        pitch: f32,
    },
}

/// Dense collection of tiles.
///
/// These tiles are unrolled from the Run Length Encoding used inside the editor.
/// This means empty tiles are still going to be inside the memory.
///
/// This is great when you require random access for tiles (i.e. collisions),
/// however suffers greatly if you have a lot of empty tiles and you only want
/// to get the full ones.
#[derive(Debug, Clone)]
pub struct DenseTiles {
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
    pub tiles: Vec<DenseTile>,
}

impl DenseTiles {
    pub fn from_raw(raw_tiles: &RawTiles) -> Self {
        //unroll tiles from raw_tiles
        let mut tiles =
            vec![DenseTile::Empty; raw_tiles.width * raw_tiles.height * raw_tiles.length];
        //unroll tiles from raw tiles
        let mut cursor = 0;
        for tile in &raw_tiles.tiles {
            //is it unroll tile
            if tile.model < 0 {
                //unroll it
                cursor += -tile.model as i64;
            } else {
                //convert it
                tiles[cursor as usize] = DenseTile::Full {
                    model: tile.model as usize,
                    texture: tile.texture as usize,
                    yaw: tile.angle as f32,
                    pitch: tile.pitch as f32,
                };
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
impl DenseTiles {
    ///Computes inner index to tile at coordinates \[x,y,z\].
    #[inline]
    pub fn index_from_coords(&self, x: usize, y: usize, z: usize) -> usize {
        x + (y * self.width * self.length) + (z * self.width)
    }

    ///Computes coordinates to tile from index.
    #[inline]
    pub fn coords_from_index(&self, index: usize) -> [usize; 3] {
        [
            index % self.width,
            index / (self.width * self.length),
            (index / self.width) % self.length,
        ]
    }

    //Retrieves tile at coordinates.
    #[inline]
    pub fn get_tile(&self, x: usize, y: usize, z: usize) -> Option<DenseTile> {
        self.tiles.get(self.index_from_coords(x, y, z)).copied()
    }
}
