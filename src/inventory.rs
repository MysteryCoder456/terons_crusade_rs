use bevy::prelude::*;

use crate::{GameState, UIAssets};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Inventory).with_system(inventory_setup_system),
        );
    }
}

fn inventory_setup_system(mut commands: Commands, ui_assets: Res<UIAssets>) {
    todo!();
}
