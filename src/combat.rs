use crate::GameState;
use bevy::prelude::*;
use crate::ascii::{AsciiSheet, spawn_ascii_sprite};
use crate::fadeout::create_fadeout;

pub struct CombatPlugin;

#[derive(Component)]
pub struct Enemy;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Combat)
                    .with_system(test_exit_combat)
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Combat)
                    .with_system(spawn_enemy)
                    .with_system(combat_camera)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Combat)
                    .with_system(despawn_enemy)
            );
    }
}

fn spawn_enemy(
    mut commands: Commands,
    ascii: Res<AsciiSheet>
) {
    let enemy = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        'b' as usize,
        Color::rgb(1.0, 1.0, 1.0),
        Vec3::new(0.0, 0.0, 0.0)
    );
    commands.entity(enemy).insert(Enemy).insert(Name::new("Bat"));
}

fn despawn_enemy(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>
) {
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn combat_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>
) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}

fn test_exit_combat(
    mut commands: Commands,
    ascii: Res<AsciiSheet>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        create_fadeout(&mut commands, GameState::Overworld, &ascii);
    }
}
