use bevy::{
    color::Color,
    ecs::component::Component,
    math::primitives::Rectangle,
    // hierarchy::Parent,
    // mesh::Mesh,
    prelude::*,
};

#[derive(Component, Debug)]
#[relationship(relationship_target = Visuals)]
pub struct VisualOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = VisualOf)]
pub struct Visuals(Vec<Entity>);

#[derive(Component, Debug)]
#[require(Transform, Visibility)]
pub struct Visual {
    pub material: ColorMaterial, // TODO change to Handle<ColorMaterial> or Handle<Material>
    pub mesh: Rectangle,         // TODO change to Handle<Mesh>
}

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct BoardData;

// Can later to attach gameplay things to a tile, e.g. tile_data, occupying_piece, fog_of_war, etc.
#[derive(Component)]
#[require(Transform, Visibility)]
pub struct TileData {
    pub color: Color,
}
