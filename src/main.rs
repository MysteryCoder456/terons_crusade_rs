use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use components::MainCamera;
use item::ItemPlugin;
use main_menu::MainMenuPlugin;
use player::PlayerPlugin;
use save_data::SaveDataPlugin;
use tile_map::TileMapPlugin;

mod components;
mod inventory;
mod item;
mod main_menu;
mod player;
mod save_data;
mod tile_map;

const TIME_STEP: f32 = 1.0 / 60.0;
const SPRITE_SCALE: f32 = 2.5;

const PIXELLARI_FONT: &str = "fonts/Pixellari.ttf";
const BUTTON_SPRITE: &str = "ui/button/button.png";
const BUTTON_PRESSED_SPRITE: &str = "ui/button/button_pressed.png";
const INVENTORY_SLOT_SPRITE: &str = "ui/inventory/inventory_slot.png";
const INVENTORY_SLOT_SELECTED_SPRITE: &str = "ui/inventory/inventory_slot_selected.png";
const INVENTORY_BG_SPRITE: &str = "ui/inventory/inventory_bg.png";

struct UIAssets {
    font: Handle<Font>,
    button: Handle<Image>,
    button_pressed: Handle<Image>,
    inventory_slot: Handle<Image>,
    inventory_slot_selected: Handle<Image>,
    inventory_bg: Handle<Image>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    MainMenu,
    NewGameMenu,
    OptionsMenu,
    Game,
    Inventory,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("87CEEB").unwrap()))
        .insert_resource(WindowDescriptor {
            title: "Teron's Crusade".to_owned(),
            width: 1080.,
            height: 720.,
            ..Default::default()
        })
        .add_state(GameState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            SPRITE_SCALE,
        ))
        .add_plugin(MainMenuPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TileMapPlugin)
        .add_plugin(SaveDataPlugin)
        .add_plugin(ItemPlugin)
        .add_startup_system(setup_system)
        .add_startup_system(ui_assets_setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    // Add camera bundles
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands.spawn_bundle(UiCameraBundle::default());

    // Add Rapier configurations
    let rapier_config = RapierConfiguration {
        gravity: Vec2::new(0., -1500.),
        ..Default::default()
    };
    commands.insert_resource(rapier_config);
}

fn ui_assets_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ui_assets = UIAssets {
        font: asset_server.load(PIXELLARI_FONT),
        button: asset_server.load(BUTTON_SPRITE),
        button_pressed: asset_server.load(BUTTON_PRESSED_SPRITE),
        inventory_slot: asset_server.load(INVENTORY_SLOT_SPRITE),
        inventory_slot_selected: asset_server.load(INVENTORY_SLOT_SELECTED_SPRITE),
        inventory_bg: asset_server.load(INVENTORY_BG_SPRITE),
    };
    commands.insert_resource(ui_assets);
}
