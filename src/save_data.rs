use bevy::{app::AppExit, core::FixedTimestep, prelude::*, utils::HashSet};
use serde::{Deserialize, Serialize};

use crate::{
    components::{Block, Item, Player},
    item::SpawnItemEvent,
    player::SpawnPlayerEvent,
    tile_map::{SpawnBlockEvent, BLOCK_SIZE},
    GameState, SPRITE_SCALE,
};

const SAVE_DATA_PATH: &str = "world_saves";

pub struct SaveDataPlugin;

impl Plugin for SaveDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Game).with_system(save_data_setup_system),
        )
        .add_system_set(SystemSet::on_update(GameState::Game).with_system(app_exit_save_system))
        .add_system_set(
            // Save world data every 5 minutes
            SystemSet::on_update(GameState::Game)
                .with_run_criteria(FixedTimestep::step(300.))
                .with_system(periodic_save_system),
        );
    }
}

// TODO: Make player data

#[derive(Serialize, Deserialize)]
struct WorldSaveData {
    pub player_spawn: PositionData,
    pub blocks: HashSet<BlockData>,
    pub items: HashSet<ItemData>,
}

impl Default for WorldSaveData {
    /// Default world, will change in the future.
    fn default() -> Self {
        let mut default_blocks = HashSet::<BlockData>::new();

        default_blocks.insert(BlockData {
            tile_set: "jungle_floor".to_owned(),
            tile_index: 0,
            tile_pos: PositionData { x: -3, y: 0 },
        });
        default_blocks.insert(BlockData {
            tile_set: "jungle_floor".to_owned(),
            tile_index: 2,
            tile_pos: PositionData { x: -2, y: 0 },
        });
        default_blocks.insert(BlockData {
            tile_set: "jungle_floor".to_owned(),
            tile_index: 2,
            tile_pos: PositionData { x: -1, y: 0 },
        });
        default_blocks.insert(BlockData {
            tile_set: "jungle_floor".to_owned(),
            tile_index: 2,
            tile_pos: PositionData { x: 0, y: 0 },
        });
        default_blocks.insert(BlockData {
            tile_set: "jungle_floor".to_owned(),
            tile_index: 2,
            tile_pos: PositionData { x: 1, y: 0 },
        });
        default_blocks.insert(BlockData {
            tile_set: "jungle_floor".to_owned(),
            tile_index: 2,
            tile_pos: PositionData { x: 2, y: 0 },
        });
        default_blocks.insert(BlockData {
            tile_set: "jungle_floor".to_owned(),
            tile_index: 4,
            tile_pos: PositionData { x: 3, y: 0 },
        });

        default_blocks.insert(BlockData {
            tile_set: "jungle_floor".to_owned(),
            tile_index: 20,
            tile_pos: PositionData { x: -3, y: -1 },
        });
        default_blocks.insert(BlockData {
            tile_set: "jungle_floor".to_owned(),
            tile_index: 22,
            tile_pos: PositionData { x: -2, y: -1 },
        });
        default_blocks.insert(BlockData {
            tile_set: "jungle_floor".to_owned(),
            tile_index: 22,
            tile_pos: PositionData { x: -1, y: -1 },
        });
        default_blocks.insert(BlockData {
            tile_set: "jungle_floor".to_owned(),
            tile_index: 22,
            tile_pos: PositionData { x: 0, y: -1 },
        });
        default_blocks.insert(BlockData {
            tile_set: "jungle_floor".to_owned(),
            tile_index: 22,
            tile_pos: PositionData { x: 1, y: -1 },
        });
        default_blocks.insert(BlockData {
            tile_set: "jungle_floor".to_owned(),
            tile_index: 22,
            tile_pos: PositionData { x: 2, y: -1 },
        });
        default_blocks.insert(BlockData {
            tile_set: "jungle_floor".to_owned(),
            tile_index: 24,
            tile_pos: PositionData { x: 3, y: -1 },
        });

        let mut default_items = HashSet::<ItemData>::new();

        default_items.insert(ItemData {
            item_name: "pickaxe".to_owned(),
            position: PositionData { x: 40, y: 80 },
        });

        Self {
            player_spawn: PositionData { x: 0, y: 300 },
            blocks: default_blocks,
            items: default_items,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
struct BlockData {
    tile_set: String,
    tile_index: usize,
    tile_pos: PositionData,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
struct ItemData {
    pub item_name: String,
    pub position: PositionData,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
struct PositionData {
    x: i32,
    y: i32,
}

/// System that loads/generates the game save data.
/// You'll see a lot of `unwrap` and/or `expect` calls here since
/// having the game crash at startup is usually not as annoying.
fn save_data_setup_system(
    mut block_events: EventWriter<SpawnBlockEvent>,
    mut item_events: EventWriter<SpawnItemEvent>,
    mut player_events: EventWriter<SpawnPlayerEvent>,
) {
    // Path to directory that holds save files
    let save_data_path = std::path::Path::new(SAVE_DATA_PATH);

    if !save_data_path.is_dir() {
        // Create save data directory if doesn't exist
        std::fs::create_dir_all(save_data_path).expect("Error creating save data directory!");
    }

    // Load world data
    // TODO: Add multiple save files in the future
    let world_data_path = save_data_path.join("world0.save");

    let world_data = if world_data_path.exists() {
        // Load world data from save file
        let world_data_bytes =
            std::fs::read(world_data_path).expect("Error reading world save data!");
        bincode::deserialize(&world_data_bytes).expect("Error deserializing world data!")
    } else {
        // Generate and save default world
        let default_world_data = WorldSaveData::default();
        let default_world_serialized =
            bincode::serialize(&default_world_data).expect("Error serializing world data!");
        std::fs::write(world_data_path, default_world_serialized)
            .expect("Error writing world data!");

        default_world_data
    };

    // Spawn blocks
    block_events.send_batch(world_data.blocks.iter().map(|block_data| SpawnBlockEvent {
        tile_set: block_data.tile_set.clone(),
        tile_index: block_data.tile_index,
        tile_pos: Vec2::new(block_data.tile_pos.x as f32, block_data.tile_pos.y as f32),
    }));

    // Spawn items
    item_events.send_batch(world_data.items.iter().map(|item_data| SpawnItemEvent {
        item_name: item_data.item_name.clone(),
        position: Vec2::new(item_data.position.x as f32, item_data.position.y as f32),
    }));

    // Spawn player
    player_events.send(SpawnPlayerEvent {
        position: Vec3::new(
            world_data.player_spawn.x as f32,
            world_data.player_spawn.y as f32,
            0.0,
        ),
    });
}

/// System that saves the world data every 5 minutes.
fn periodic_save_system(
    block_query: Query<(&Transform, &Block)>,
    item_query: Query<(&Transform, &Item)>,
    player_query: Query<&Transform, With<Player>>,
) {
    save_world_data(block_query, item_query, player_query);
}

/// System that save the world data when the game is closed.
fn app_exit_save_system(
    app_exit_events: EventReader<AppExit>,
    block_query: Query<(&Transform, &Block)>,
    item_query: Query<(&Transform, &Item)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if !app_exit_events.is_empty() {
        save_world_data(block_query, item_query, player_query);
    }
}

/// Saves world data. This is NOT a system, despite having function
/// parameters that look like a system's. This is because it is
/// being called by periodic_save_system and app_exit_save_system
/// to avoid code duplication.
fn save_world_data(
    block_query: Query<(&Transform, &Block)>,
    item_query: Query<(&Transform, &Item)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let blocks: HashSet<BlockData> = block_query
        .iter()
        .map(|(block_tf, block)| BlockData {
            tile_set: block.tile_set.clone(),
            tile_index: block.tile_index,
            tile_pos: PositionData {
                x: (block_tf.translation.x / BLOCK_SIZE / SPRITE_SCALE) as i32,
                y: (block_tf.translation.y / BLOCK_SIZE / SPRITE_SCALE) as i32,
            },
        })
        .collect();

    let items: HashSet<ItemData> = item_query
        .iter()
        .map(|(item_tf, item)| ItemData {
            item_name: item.item_name.clone(),
            position: PositionData {
                x: item_tf.translation.x as i32,
                y: item_tf.translation.y as i32,
            },
        })
        .collect();

    let player_position = player_query.single().translation;
    let player_spawn = PositionData {
        x: player_position.x as i32,
        y: player_position.y as i32,
    };

    let world_data = WorldSaveData {
        player_spawn,
        blocks,
        items,
    };

    match bincode::serialize(&world_data) {
        Ok(world_data_serialized) => {
            // TODO: Add multiple save files in the future
            let world_data_path = std::path::Path::new(SAVE_DATA_PATH).join("world0.save");

            if let Err(e) = std::fs::write(world_data_path, world_data_serialized) {
                eprintln!("Error writing world data: {}", e);
            }
        }
        Err(e) => eprintln!("Error serializing world data: {}", e),
    }
}
