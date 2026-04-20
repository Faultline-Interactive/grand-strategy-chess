use bevy::{color::Color, ecs::component::Component, prelude::*};

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct BoardData;

// Can later to attach gameplay things to a tile, e.g. tile_data, occupying_piece, fog_of_war, etc.
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct TileData {
    pub color: Color,
}
