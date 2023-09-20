use bevy::prelude::*;

pub struct Ground;

impl Plugin for Ground {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground);
            //.add_systems(Update, move_camera);
    }
}

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(500.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}