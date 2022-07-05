use bevy::prelude::*;

// Entity Types

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
            current: AnimationStates::IDLE,
            previous: AnimationStates::IDLE,
        }
    }
}

#[derive(Component, Clone, PartialEq)]
pub enum AnimationStates {
    IDLE,
    RUNNING,
    JUMPING,
    FALLING,
}

// Entity Components
