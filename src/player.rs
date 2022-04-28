use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_inspector_egui::Inspectable;

use crate::ascii::{AsciiSheet, spawn_ascii_sprite};
use crate::{TILE_SIZE, PLAYER_SPEED};
use crate::tilemap::TileCollider;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(player_movement.label("movement"))
            .add_system(camera_follow.after("movement"));
    }
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
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

    let target = transform.translation + velocity.clamp_length(0.0, speed);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target;
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    !wall_query.iter().any(
        |wall_transform| collide(
        target_player_pos,
        Vec2::splat(TILE_SIZE),
        wall_transform.translation,
        Vec2::splat(TILE_SIZE)
    ).is_some())
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(0.8, 0.8, 0.8),
        Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0)
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