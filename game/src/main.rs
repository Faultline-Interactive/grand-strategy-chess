#[cfg(not(target_arch = "wasm32"))]
use bevy::{
    input::common_conditions::input_just_pressed,
    sprite_render::{Wireframe2dConfig, Wireframe2dPlugin},
};
use bevy::{input::common_conditions::input_toggle_active, prelude::*};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
    ));
    app.run();
}

fn setup(
    mut commands: Commands,
) {
}



