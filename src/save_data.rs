use bevy::{prelude::*, utils::HashSet};
use serde::{Deserialize, Serialize};

const SAVE_DATA_PATH: &str = "save_data";

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
    fn default() -> Self {
        Self {
            blocks: HashSet::default(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
struct BlockData {
    pub tile_set: String,
    pub tile_id: usize,
    pub position: PositionData,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
struct PositionData {
    x: i32,
    y: i32,
}

/// System that loads/generates the game save data.
/// You'll a lot of `unwrap` and/or `expect` calls here since
/// having the game crash at startup is usually not as annoying.
fn save_data_setup_system() {
    // Path to directory that holds save files
    let save_data_path = std::path::Path::new(SAVE_DATA_PATH);

    if !save_data_path.is_dir() {
        // Create save data directory if doesn't exist
        std::fs::create_dir_all(save_data_path).expect("Error creating save data directory!");
    }

    // Load world data
    let world_data_path = save_data_path.join("world_data.json");

    if world_data_path.exists() {
        let json_text =
            std::fs::read_to_string(world_data_path).expect("Error reading world save data!");
        let world_data: WorldSaveData =
            serde_json::from_str(&json_text).expect("Error parsing world data!");

        // TODO: Load world
    } else {
        let default_world_data = serde_json::to_string(&WorldSaveData::default())
            .expect("Error serializing world data!");
        std::fs::write(world_data_path, default_world_data).expect("Error writing world data!");
    }
}
