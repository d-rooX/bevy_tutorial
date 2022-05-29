use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_inspector_egui::egui::Event::Key;
use bevy_inspector_egui::Inspectable;

use crate::ascii::{spawn_ascii_sprite, AsciiSheet};
use crate::tilemap::{EncounterSpawner, TileCollider};
use crate::{GameState, PLAYER_SPEED, TILE_SIZE};

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Combat).with_system(hide_player))
            .add_system_set(SystemSet::on_exit(GameState::Combat).with_system(show_player))
            .add_system_set(
                SystemSet::on_update(GameState::Overworld)
                    .with_system(player_movement.label("movement"))
                    .with_system(camera_follow.after("movement"))
                    .with_system(player_encounter_checking.after("movement")),
            )
            .add_startup_system(spawn_player)
            .add_system(test_exit_combat);
    }
}

fn hide_player(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let mut player_visibility = player_query.single_mut();
    player_visibility.is_visible = false;
    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = false;
            }
        }
    }
}

fn show_player(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let mut player_visibility = player_query.single_mut();
    player_visibility.is_visible = true;
    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = true;
            }
        }
    }
}

fn test_exit_combat(keyboard: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        state
            .set(GameState::Overworld)
            .expect("Failed to change state");
    }
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
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
    if !wall_query
        .iter()
        .any(|&w_tranform| wall_collision_check(target, w_tranform.translation))
    {
        transform.translation = target;
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn wall_collision_check(target_player_pos: Vec3, wall_translation: Vec3) -> bool {
    let size = Vec2::splat(TILE_SIZE);
    let collision = collide(target_player_pos, size * 0.9, wall_translation, size);
    collision.is_some()
}

fn player_encounter_checking(
    player_query: Query<&Transform, With<Player>>,
    encounter_query: Query<&Transform, (With<EncounterSpawner>, Without<Player>)>,
    mut state: ResMut<State<GameState>>,
) {
    let player_translation = player_query.single().translation;
    if encounter_query
        .iter()
        .any(|&tile_transform| wall_collision_check(player_translation, tile_transform.translation))
    {
        println!("Changing state");
        state
            .set(GameState::Combat)
            .expect("Failed to change states");
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(0.8, 0.8, 0.8),
        Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0),
    );
    let background = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        0,
        Color::rgb(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
    );

    commands.entity(background).insert(Name::new("Background"));

    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player {
            speed: PLAYER_SPEED,
        })
        .push_children(&[background]);
}
