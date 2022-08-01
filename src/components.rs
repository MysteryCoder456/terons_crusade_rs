use bevy::prelude::*;

// Entity Types

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Block;

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
    pub animation_timer: Timer,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            animation_timer: Timer::from_seconds(0.1, true),
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
