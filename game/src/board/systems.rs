use super::components::*;
use bevy::prelude::*;

// would like to remove consts by making a settings struct or something else
const BOARD_SIZE: u32 = 8;
const TILE_SIZE: f32 = 80.0;

// This fn is the system that is exposed to Bevy
pub fn create_board(
    meshes: ResMut<Assets<Mesh>>,
    commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    // let gameboard = Board;
    let tiles = generate_tiles(BOARD_SIZE);
    spawn_tiles(meshes, commands, materials, tiles);
}

fn spawn_tiles(
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    tiles: Vec<Tile>,
    // board: Board,
) {
    for t in tiles {
        commands.spawn((
            Mesh2d(meshes.add(t.mesh)),
            MeshMaterial2d(materials.add(t.material)),
            Transform::from_xyz(t.location.x, t.location.y, 0.0),
        ));
    }
}

fn generate_tiles(board_size: u32) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = Vec::new();
    let mut tile_locs = tiles_locus(board_size);

    for i in 0..(board_size * board_size) {
        tiles.push(Tile {
            location: tile_locs.pop().unwrap(),
            material: {
                if (i / board_size + i % board_size) % 2 == 0 {
                    Color::BLACK
                } else {
                    Color::WHITE
                }
            },
            mesh: Rectangle::new(TILE_SIZE, TILE_SIZE),
        })
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
