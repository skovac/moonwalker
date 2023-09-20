use bevy::prelude::*;

pub struct Ground;

impl Plugin for Ground {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_ground, spawn_gltf));
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

fn spawn_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    // note that we have to include the `Scene0` label
    let my_gltf = ass.load("houses.glb#Scene0");

    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    commands.spawn(SceneBundle {
        scene: my_gltf,
        transform: Transform::from_xyz(0.0, 4.0, 0.0),
        ..Default::default()
    });
}