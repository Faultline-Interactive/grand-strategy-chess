use crate::visuals::components::HasVisual;

use super::components::*;
use bevy::prelude::*;

// TODO Figure out nice pattern to split generation and spawning of things.
// TODO Remove consts by making a settings entity that can be queried.

const BOARD_SIZE: u32 = 8;
const TILE_SIZE: f32 = 80.0;

pub fn debug_board_parent_animation(boards_query: Query<&mut Transform, With<TileData>>) {
    for mut transform in boards_query {
        transform.translate_around(
            Vec3::new(0.0, 0.0, 0.0),
            Quat::from_rotation_z(std::f32::consts::FRAC_PI_2 / 64.0),
        );
    }
}

pub fn create_board(mut commands: Commands) {
    debug!(target: "gsc", "Entered create_board");
    // Generate
    let tiles_data = generate_tiles_data(BOARD_SIZE);
    debug_once!(target: "gsc", tile_data = tiles_data.iter().count().to_string(), "Created tiles:");

    // Spawn
    let board_data_id = commands.spawn((BoardData, Transform::default())).id();
    debug_once!(target: "gsc", tile_data = tiles_data.iter().count().to_string(), "Created tiles:");

    let mut tile_data_ids = Vec::new();
    for t in tiles_data {
        tile_data_ids.push(commands.spawn((t, HasVisual)).id());
    }

    for child_id in tile_data_ids {
        commands.entity(board_data_id).add_child(child_id);
        // child_id.
    }
}

fn generate_tiles_data(board_size: u32) -> Vec<(TileData, Transform)> {
    let mut tiles = Vec::new();
    let mut tile_locs = tiles_locus(board_size);

    for i in 0..(board_size * board_size) {
        if let Some(loc) = tile_locs.pop() {
            let tile_data = (
                TileData {
                    color: {
                        if (i / board_size + i % board_size) % 2 == 0 {
                            TileColor::Dark
                        } else {
                            TileColor::Light
                        }
                    },
                },
                Transform::from_xyz(loc.x, loc.y, 0.0),
            );
            tiles.push(tile_data)
        }
    }
    tiles
}

fn tiles_locus(board_size: u32) -> Vec<Vec2> {
    let mut locs = Vec::new();
    for i in -((board_size / 2) as i32)..((board_size / 2) as i32) {
        for j in -((board_size / 2) as i32)..((board_size / 2) as i32) {
            locs.push(Vec2::new(
                TILE_SIZE * (i as f32 + 0.5),
                TILE_SIZE * (j as f32 + 0.5),
            ));
        }
    }
    locs
}
