use std::{path::PathBuf, str::FromStr};

use crate::interim::TileInterim;

/// Collection of all textures and model paths used by the editor.
///
/// These paths are relative to the editor's executable location.
#[derive(Debug, Clone)]
pub struct Assets {
    textures: Vec<PathBuf>,
    models: Vec<PathBuf>,
}

//Constructors
impl Assets {
    ///Creates [Assets] from internal intermediate representation.
    ///
    ///This method fails if one of the paths is not a valid path.
    pub(crate) fn from_interim(interim: &TileInterim) -> Self {
        //turn into paths
        let textures = interim
            .textures
            .iter()
            .map(|path| PathBuf::from_str(path))
            .collect::<Result<Vec<PathBuf>, _>>()
            .unwrap();

        let models = interim
            .models
            .iter()
            .map(|path| PathBuf::from_str(path))
            .collect::<Result<Vec<PathBuf>, _>>()
            .unwrap();

        //compose the struct
        Self { textures, models }
    }
}

//Getters
impl Assets {
    /// Gets all textures used by the editor.
    pub fn get_textures(&self) -> &[PathBuf] {
        &self.textures[..]
    }

    /// Gets all models used by the editor.
    #[inline]
    pub fn get_models(&self) -> &[PathBuf] {
        &self.models[..]
    }

    /// Gets texture path indexed by this index.
    #[inline]
    pub fn get_texture(&self, index: usize) -> Option<&PathBuf> {
        self.textures.get(index)
    }

    /// Gets model path indexed by this index.
    #[inline]
    pub fn get_model(&self, index: usize) -> Option<&PathBuf> {
        self.models.get(index)
    }
}
