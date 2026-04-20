use bevy::prelude::*;

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
