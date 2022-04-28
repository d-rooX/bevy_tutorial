use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::ascii::{AsciiSheet, spawn_ascii_sprite};
use crate::{TILE_SIZE, PLAYER_SPEED};

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    let delta = time.delta_seconds();
    let (player, mut transform) = player_query.single_mut();
    let mut velocity = Vec3::splat(0.0);
    let speed = TILE_SIZE * player.speed * delta;

    if input.pressed(KeyCode::W) {
        velocity.y += speed;
    }
    if input.pressed(KeyCode::S) {
        velocity.y -= speed;
    }
    if input.pressed(KeyCode::A) {
        velocity.x -= speed;
    }
    if input.pressed(KeyCode::D) {
        velocity.x += speed;
    }

    transform.translation += velocity.clamp_length(0.0, speed);
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(0.8, 0.8, 0.8),
        Vec3::new(0.0, 0.0, 900.0)
    );
    let background = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        0,
        Color::rgb(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0)
    );

    commands.entity(background)
        .insert(Name::new("Background"));

    commands.entity(player)
        .insert(Name::new("Player"))
        .insert(Player { speed: PLAYER_SPEED })
        .push_children(&[background]);
}