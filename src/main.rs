use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy::window::WindowMode;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod cameras;
mod cube;
mod ground;
mod light;
mod sphere;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "moonwalker".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, cursor_grab_system)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins((
            cameras::GameCameras,
            ground::Ground,
            sphere::Sphere,
            cube::Cube,
            light::Light,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .run();
}

fn cursor_grab_system(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}
