use bevy::{
    prelude::*,
    ui::{widget::ImageMode, FocusPolicy},
};

use crate::{components::InventoryMenu, GameState, UIAssets};

pub struct InventoryMenuPlugin;

impl Plugin for InventoryMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Inventory).with_system(inventory_menu_setup_system),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Inventory).with_system(inventory_menu_unload_system),
        )
        .add_system(inventory_menu_toggle_system);
    }
}

fn inventory_menu_setup_system(mut commands: Commands, ui_assets: Res<UIAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..Default::default()
            },
            color: UiColor(Color::rgba(0., 0., 0., 0.4)),
            focus_policy: FocusPolicy::Pass,
            ..Default::default()
        })
        .insert(InventoryMenu)
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(60.), Val::Auto),
                    ..Default::default()
                },
                focus_policy: FocusPolicy::Pass,
                image: UiImage(ui_assets.inventory_bg.clone()),
                image_mode: ImageMode::KeepAspect,
                ..Default::default()
            });
        });
}

fn inventory_menu_unload_system(mut commands: Commands, query: Query<Entity, With<InventoryMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

fn inventory_menu_toggle_system(kb: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if kb.just_pressed(KeyCode::E) {
        match *game_state.current() {
            GameState::Game => {
                if let Err(e) = game_state.push(GameState::Inventory) {
                    eprintln!("Something went wrong while pushing Inventory state: {}", e);
                }
            }
            GameState::Inventory => {
                if let Err(e) = game_state.pop() {
                    eprintln!("Something went wrong while popping Inventory state: {}", e);
                }
            }
            _ => {}
        }
    }
}
