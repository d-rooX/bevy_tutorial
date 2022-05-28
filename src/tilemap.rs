use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use bevy::prelude::*;
use crate::ascii::{AsciiSheet, spawn_ascii_sprite};
use crate::TILE_SIZE;

#[derive(Component)]
pub struct EncounterSpawner;

#[derive(Component)]
pub struct TileCollider;

pub struct TileMapPlugin;
impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_simple_map);
    }
}

fn create_simple_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let file = File::open("./assets/map.txt").expect("No map.txt file found");
    let mut tiles = Vec::new();

    for (y, line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = line {
            for (x, char) in line.chars().enumerate() {
                let tile = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    char as usize,
                    Color::rgb(0.9, 0.9, 0.9),
                    Vec3::new(
                        x as f32 * TILE_SIZE,
                        -(y as f32) * TILE_SIZE,
                        100.0
                    )
                );
                if char == '#' {
                    commands.entity(tile).insert(TileCollider);
                }
                else if char == '~' {
                    commands.entity(tile).insert(EncounterSpawner);
                }
                tiles.push(tile);
            }
        }
    }
    commands
        .spawn()
        .insert(Name::new("Tiles".to_string()))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&tiles);
}