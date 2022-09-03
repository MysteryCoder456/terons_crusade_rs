use bevy::{
    prelude::*,
    ui::{widget::ImageMode, FocusPolicy},
};

use crate::{
    components::{Inventory, InventoryMenu, Player},
    item::Items,
    GameState, UIAssets,
};

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

fn inventory_menu_setup_system(
    mut commands: Commands,
    ui_assets: Res<UIAssets>,
    items: Res<Items>,
    query: Query<&Inventory, With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    let player_inv: &Inventory = query.single();

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
            parent
                .spawn_bundle(ImageBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::FlexEnd,
                        size: Size::new(Val::Percent(60.), Val::Percent(60.)),
                        flex_direction: FlexDirection::RowReverse,
                        padding: Rect::all(Val::Percent(3.)),
                        ..Default::default()
                    },
                    focus_policy: FocusPolicy::Pass,
                    image: UiImage(ui_assets.inventory_bg.clone()),
                    image_mode: ImageMode::KeepAspect,
                    ..Default::default()
                })
                .with_children(|inventory| {
                    for (item_name, item_count) in &player_inv.slots {
                        if let Some(item_data) = items.get(item_name) {
                            inventory
                                .spawn_bundle(ImageBundle {
                                    style: Style {
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        size: Size::new(Val::Px(60.), Val::Px(60.)),
                                        ..Default::default()
                                    },
                                    image: UiImage(ui_assets.inventory_slot.clone()),
                                    ..Default::default()
                                })
                                .with_children(|slot| {
                                    // Item Icon
                                    slot.spawn_bundle(ImageBundle {
                                        style: Style {
                                            position_type: PositionType::Absolute,
                                            size: Size::new(Val::Percent(75.), Val::Percent(75.)),
                                            ..Default::default()
                                        },
                                        image: UiImage(item_data.sprite.clone()),
                                        ..Default::default()
                                    });

                                    if *item_count > 1 {
                                        // Item Count Label
                                        slot.spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                item_count.to_string(),
                                                TextStyle {
                                                    color: Color::DARK_GREEN,
                                                    font: ui_assets.font.clone(),
                                                    font_size: 16.,
                                                },
                                                TextAlignment {
                                                    horizontal: HorizontalAlign::Center,
                                                    vertical: VerticalAlign::Center,
                                                },
                                            ),
                                            style: Style {
                                                position: Rect {
                                                    bottom: Val::Percent(5.),
                                                    right: Val::Auto,
                                                    left: Val::Percent(10.),
                                                    top: Val::Auto,
                                                },
                                                size: Size::new(
                                                    Val::Percent(100.),
                                                    Val::Percent(100.),
                                                ),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        });
                                    }
                                });
                        }
                    }
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
