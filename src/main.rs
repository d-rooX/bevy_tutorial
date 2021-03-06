use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const PLAYER_SPEED: f32 = 5.0;
pub const TILE_SIZE: f32 = 0.1;

mod ascii;
mod debug;
mod player;
mod tilemap;
mod combat;
mod fadeout;

use ascii::AsciiPlugin;
use debug::DebugPlugin;
use player::PlayerPlugin;
use tilemap::TileMapPlugin;
use combat::CombatPlugin;
use fadeout::FadeoutPlugin;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Overworld,
    Combat,
}

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
        .add_state(GameState::Overworld)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(AsciiPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(FadeoutPlugin)
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
