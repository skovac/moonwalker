use bevy::prelude::*;
use bevy::window::CursorGrabMode;

mod cameras;
mod ground;
mod sphere;
mod cube;
mod light;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "moonwalker".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, cursor_grab_system)
        .add_plugins((
            cameras::GameCameras,
            ground::Ground,
            sphere::Sphere,
            cube::Cube,
            light::Light,
        ))
        .run();
}

fn cursor_grab_system(
    mut windows: Query<&mut Window>
) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}