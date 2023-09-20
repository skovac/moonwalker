use bevy::prelude::*;

pub struct Sphere;

impl Plugin for Sphere {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sphere);
            //.add_systems(Update, move_camera);
    }
}

fn spawn_sphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere { ..Default::default() })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });
}