use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

pub struct GameCameras;

impl Plugin for GameCameras {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (move_camera, camera_look));
    }
}

#[derive(Component)]
pub struct FreeCamera {
    pub movement_speed: f32,
    pub rotation_speed: f32,
    pub yaw: f32,
    pub pitch: f32,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-50.0, 20.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FreeCamera {
            movement_speed: 10.0,
            rotation_speed: 100.0_f32.to_radians(),
            yaw: 0.0,
            pitch: 0.0,
        },
        Name::new("Free Camera"),
    ));
}

fn move_camera(
    inputs: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut q: Query<&mut Transform, With<FreeCamera>>,
) {
    for mut transform in q.iter_mut() {
        let mut v = Vec3::ZERO;

        let forward = transform.forward();
        let right = transform.right();

        for input in inputs.get_pressed() {
            match input {
                KeyCode::W => v += forward,
                KeyCode::S => v -= forward,
                KeyCode::A => v -= right,
                KeyCode::D => v += right,
                KeyCode::Space => v += Vec3::Y,
                KeyCode::ShiftLeft => v -= Vec3::Y,
                _ => (),
            }
        }

        v = v.normalize_or_zero();

        transform.translation += v * time.delta_seconds() * 10.0;
    }
}

fn camera_look(
    mut motion: EventReader<MouseMotion>,
    mut q: Query<(&mut Transform, &mut FreeCamera)>,
) {
    let sensitivity = 0.001;
    for (mut transform, mut freecam) in q.iter_mut() {
        for event in motion.iter() {
            freecam.yaw -= sensitivity * event.delta.x;
            freecam.pitch -= sensitivity * event.delta.y;
            freecam.pitch = freecam
                .pitch
                .clamp(-std::f32::consts::PI / 2.0, std::f32::consts::PI / 2.0);
            transform.rotation = Quat::from_axis_angle(Vec3::Y, freecam.yaw)
                * Quat::from_axis_angle(Vec3::X, freecam.pitch);
        }
    }
}
