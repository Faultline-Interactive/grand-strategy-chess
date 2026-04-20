use super::components::*;
use bevy::prelude::*;

// TODO Figure out nice pattern to split generation and spawning of things.
// TODO Create pattern for two-entity spawning that can be reused.
// TODO Remove consts by making a settings entity that can be queried.

const BOARD_SIZE: u32 = 8;
const TILE_SIZE: f32 = 80.0;

pub fn debug_board_parent_animation(boards_query: Query<&mut Transform, With<TileData>>) {
    for mut transform in boards_query {
        transform.translate_around(
            Vec3::new(0.0, 0.0, 0.0),
            Quat::from_rotation_z(std::f32::consts::FRAC_PI_2 / 8.0),
        );
    }
}

pub fn create_board(mut commands: Commands) {
    debug!(target: "gsc", "Entered create_board");
    // Generate
    let tiles_data = generate_tiles_data(BOARD_SIZE);
    debug_once!(target: "gsc", tile_data = tiles_data.iter().count().to_string(), "Created tiles:");

    // Spawn
    let board_data_id = commands
        .spawn((BoardData, Transform::default(), Visibility::Visible))
        .id();
    debug_once!(target: "gsc", tile_data = tiles_data.iter().count().to_string(), "Created tiles:");

    let mut tile_data_ids = Vec::new();
    for t in tiles_data {
        tile_data_ids.push(commands.spawn(t).id());
    }

    for child_id in tile_data_ids {
        commands.entity(board_data_id).add_child(child_id);
    }
}

pub fn create_tile_visuals(
    tile_query: Query<(Entity, &TileData)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    debug!(target: "gsc", tile_data = tile_query.iter().count().to_string(), "Entered create_tile_visuals:");
    // Generate visuals and links
    let mut tile_bundles = Vec::new();
    for (entity, tile_data) in tile_query {
        tile_bundles.push((generate_tile_visual(tile_data), entity))
    }

    // Spawn
    debug!(target: "gsc", tile_bundles = tile_bundles.len().to_string(), "Begin spawning TileVisual:");
    for (visual, data_entity) in tile_bundles {
        let visual_entity = commands
            .spawn((
                Mesh2d(meshes.add(visual.mesh)),
                MeshMaterial2d(materials.add(visual.material)),
                Transform::default(),
            ))
            .id();
        commands.entity(data_entity).add_child(visual_entity);
        debug_once!(target: "gsc", "Spawned TileVisual");
    }
}

fn generate_tile_visual(tile_data: &TileData) -> Visual {
    Visual {
        material: ColorMaterial::from_color(tile_data.color),
        mesh: Rectangle::new(TILE_SIZE, TILE_SIZE),
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
                            Color::srgb_u8(40, 40, 40)
                        } else {
                            Color::WHITE
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
