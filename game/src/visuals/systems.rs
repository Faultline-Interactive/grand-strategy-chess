use super::components::*;
use crate::board::components::*;
use bevy::{ecs::event::Trigger, prelude::*};

const TILE_SIZE: f32 = 80.0;

pub fn visual_spawn_event_listener(
    event: On<SpawnVisualRequest>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
}

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
        let material = ColorMaterial::from_color({
            match tile_data.color {
                TileColor::Light => Color::WHITE,
                TileColor::Dark => Color::srgb_u8(80, 40, 40),
            }
        });

        let visual_entity = commands
            .spawn((
                VisualOf(tile_entity),
                Mesh2d(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE))),
                MeshMaterial2d(materials.add(material)),
                Transform::default(),
            ))
            .id();

        commands.entity(tile_entity).add_child(visual_entity);
        debug_once!(target: "gsc", "Spawned TileVisual from VisualsPlugin");
    }
}
