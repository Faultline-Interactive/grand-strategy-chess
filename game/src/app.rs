use bevy::{log::LogPlugin, prelude::*};

pub fn run() {
    info!(target: "gsc", "Entered run");
    let mut app = App::new();
    // Bevy plugins
    // .add_plugins(DefaultPlugins)
    // External plugins

    // this code is compiled only if debug assertions are enabled (debug mode)
    #[cfg(debug_assertions)]
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        level: bevy::log::Level::TRACE,
        filter: "info,gsc=trace,wgpu_core=warn,wgpu_hal=warn,naga=warn".into(),
        ..default()
    }));

    // this code is compiled only if debug assertions are disabled (release mode)
    #[cfg(not(debug_assertions))]
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        level: bevy::log::Level::INFO,
        filter: "info,gsc=info,wgpu_core=warn,wgpu_hal=warn,naga=warn".into(),
        ..default()
    }));

    app.add_plugins(crate::board::BoardPlugin);
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(mut commands: Commands) {
    debug!(target: "gsc", "Entered setup");
    commands.spawn(Camera2d);
}
