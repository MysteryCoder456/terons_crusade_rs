use bevy::{core::FixedTimestep, prelude::*};

use crate::{components::Player, SPRITE_SCALE, TIME_STEP};

const IDLE_SHEET: &str = "player/idle.png";
const RUN_SHEET: &str = "player/run.png";
const FALL_SHEET: &str = "player/fall.png";
const JUMP_SHEET: &str = "player/jump.png";
const LANDING_SHEET: &str = "player/landing.png";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_setup_system)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system)
            .add_system_set(
                // All physics/animation related stuff here
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(player_animation_system),
            );
    }
}

struct PlayerTextures {
    pub idle: Handle<TextureAtlas>,
    pub run: Handle<TextureAtlas>,
    pub fall: Handle<TextureAtlas>,
    pub jump: Handle<TextureAtlas>,
    pub landing: Handle<TextureAtlas>,
}

fn player_setup_system(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    // Idle
    let idle_texture = asset_server.load(IDLE_SHEET);
    let idle_atlas = TextureAtlas::from_grid(idle_texture, Vec2::new(19., 34.), 12, 1);
    let idle_handle = texture_atlases.add(idle_atlas);

    // Run
    let run_texture = asset_server.load(RUN_SHEET);
    let run_atlas = TextureAtlas::from_grid(run_texture, Vec2::new(21., 33.), 8, 1);
    let run_handle = texture_atlases.add(run_atlas);

    // Fall
    let fall_texture = asset_server.load(FALL_SHEET);
    let fall_atlas = TextureAtlas::from_grid(fall_texture, Vec2::new(20., 35.), 2, 1);
    let fall_handle = texture_atlases.add(fall_atlas);

    // Jump
    let jump_texture = asset_server.load(JUMP_SHEET);
    let jump_atlas = TextureAtlas::from_grid(jump_texture, Vec2::new(17., 34.), 1, 1);
    let jump_handle = texture_atlases.add(jump_atlas);

    // Landing
    let landing_texture = asset_server.load(LANDING_SHEET);
    let landing_atlas = TextureAtlas::from_grid(landing_texture, Vec2::new(20., 35.), 1, 1);
    let landing_handle = texture_atlases.add(landing_atlas);

    // PlayerTextures resource
    let player_textures = PlayerTextures {
        idle: idle_handle,
        run: run_handle,
        fall: fall_handle,
        jump: jump_handle,
        landing: landing_handle,
    };
    commands.insert_resource(player_textures);
}

fn spawn_player_system(mut commands: Commands, game_textures: Res<PlayerTextures>) {
    // TODO: Load this from world save
    let spawn_pos = Vec3::new(0., 60., 0.);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_textures.idle.clone(),
            transform: Transform {
                translation: spawn_pos,
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player::default());
}

fn player_animation_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Player, &Handle<TextureAtlas>)>,
) {
    if let Ok((mut sprite, mut player, atlas_handle)) = query.get_single_mut() {
        player.animation_timer.tick(time.delta());

        if player.animation_timer.finished() {
            let texture_atlas = texture_atlases.get(atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}