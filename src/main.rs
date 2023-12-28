mod meshes;
mod camera;
mod lights;

use bevy::prelude::*;
use bevy_flycam::prelude::*;

use camera::CameraMovementPlugin;
use meshes::MeshesPlugin;
use lights::LightsPlughin;

fn main(){
    App::new()
    .add_plugins(DefaultPlugins)

    .add_plugins(LightsPlughin)
    .add_plugins(MeshesPlugin)
    .add_plugins(CameraMovementPlugin)
    .add_plugins(NoCameraPlayerPlugin)
    .run()
}