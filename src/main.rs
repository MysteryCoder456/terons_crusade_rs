use bevy::prelude::*;

use components::Block;
use player::PlayerPlugin;

mod components;
mod player;

const JUNGLE_FLOOR_SHEET: &str = "overworld/jungle_floor.png";

const TIME_STEP: f32 = 1.0 / 60.0;
const SPRITE_SCALE: f32 = 2.5;
const BLOCK_SIZE: f32 = 16.;

pub struct TileSets {
    pub jungle_floor: Handle<TextureAtlas>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("87CEEB").unwrap()))
        .insert_resource(WindowDescriptor {
            title: "Teron's Crusade".to_owned(),
            width: 1080.,
            height: 720.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_world_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    // Add camera bundles
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Jungle floor texture atlas
    let jungle_floor_texture = asset_server.load(JUNGLE_FLOOR_SHEET);
    let jungle_floor_atlas = TextureAtlas::from_grid(
        jungle_floor_texture,
        Vec2::new(BLOCK_SIZE, BLOCK_SIZE),
        5,
        5,
    );
    let jungle_floor = texture_atlases.add(jungle_floor_atlas);

    let tile_sets = TileSets { jungle_floor };
    commands.insert_resource(tile_sets);
}

fn spawn_world_system(mut commands: Commands, tile_sets: Res<TileSets>) {
    // TODO: Add world loading and saving

    let mut spawn_tile = |index: usize, translation: Vec3| {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: tile_sets.jungle_floor.clone(),
                transform: Transform {
                    translation,
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, SPRITE_SCALE),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite {
                    index,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Block);
    };

    spawn_tile(0, Vec3::new(-BLOCK_SIZE * SPRITE_SCALE, 0., 0.));
    spawn_tile(2, Vec3::new(0., 0., 0.));
    spawn_tile(4, Vec3::new(BLOCK_SIZE * SPRITE_SCALE, 0., 0.));
    spawn_tile(
        20,
        Vec3::new(-BLOCK_SIZE * SPRITE_SCALE, -BLOCK_SIZE * SPRITE_SCALE, 0.),
    );
    spawn_tile(22, Vec3::new(0., -BLOCK_SIZE * SPRITE_SCALE, 0.));
    spawn_tile(
        24,
        Vec3::new(16. * SPRITE_SCALE, -BLOCK_SIZE * SPRITE_SCALE, 0.),
    );
}
