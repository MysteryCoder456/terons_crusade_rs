use bevy::{prelude::*, utils::HashMap};

use crate::GameState;

// UI Components

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct MainMenuFader {
    pub fade_timer: Timer,
    pub next_state: GameState,
}

impl MainMenuFader {
    pub fn new(next_state: GameState) -> Self {
        Self {
            fade_timer: Timer::from_seconds(2., false),
            next_state,
        }
    }
}

#[derive(Component)]
pub enum MainMenuButton {
    NewGame,
    LoadGame,
    Options,
}

// UI Components

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

/// Contains HashMap with key as item name and value as item quantity.
#[derive(Component, Default)]
pub struct Inventory {
    pub slots: Vec<(String, usize)>,
    pub max_slots: usize,
}

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

// Entity Components
