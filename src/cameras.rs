use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

pub struct GameCameras;

impl Plugin for GameCameras {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, move_camera);
    }
}

#[derive(Component)]
pub struct FreeCamera {
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FreeCamera {
            movement_speed: 10.0,
            rotation_speed: 100.0_f32.to_radians(),
        },
        Name::new("Free Camera"),
    ));
}

fn move_camera(
    mut cameras: Query<(&mut Transform, &FreeCamera)>,
    input: Res<Input<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    for (mut transform, camera) in &mut cameras {
        let movement_amount = camera.movement_speed * time.delta_seconds();
        let movement_amount_rad = camera.rotation_speed * time.delta_seconds();

        if input.pressed(KeyCode::W) {
            transform.translation.z -= movement_amount;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.z += movement_amount;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amount;
        }
        if input.pressed(KeyCode::ShiftLeft) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::Space) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::Q) {
            transform.rotate_y(movement_amount_rad);
        }
        if input.pressed(KeyCode::E) {
            transform.rotate_y(-movement_amount_rad);
        }

        for motion in mouse_motion.iter() {
            //transform.rotate_x(motion.delta.y / 400.0);
            transform.rotate_y(motion.delta.x / 400.0);
        }
    }
}