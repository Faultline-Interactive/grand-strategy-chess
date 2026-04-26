use bevy::prelude::*;

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct BoardData;

// Can later to attach gameplay things to a tile, e.g. tile_data, occupying_piece, fog_of_war, etc.
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct TileData {
    pub color: TileColor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileColor {
    Light,
    Dark,
}
