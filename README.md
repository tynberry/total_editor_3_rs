# Total Editor Rust Loader

## What is this?

This crate loads *.te3 files from [Total Editor 3](https://x54321.itch.io/total-editor-3).

## How to use?

For examples you can look into the examples/ folder of this repository.

Or you can use this snippet.

```rust
use total_editor_rs::{
    serde,
    tiles::{dense::DenseTiles, sparse::SparseTiles},
};

fn main() {
    // Loads the assets paths, entities and raw tiles.
    let (entities, raw_tiles, assets) = serde::from_path("path/to/file").unwrap();

    // Turns raw tiles into a more usable format.
    // Dense format for random tile accesses.
    let dense_tiles = DenseTiles::from_raw(&raw_tiles);
    // Sparse format for iterations.
    let sparse_tiles = SparseTiles::from_raw(&raw_tiles);
}
```

This snippet first loads entities, asset paths (textures and models) and raw
tiles. Raw tiles represent tiles as is in the file and as such is not very
useful.

Therefore it must be first converted into one of two structures.

`DenseTiles` is a structure which allows O(1) random access of tiles at the cost
of including Empty tiles in its representation. This makes iteration slower
since you need to iterate through Empty tiles as well.

`SparseTiles` is a structure which allows quick iteration by ignoring the Empty
tiles. This however slows random access of tiles as they must by found inside
the structure in O(n) time.

## Features

This crate has two features specifying what deserialization library you want to
use.

Use `serde` feature (default) if you want to use `serde` and `serde_json`.

Use `nanoserde` feature if you want to use `nanoserde`.

You can use both features and they only affect the underlying backend deserialization.
