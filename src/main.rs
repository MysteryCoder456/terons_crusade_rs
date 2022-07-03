use bevy::prelude::*;

const JUNGLE_FLOOR_SHEET: &str = "overworld/jungle_floor.png";

const SPRITE_SCALE: f32 = 2.5;

pub struct GameTextures {
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
    let jungle_floor_atlas =
        TextureAtlas::from_grid(jungle_floor_texture, Vec2::new(16., 16.), 5, 5);
    let jungle_floor = texture_atlases.add(jungle_floor_atlas);

    let game_textures = GameTextures { jungle_floor };
    commands.insert_resource(game_textures);
}

fn spawn_world_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    // TODO: Add world loading and saving

    let mut spawn_tile = |index: usize, translation: Vec3| {
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_textures.jungle_floor.clone(),
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
        });
    };

    spawn_tile(0, Vec3::new(-16. * SPRITE_SCALE, 0., 0.));
    spawn_tile(2, Vec3::new(0., 0., 0.));
    spawn_tile(4, Vec3::new(16. * SPRITE_SCALE, 0., 0.));
    spawn_tile(20, Vec3::new(-16. * SPRITE_SCALE, -16. * SPRITE_SCALE, 0.));
    spawn_tile(22, Vec3::new(0., -16. * SPRITE_SCALE, 0.));
    spawn_tile(24, Vec3::new(16. * SPRITE_SCALE, -16. * SPRITE_SCALE, 0.));
}
