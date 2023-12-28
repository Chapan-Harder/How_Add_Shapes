use bevy::prelude::*;
use bevy_flycam::{FlyCam, MovementSettings, KeyBindings};

pub struct CameraMovementPlugin;

impl Plugin for CameraMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, cameramovement).insert_resource(MovementSettings{
            sensitivity: 0.00010,
            speed: 7.0
        })
        .insert_resource(KeyBindings{
            move_ascend: KeyCode::E,
            move_descend: KeyCode::Q,
            ..Default::default()
        });
    }
}

fn cameramovement(mut commands: Commands){
    commands.spawn((Camera3dBundle{transform: Transform::from_xyz(-2.0, 3.0, -2.0)
        .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()},
    FlyCam));
}