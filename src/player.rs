use bevy::prelude::*;
use crate::AsciiSheet;
use crate::TILE_SIZE;

pub const PLAYER_SPEED: f32 = 5.0;

#[derive(Component)]
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
    // player sprite
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(0.7, 0.7, 0.7);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    let player = commands.spawn_bundle( SpriteSheetBundle {
        sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 900.0),
            ..Default::default()
        },
        ..Default::default()
    })
        .insert(Name::new("Player"))
        .insert(Player {
            speed: PLAYER_SPEED
        })
        .id();

    // background sprite
    let mut background_sprite = TextureAtlasSprite::new(0);
    background_sprite.color = Color::rgb(0., 0., 0.);
    background_sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

    let background = commands.spawn_bundle( SpriteSheetBundle {
        sprite: background_sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            ..Default::default()
        },
        ..Default::default()
    }).id();
    commands.entity(player).push_children(&[background]);
}