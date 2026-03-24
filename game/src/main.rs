use bevy::{math::Vec2, prelude::*};

mod board;

/*

All code needs to be refactored to follow ECS design (entity/data, component, system)

*/

const TILE_SIZE: f32 = 80.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // TODO: Move to "board.rs" along with other todos tagged "BOARD"
    let board_size: u8 = 8;

    commands.spawn(Camera2d);

    let mut tiles: Vec<Handle<Mesh>> = Vec::new();

    for _ in 0..(board_size * board_size) {
        tiles.push(meshes.add(Rectangle::new(TILE_SIZE, TILE_SIZE)));
    }

    let mut tile_locs = tiles_locus(board_size);

    // TODO: BOARD
    for (i, tile) in tiles.into_iter().enumerate() {
        // TODO: Move into a shape object/array

        let loc = tile_locs.pop().unwrap();

        let color = {
            // if (loc.x + loc.y) % 2 == 0 {
            if (i as u8 / board_size + i as u8 % board_size) % 2 == 0 {
                Color::BLACK
            } else {
                Color::WHITE
            }
        };

        commands.spawn((
            Mesh2d(tile),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(loc.x, loc.y, 0.0),
        ));
    }
}

fn tiles_locus(board_size: u8) -> Vec<Vec2> {
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
