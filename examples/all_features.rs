use total_editor_rs::{
    serde,
    tiles::{dense::DenseTiles, sparse::SparseTiles},
};

const EXAMPLE_FILE: &str = include_str!("../test/testing_map.te3");

fn main() {
    // Loads the assets paths, entities and raw tiles.
    let (entities, raw_tiles, assets) = serde::from_str(EXAMPLE_FILE).unwrap();

    // Print all asset paths
    println!("Textures");
    for path in assets.get_textures() {
        println!("{:?}", path);
    }
    println!("Models");
    for path in assets.get_models() {
        println!("{:?}", path);
    }

    // Turns raw tiles into a more usable format.
    // Dense format for random tile accesses.
    let dense_tiles = DenseTiles::from_raw(&raw_tiles);
    // Sparse format for iterations.
    let sparse_tiles = SparseTiles::from_raw(&raw_tiles);

    // Print debug information from them.
    println!("Dense tiles");
    dbg!(dense_tiles);
    println!("Sparse tiles");
    dbg!(sparse_tiles);
}
