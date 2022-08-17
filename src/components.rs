use bevy::prelude::*;

// Entity Types

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Block {
    pub tile_set: String,
    pub tile_index: usize,
}

#[derive(Component)]
pub struct Player {
    pub animation_timer: Timer,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            animation_timer: Timer::from_seconds(0.1, true),
        }
    }
}

#[derive(Component)]
pub struct Item {
    pub item_name: String,
    pub picked_up: bool,
}

impl Item {
    pub fn new(item_name: &str) -> Self {
        Self {
            item_name: item_name.to_owned(),
            picked_up: false,
        }
    }
}

// Entity Types

// Entity Components

#[derive(Component)]
pub struct AnimationState {
    pub current: AnimationStates,
    pub previous: AnimationStates,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            current: AnimationStates::Idle,
            previous: AnimationStates::Idle,
        }
    }
}

#[derive(Component, Clone, PartialEq)]
pub enum AnimationStates {
    Idle,
    Running,
    Jumping,
    Falling,
}

#[derive(Component)]
pub struct PlayerAttractor {
    pub strength: f32,
}

// Entity Components

// Entity Spawner Components

#[derive(Component)]
pub struct SpawnBlock {
    pub tile_set: String,
    pub tile_index: usize,
    pub tile_pos: Vec2,
}

#[derive(Component)]
pub struct SpawnItem {
    pub item_name: String,
    pub position: Vec2,
}

// Entity Spawner Components
