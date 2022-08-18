use bevy::{prelude::*, ui::FocusPolicy};

use crate::{GameState, UIAssets};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu).with_system(main_menu_setup_system),
        );
    }
}

fn main_menu_setup_system(mut commands: Commands, ui_assets: Res<UIAssets>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
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
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
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
}
