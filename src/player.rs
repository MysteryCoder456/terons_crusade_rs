use bevy::{
    core::{FixedTimestep, Zeroable},
    math::Vec3Swizzles,
    prelude::*,
    render::render_resource::internal::bytemuck::Contiguous,
    sprite::collide_aabb,
};
use bevy_rapier2d::prelude::*;

use crate::{
    components::{
        AnimationState, AnimationStates, Inventory, Item, MainCamera, Player, PlayerAttractor,
        SpriteSize,
    },
    item::Items,
    GameState, SPRITE_SCALE, TIME_STEP,
};

const IDLE_SHEET: &str = "player/idle.png";
const RUN_SHEET: &str = "player/run.png";
const FALL_SHEET: &str = "player/fall.png";
const JUMP_SHEET: &str = "player/jump.png";

const PLAYER_SPEED: f32 = 170.;
const PLAYER_JUMP_SPEED: f32 = 530.;
const PLAYER_REACH: f32 = 120.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player_setup_system)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(spawn_player_system))
            .add_system_set(
                // All physics and movement related systems here
                SystemSet::on_update(GameState::Game)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(player_camera_follow_system)
                    .with_system(player_movement_system)
                    .with_system(player_attractor_system)
                    .with_system(player_item_collision_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(player_animation_system)
                    .with_system(player_item_pickup_system),
            );
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
        .insert(MassProperties {
            mass: 10.0,
            ..Default::default()
        })
        .insert(Velocity::zero())
        .insert(Player::default())
        .insert(AnimationState::default())
        .insert(Inventory::default());
}

/// System that makes the main camera follow the player
fn player_camera_follow_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    if let Ok(player_tf) = player_query.get_single() {
        if let Ok(mut camera_tf) = camera_query.get_single_mut() {
            let cam_velocity = (player_tf.translation - camera_tf.translation) * 5.;
            camera_tf.translation += cam_velocity * time.delta_seconds();
        }
    }
}

/// System to animate the player's `TextureAtlas`
fn player_animation_system(
    time: Res<Time>,
    player_textures: Res<PlayerTextures>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut TextureAtlasSprite,
        &mut Player,
        &mut Handle<TextureAtlas>,
        &AnimationState,
    )>,
) {
    if let Ok((mut sprite, mut player, mut atlas_handle, anim_state)) = query.get_single_mut() {
        player.animation_timer.tick(time.delta());

        atlas_handle.id = match anim_state.current {
            AnimationStates::Idle => player_textures.idle.id,
            AnimationStates::Running => player_textures.run.id,
            AnimationStates::Falling => player_textures.fall.id,
            AnimationStates::Jumping => player_textures.jump.id,
        };

        if player.animation_timer.finished() {
            let texture_atlas = texture_atlases.get(atlas_handle.clone()).unwrap();
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

        // Jumping
        if (kb.just_pressed(KeyCode::W) || kb.just_pressed(KeyCode::Space))
            && -2. < velocity.linvel.y
            && velocity.linvel.y < 2.
        {
            velocity.linvel.y += PLAYER_JUMP_SPEED;
        }

        // Orient sprite in correct direction
        if direction > 0. {
            sprite.flip_x = false;
        } else if direction < 0. {
            sprite.flip_x = true;
        }

        // Update state machine
        anim_state.previous = anim_state.current.clone();
        anim_state.current = if velocity.linvel.y < -12. {
            AnimationStates::Falling
        } else if velocity.linvel.y > 12. {
            AnimationStates::Jumping
        } else if direction != 0. {
            AnimationStates::Running
        } else {
            AnimationStates::Idle
        };

        // Reset sprite index to 0 to avoid animation issues
        if anim_state.current != anim_state.previous {
            sprite.index = 0;
        }
    }
}

/// System that handles item pickups by the player
fn player_item_pickup_system(
    kb: Res<Input<KeyCode>>,
    player_query: Query<&Transform, (With<Player>, Without<Item>)>,
    mut item_query: Query<(Entity, &Transform, &mut Item)>,
    mut commands: Commands,
) {
    if kb.just_pressed(KeyCode::E) {
        if player_query.is_empty() {
            return;
        }
        let player_tf = player_query.get_single().unwrap();

        for (item_entity, item_tf, mut item) in item_query.iter_mut() {
            if item.picked_up {
                continue;
            }

            if player_tf.translation.distance(item_tf.translation) <= PLAYER_REACH {
                item.picked_up = true;
                commands
                    .entity(item_entity)
                    .insert(PlayerAttractor { strength: 2600. });
            }
        }
    }
}

fn player_item_collision_system(
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    items: Res<Items>,
    mut player_query: Query<(&Transform, &Handle<TextureAtlas>, &mut Inventory), With<Player>>,
    item_query: Query<(Entity, &Transform, &SpriteSize, &Item)>,
) {
    if let Ok((player_tf, player_atlas_handle, mut player_inv)) = player_query.get_single_mut() {
        let player_atlas_first = texture_atlases
            .get(player_atlas_handle)
            .unwrap()
            .textures
            .first()
            .unwrap();
        let player_sprite_size = player_atlas_first.max - player_atlas_first.min;

        for (item_entity, item_tf, item_size, item) in item_query.iter() {
            if !item.picked_up {
                continue;
            }

            let col = collide_aabb::collide(
                player_tf.translation,
                player_sprite_size * player_tf.scale.xy(),
                item_tf.translation,
                item_size.0 * item_tf.scale.xy(),
            );

            if col.is_some() {
                if let Some(item_data) = items.get(&item.item_name) {
                    let inv = &mut player_inv.0;

                    if let Some(current_count) = inv.get_mut(&item.item_name) {
                        if *current_count < item_data.stack_size {
                            *current_count += 1;
                        }
                    } else {
                        inv.insert(item.item_name.clone(), 1);
                    }
                }

                commands.entity(item_entity).despawn();
            }
        }
    }
}

/// System that handles attraction of entities to the player
fn player_attractor_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut attractor_query: Query<(&Transform, &mut Velocity, &PlayerAttractor)>,
) {
    if let Ok(player_tf) = player_query.get_single() {
        for (attr_tf, mut attr_velocity, attractor) in attractor_query.iter_mut() {
            let acceleration =
                (player_tf.translation - attr_tf.translation).normalize() * attractor.strength;
            attr_velocity.linvel +=
                Vec2::new(acceleration.x, acceleration.y) * time.delta_seconds();
        }
    }
}
