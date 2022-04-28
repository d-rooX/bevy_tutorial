use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

mod player;
mod debug;
mod ascii;

use player::PlayerPlugin;
use debug::DebugPlugin;
use ascii::AsciiPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            height: 500.0,
            width: 600.0,
            title: "Bevy Tutorial".to_string(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(AsciiPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0;
    camera.orthographic_projection.left = -1.0;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
