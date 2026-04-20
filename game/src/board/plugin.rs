use bevy::prelude::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                super::systems::create_board,
                super::systems::create_tile_visuals,
            )
                .chain(),
        )
        .add_systems(FixedUpdate, super::systems::debug_board_parent_animation);
    }
}
