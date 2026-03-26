use bevy::{
    color::Color,
    ecs::component::Component,
    math::{Vec2, primitives::Rectangle},
    // hierarchy::Parent,
    // mesh::Mesh,
};

#[derive(Component)]
pub struct Tile {
    pub location: Vec2,
    pub material: Color,
    pub mesh: Rectangle,
    // pub parent: Parent,
}

// Currently Board doesn't do anything. Ideally it holds tiles as its children so we can translate and despawn everything together.
#[derive(Component)]
// #[relationship_target(relationship_target = Children, linked_spawn)]
// pub struct Children(Vec<Entity>);
pub struct Board {}
