use bevy::prelude::*;

pub struct LightsPlughin;

impl Plugin for LightsPlughin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_pointlight);
    }
}

fn setup_pointlight(mut commands: Commands){
    let position_of_light = Transform::from_xyz(-1.8, 3.0, -1.8).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn(PointLightBundle{
        point_light: PointLight{
            intensity: 100.0,
            range: 100.0,
            ..default()
        },
        transform: position_of_light,
        ..default()
    });
}