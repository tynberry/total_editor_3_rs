use super::*;

const TEST_FILE: &str = include_str!("../test/testing_map.te3");

#[test]
#[cfg(feature = "serde")]
fn test_assets_serde() {
    //load thingy

    let (_entites, _tiles, assets) = serde::from_str(TEST_FILE).unwrap();
    //check correct assets
    assert_eq!(
        assets.get_model(0).unwrap().to_str().unwrap(),
        "assets/models/shapes/cube.obj"
    );
    assert_eq!(
        assets.get_model(1).unwrap().to_str().unwrap(),
        "assets/models/shapes/cylinder.obj"
    );
    assert_eq!(
        assets.get_texture(0).unwrap().to_str().unwrap(),
        "assets/textures/tiles/brickwall.png"
    );
    assert_eq!(
        assets.get_texture(1).unwrap().to_str().unwrap(),
        "assets/textures/tiles/bluecarpet.png"
    );
}

#[test]
#[cfg(feature = "nanoserde")]
fn test_assets_nanoserde() {
    //load thingy

    let (_entites, _tiles, assets) = nanoserde::from_str(TEST_FILE).unwrap();
    //check correct assets
    assert_eq!(
        assets.get_model(0).unwrap().to_str().unwrap(),
        "assets/models/shapes/cube.obj"
    );
    assert_eq!(
        assets.get_model(1).unwrap().to_str().unwrap(),
        "assets/models/shapes/cylinder.obj"
    );
    assert_eq!(
        assets.get_texture(0).unwrap().to_str().unwrap(),
        "assets/textures/tiles/brickwall.png"
    );
    assert_eq!(
        assets.get_texture(1).unwrap().to_str().unwrap(),
        "assets/textures/tiles/bluecarpet.png"
    );
}

#[test]
#[cfg(feature = "serde")]
fn test_entities_serde() {
    //load thingy

    let (entites, _tiles, _assets) = serde::from_str(TEST_FILE).unwrap();
    //check correct entities
    assert_eq!(entites.len(), 3);
    //entity 0
    assert_eq!(entites[0].position, [0.5, 1.5, 0.5]);
    assert_eq!(entites[0].color, [125, 255, 102]);
    assert_eq!(entites[0].angles, [0.0, 0.0, 0.0]);
    assert_eq!(entites[0].radius, 1.0);
    assert_eq!(entites[0].properties["name"], "test");
    //entity 1
    assert_eq!(entites[1].position, [1.5, 3.5, 1.5]);
    assert_eq!(entites[1].color, [125, 255, 102]);
    assert_eq!(entites[1].angles, [315.0, 315.0, 0.0]);
    assert_eq!(entites[1].radius, 1.0);
    assert_eq!(entites[1].properties["name"], "test");
    //entity 2
    assert_eq!(entites[2].position, [2.5, 3.5, 4.5]);
    assert_eq!(entites[2].color, [0, 72, 255]);
    assert_eq!(entites[2].angles, [0.0, 0.0, 0.0]);
    assert_eq!(entites[2].radius, 1.0);
    assert_eq!(entites[2].properties["name"], "entity");
    assert_eq!(entites[2].properties["test"], "hodnota");
}

#[test]
#[cfg(feature = "nanoserde")]
fn test_entities_nanoserde() {
    //load thingy

    let (entites, _tiles, _assets) = nanoserde::from_str(TEST_FILE).unwrap();
    //check correct entities
    assert_eq!(entites.len(), 3);
    //entity 0
    assert_eq!(entites[0].position, [0.5, 1.5, 0.5]);
    assert_eq!(entites[0].color, [125, 255, 102]);
    assert_eq!(entites[0].angles, [0.0, 0.0, 0.0]);
    assert_eq!(entites[0].radius, 1.0);
    assert_eq!(entites[0].properties["name"], "test");
    //entity 1
    assert_eq!(entites[1].position, [1.5, 3.5, 1.5]);
    assert_eq!(entites[1].color, [125, 255, 102]);
    assert_eq!(entites[1].angles, [315.0, 315.0, 0.0]);
    assert_eq!(entites[1].radius, 1.0);
    assert_eq!(entites[1].properties["name"], "test");
    //entity 2
    assert_eq!(entites[2].position, [2.5, 3.5, 4.5]);
    assert_eq!(entites[2].color, [0, 72, 255]);
    assert_eq!(entites[2].angles, [0.0, 0.0, 0.0]);
    assert_eq!(entites[2].radius, 1.0);
    assert_eq!(entites[2].properties["name"], "entity");
    assert_eq!(entites[2].properties["test"], "hodnota");
}
