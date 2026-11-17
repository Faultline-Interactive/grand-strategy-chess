use bevy::prelude::*;

pub struct VisualsPlugin;

impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(
        //     Startup,
        //     super::systems::create_tile_visuals.after(crate::board::systems::create_board),
        // );
        app.add_observer(super::systems::visual_spawn_event_listener);
    }
}
