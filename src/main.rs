use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            height: 500.0,
            width: 600.0,
            title: "Bevy Tutorial".to_string(),
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .run();
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    // player sprite
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(0.7, 0.7, 0.7);
    sprite.custom_size = Some(Vec2::splat(1.0));

    let player = commands.spawn_bundle( SpriteSheetBundle {
        sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 900.0),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Name::new("Player")).id();

    // background sprite
    let mut background_sprite = TextureAtlasSprite::new(0);
    background_sprite.color = Color::rgb(0., 0., 0.);
    background_sprite.custom_size = Some(Vec2::splat(1.0));

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

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0;
    camera.orthographic_projection.left = -1.0;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

struct AsciiSheet(Handle<TextureAtlas>);

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let image = assets.load("Ascii.png");
    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::splat(9.0),
        16,
        16,
        Vec2::splat(2.0)
    );
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}





