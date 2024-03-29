use bevy::{prelude::*, ui::FocusPolicy};

use crate::{
    components::{MainMenu, MainMenuButton, MainMenuFader},
    GameState, UIAssets,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu).with_system(main_menu_setup_system),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::MainMenu).with_system(main_menu_unload_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::MainMenu)
                .with_system(main_menu_interaction_system)
                .with_system(main_menu_fade_system),
        );
    }
}

// System that spawns the main menu UI.
fn main_menu_setup_system(mut commands: Commands, ui_assets: Res<UIAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                flex_direction: FlexDirection::ColumnReverse,
                padding: Rect {
                    top: Val::Px(40.),
                    bottom: Val::Px(40.),
                    left: Val::Undefined,
                    right: Val::Undefined,
                },
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            focus_policy: FocusPolicy::Pass,
            ..Default::default()
        })
        .insert(MainMenu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Teron's Crusade",
                    TextStyle {
                        font: ui_assets.font.clone(),
                        font_size: 50.,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    },
                ),
                focus_policy: FocusPolicy::Pass,
                ..Default::default()
            });

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Percent(20.), Val::Percent(10.)),
                        margin: Rect::all(Val::Auto),
                        ..Default::default()
                    },
                    color: UiColor(Color::NONE),

                    ..Default::default()
                })
                .insert(MainMenuButton::LoadGame)
                .with_children(|button| {
                    button.spawn_bundle(ImageBundle {
                        image: UiImage(ui_assets.button.clone()),
                        style: Style {
                            position_type: PositionType::Absolute,
                            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                            ..Default::default()
                        },
                        focus_policy: FocusPolicy::Pass,
                        ..Default::default()
                    });

                    button.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Play",
                            TextStyle {
                                font: ui_assets.font.clone(),
                                color: Color::WHITE,
                                font_size: 32.,
                            },
                            TextAlignment {
                                horizontal: HorizontalAlign::Center,
                                vertical: VerticalAlign::Center,
                            },
                        ),
                        focus_policy: FocusPolicy::Pass,
                        ..Default::default()
                    });
                });
        });
}

// System that despawns the main menu when switching states.
fn main_menu_unload_system(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

// System that handles button interactions in the main menu.
fn main_menu_interaction_system(
    mut commands: Commands,
    ui_assets: Res<UIAssets>,
    interaction_query: Query<(&Children, &MainMenuButton, &Interaction), Changed<Interaction>>,
    main_menu_query: Query<Entity, With<MainMenu>>,
    mut image_query: Query<&mut UiImage>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        for (children, button, interaction) in interaction_query.iter() {
            let image_child = children.iter().next().unwrap();
            let mut button_image = image_query.get_mut(*image_child).unwrap();

            match interaction {
                Interaction::Clicked => {
                    button_image.0 = ui_assets.button_pressed.clone();

                    let next_state = match button {
                        MainMenuButton::NewGame => GameState::NewGameMenu,
                        MainMenuButton::LoadGame => GameState::Game,
                        MainMenuButton::Options => GameState::OptionsMenu,
                    };

                    commands.entity(main_menu_entity).with_children(|parent| {
                        parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    align_self: AlignSelf::Center,
                                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                    ..Default::default()
                                },
                                color: UiColor(Color::hsla(0., 0., 0., 0.)),
                                focus_policy: FocusPolicy::Block,
                                ..Default::default()
                            })
                            .insert(MainMenuFader::new(next_state));
                    });
                }
                Interaction::Hovered | Interaction::None => {
                    button_image.0 = ui_assets.button.clone();
                }
            }
        }
    }
}

// System that handles screen fading and state switching.
fn main_menu_fade_system(
    time: Res<Time>,
    mut game_state: ResMut<State<GameState>>,
    mut query: Query<(&mut MainMenuFader, &mut UiColor)>,
) {
    if let Ok((mut menu_fader, mut ui_color)) = query.get_single_mut() {
        menu_fader.fade_timer.tick(time.delta());

        let alpha =
            menu_fader.fade_timer.elapsed_secs() / menu_fader.fade_timer.duration().as_secs_f32();
        ui_color.0.set_a(alpha);

        if menu_fader.fade_timer.finished() {
            game_state.set(menu_fader.next_state).unwrap();
        }
    }
}
