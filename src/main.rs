use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use components::MainCamera;
use player::PlayerPlugin;
use save_data::SaveDataPlugin;
use tile_map::TileMapPlugin;

mod components;
mod player;
mod save_data;
mod tile_map;

const TIME_STEP: f32 = 1.0 / 60.0;
const SPRITE_SCALE: f32 = 2.5;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("87CEEB").unwrap()))
        .insert_resource(WindowDescriptor {
            title: "Teron's Crusade".to_owned(),
            width: 1080.,
            height: 720.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            SPRITE_SCALE,
        ))
        .add_plugin(PlayerPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(SaveDataPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    // Add camera bundles
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    // Add Rapier configurations
    let rapier_config = RapierConfiguration {
        gravity: Vec2::new(0., -1500.),
        ..Default::default()
    };
    commands.insert_resource(rapier_config);
}
