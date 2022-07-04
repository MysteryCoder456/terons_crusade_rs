use bevy::prelude::*;

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
pub enum AnimationState {
    IDLE,
    RUNNING,
    JUMPING,
    FALLING,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
