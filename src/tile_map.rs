use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

use crate::{
    components::{Block, SpawnBlock},
    SPRITE_SCALE,
};

const JUNGLE_FLOOR_SHEET: &str = "tile_sets/overworld/jungle_floor.png";
const BLOCK_SIZE: f32 = 16.;

type TileSets = HashMap<String, Handle<TextureAtlas>>;

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(tile_map_setup_system)
            .add_system(block_spawn_system);
    }
}

fn tile_map_setup_system(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    // Jungle floor texture atlas
    let jungle_floor_texture = asset_server.load(JUNGLE_FLOOR_SHEET);
    let jungle_floor_atlas = TextureAtlas::from_grid(
        jungle_floor_texture,
        Vec2::new(BLOCK_SIZE, BLOCK_SIZE),
        5,
        5,
    );
    let jungle_floor = texture_atlases.add(jungle_floor_atlas);

    let mut tile_sets = TileSets::new();
    tile_sets.insert("jungle_floor".to_owned(), jungle_floor);
    commands.insert_resource(tile_sets);
}

fn block_spawn_system(
    mut commands: Commands,
    tile_sets: Res<TileSets>,
    query: Query<(Entity, &SpawnBlock)>,
) {
    for (entity, spawn_data) in query.iter() {
        if let Some(atlas_handle) = tile_sets.get(&spawn_data.tile_set) {
            let translation = Vec3::new(spawn_data.tile_pos.x, spawn_data.tile_pos.y, 0.0)
                * SPRITE_SCALE
                * BLOCK_SIZE;
            let mut new_handle = Handle::<TextureAtlas>::default();
            new_handle.id = atlas_handle.id;

            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: new_handle,
                    transform: Transform {
                        translation,
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, SPRITE_SCALE),
                        ..Default::default()
                    },
                    sprite: TextureAtlasSprite {
                        index: spawn_data.tile_index,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Block)
                .insert(RigidBody::Fixed)
                .insert(Collider::cuboid(BLOCK_SIZE / 2., BLOCK_SIZE / 2.));
        } else {
            eprintln!(
                "Tried to spawn block belonging to undefined tile set: {}",
                spawn_data.tile_set
            );
        }

        commands.entity(entity).despawn();
    }
}
