use bevy::{
    core::{FixedTimestep, Zeroable},
    prelude::*,
    render::render_resource::internal::bytemuck::Contiguous,
};
use bevy_rapier2d::prelude::*;

use crate::{
    components::{AnimationState, AnimationStates, Player},
    SPRITE_SCALE, TIME_STEP,
};

const IDLE_SHEET: &str = "player/idle.png";
const RUN_SHEET: &str = "player/run.png";
const FALL_SHEET: &str = "player/fall.png";
const JUMP_SHEET: &str = "player/jump.png";

const PLAYER_SPEED: f32 = 160.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_setup_system)
            .add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system)
            .add_system_set(
                // All physics related stuff here
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(player_movement_system),
            )
            .add_system(player_texture_atlas_state_system)
            .add_system(player_animation_system);
    }
}

struct PlayerTextures {
    pub idle: Handle<TextureAtlas>,
    pub run: Handle<TextureAtlas>,
    pub fall: Handle<TextureAtlas>,
    pub jump: Handle<TextureAtlas>,
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

    // PlayerTextures resource
    let player_textures = PlayerTextures {
        idle: idle_handle,
        run: run_handle,
        fall: fall_handle,
        jump: jump_handle,
    };
    commands.insert_resource(player_textures);
}

fn spawn_player_system(mut commands: Commands, player_textures: Res<PlayerTextures>) {
    // TODO: Load this from world save
    let spawn_pos = Vec3::new(0., 300., 0.);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: player_textures.idle.clone(),
            transform: Transform {
                translation: spawn_pos,
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::capsule_y(8., 9.))
        .insert(Velocity::zero())
        .insert(GravityScale(1.))
        .insert(Player::default())
        .insert(AnimationState::default());
}

/// System to switch between player's `TextureAtlas`'s using a state machine
fn player_texture_atlas_state_system(
    player_textures: Res<PlayerTextures>,
    mut query: Query<(&AnimationState, &mut Handle<TextureAtlas>), With<Player>>,
) {
    if let Ok((anim_state, mut atlas_handle)) = query.get_single_mut() {
        match anim_state.current {
            AnimationStates::IDLE => atlas_handle.id = player_textures.idle.id,
            AnimationStates::RUNNING => atlas_handle.id = player_textures.run.id,
            AnimationStates::FALLING => atlas_handle.id = player_textures.fall.id,
            AnimationStates::JUMPING => atlas_handle.id = player_textures.jump.id,
        }
    }
}

/// System to animate the player's `TextureAtlas`
fn player_animation_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Player, &Handle<TextureAtlas>)>,
) {
    if let Ok((mut sprite, mut player, atlas_handle)) = query.get_single_mut() {
        player.animation_timer.tick(time.delta());

        // FIXME: program sometimes panics due to sprite index being invalid

        if player.animation_timer.finished() {
            let texture_atlas = texture_atlases.get(atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.len();
        }
    }
}

/// System that handles player movement
fn player_movement_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &mut Transform,
            &mut Velocity,
            &mut AnimationState,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    if let Ok((mut transform, mut velocity, mut anim_state, mut sprite)) = query.get_single_mut() {
        // Keep angular velocity and rotation fixed
        velocity.angvel = 0.;
        transform.rotation = Quat::zeroed();

        // Horizontal movement
        let direction = kb.pressed(KeyCode::D).into_integer() as f32
            - kb.pressed(KeyCode::A).into_integer() as f32;
        velocity.linvel.x = direction * PLAYER_SPEED;

        // Orient sprite in correct direction
        if direction > 0. {
            sprite.flip_x = false;
        } else if direction < 0. {
            sprite.flip_x = true;
        }

        // Update state machine
        anim_state.previous = anim_state.current.clone();
        anim_state.current = if velocity.linvel.y < -5. {
            AnimationStates::FALLING
        } else if velocity.linvel.y > 5. {
            AnimationStates::JUMPING
        } else if direction != 0. {
            AnimationStates::RUNNING
        } else {
            AnimationStates::IDLE
        };

        // Reset sprite index to 0 to avoid animation issues
        if anim_state.current != anim_state.previous {
            sprite.index = 0;
        }
    }
}
