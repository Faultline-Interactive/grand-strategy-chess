use super::components::{Visual, VisualOf};
use crate::board::components::{TileColor, TileData};
use bevy::prelude::*;

const TILE_SIZE: f32 = 80.0;

pub fn create_tile_visuals(
    tile_query: Query<(Entity, &TileData), Added<TileData>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    debug!(
        target: "gsc",
        tile_data = tile_query.iter().count().to_string(),
        "Entered visuals::create_tile_visuals:"
    );

    for (tile_entity, tile_data) in tile_query {
        let visual = Visual {
            material: ColorMaterial::from_color({
                match tile_data.color {
                    TileColor::Light => Color::WHITE,
                    TileColor::Dark => Color::srgb_u8(80, 40, 40),
                }
            }),
            mesh: Rectangle::new(TILE_SIZE, TILE_SIZE),
        };

        let mesh = visual.mesh;
        let material = visual.material.clone();

        let visual_entity = commands
            .spawn((
                visual,
                VisualOf(tile_entity),
                Mesh2d(meshes.add(mesh)),
                MeshMaterial2d(materials.add(material)),
                Transform::default(),
            ))
            .id();

        commands.entity(tile_entity).add_child(visual_entity);
        debug_once!(target: "gsc", "Spawned TileVisual from VisualsPlugin");
    }
}
