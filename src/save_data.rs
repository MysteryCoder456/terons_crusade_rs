use bevy::{prelude::*, utils::HashSet};
use serde::{Deserialize, Serialize};

use crate::components::SpawnBlock;

const SAVE_DATA_PATH: &str = "world_saves";

pub struct SaveDataPlugin;

impl Plugin for SaveDataPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, save_data_setup_system);
    }
}

// TODO: Make player data

#[derive(Serialize, Deserialize)]
struct WorldSaveData {
    pub blocks: HashSet<BlockData>,
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

        Self {
            blocks: default_blocks,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
struct BlockData {
    pub tile_set: String,
    pub tile_index: usize,
    pub tile_pos: PositionData,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
struct PositionData {
    x: i32,
    y: i32,
}

/// System that loads/generates the game save data.
/// You'll see a lot of `unwrap` and/or `expect` calls here since
/// having the game crash at startup is usually not as annoying.
fn save_data_setup_system(mut commands: Commands) {
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

    // Spawn world
    for block_data in world_data.blocks {
        commands.spawn().insert(SpawnBlock {
            tile_set: block_data.tile_set,
            tile_index: block_data.tile_index,
            tile_pos: Vec2::new(block_data.tile_pos.x as f32, block_data.tile_pos.y as f32),
        });
    }
}
