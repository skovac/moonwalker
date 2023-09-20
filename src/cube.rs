use bevy::prelude::*;

pub struct Cube;

impl Plugin for Cube {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cube);
    }
}

fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 2.5, 0.0),
        ..default()
    });
}