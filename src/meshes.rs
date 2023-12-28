use bevy::prelude::*;

pub struct MeshesPlugin;

impl Plugin for MeshesPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (ground, cube, uvsphere, icosphere, cylinder, torus, capsule));
    }
}

fn ground(mut commands: Commands, assets_server: ResMut<AssetServer>){
    let ground_glb: Handle<Scene> = assets_server.load("ground.glb#Scene0");
    commands.spawn(SceneBundle{
        scene: ground_glb,
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    });
}

fn cube(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materialmeshes: ResMut<Assets<StandardMaterial>>){
    let mesh: Handle<Mesh> = meshes.add(shape::Box::new(1.0, 1.0, 1.0).into());
    // OR
    // let mesh: Handle<Mesh> = meshes.add(shape::Box{min_x: 0.0, max_x: 1.0, min_y: 0.0, max_y: 1.0, min_z: 0.0, max_z: 1.0}.into());

    commands.spawn(PbrBundle{
        mesh: mesh.clone(),
        material: materialmeshes.add(Color::rgb_u8(200, 200, 0).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
}

fn uvsphere(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materialmeshes: ResMut<Assets<StandardMaterial>>){
    let mesh: Handle<Mesh> = meshes.add(shape::UVSphere{radius: 0.5, sectors: 32, stacks: 64}.into());

    commands.spawn(PbrBundle{
        mesh: mesh.clone(),
        material: materialmeshes.add(Color::rgb_u8(0, 200, 200).into()),
        transform: Transform::from_xyz(-1.0, 0.5, 1.0),
        ..Default::default()
    });
}

fn icosphere(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materialmeshes: ResMut<Assets<StandardMaterial>>){
    let mesh: Handle<Mesh> = meshes.add(shape::Icosphere{radius: 0.5, subdivisions: 2}.try_into().unwrap());

    commands.spawn(PbrBundle{
        mesh: mesh.clone(),
        material: materialmeshes.add(Color::rgb_u8(200, 0, 0).into()),
        transform: Transform::from_xyz(1.0, 0.5, -1.0),
        ..Default::default()
    });
}

fn cylinder(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materialmeshes: ResMut<Assets<StandardMaterial>>){
    let mesh: Handle<Mesh> = meshes.add(shape::Cylinder{radius: 0.5, height: 1.0, resolution: 32, segments: 1}.into());

    commands.spawn(PbrBundle{
        mesh: mesh.clone(),
        material: materialmeshes.add(Color::rgb_u8(200, 0, 200).into()),
        transform: Transform::from_xyz(2.0, 0.5, -2.0),
        ..Default::default()
    });
}

fn torus(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materialmeshes: ResMut<Assets<StandardMaterial>>){
    let mesh: Handle<Mesh> = meshes.add(shape::Torus{radius: 0.5, ring_radius: 0.3, subdivisions_segments: 32, subdivisions_sides: 20}.into());

    commands.spawn(PbrBundle{
        mesh: mesh.clone(),
        material: materialmeshes.add(Color::rgb_u8(50, 200, 50).into()),
        transform: Transform::from_xyz(-2.0, 0.5, 2.0),
        ..Default::default()
    });
}

fn capsule(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materialmeshes: ResMut<Assets<StandardMaterial>>){
    let mesh: Handle<Mesh> = meshes.add(shape::Capsule{radius: 0.5, rings: 32, depth: 0.5, latitudes: 10, longitudes: 20, uv_profile: shape::CapsuleUvProfile::Fixed}.into());

    commands.spawn(PbrBundle{
        mesh: mesh.clone(),
        material: materialmeshes.add(Color::rgb_u8(100, 50, 255).into()),
        transform: Transform::from_xyz(-3.0, 0.8, 3.0),
        ..Default::default()
    });
}