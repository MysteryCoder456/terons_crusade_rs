use bevy::{prelude::*, utils::HashMap};
// use bevy_rapier2d::prelude::*;

use crate::{components::SpawnItem, SPRITE_SCALE};

const ITEMS_DIR: &str = "assets/items";

type ItemSprites = HashMap<String, Handle<Image>>;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, item_setup_system)
            .add_system(item_spawn_system);
    }
}

fn item_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let item_sprites = std::fs::read_dir(ITEMS_DIR)
        .unwrap()
        .filter_map(|dir| match dir {
            Ok(item_dir) => {
                let os_str = item_dir.file_name();
                let item_name = os_str.to_str().unwrap();

                let sprite_path = format!("items/{0}/{0}.png", item_name);
                let img_handle: Handle<Image> = asset_server.load(&sprite_path);

                Some((item_name.to_owned(), img_handle))
            }
            Err(e) => {
                eprintln!("Error while loading item: {}", e);
                None
            }
        })
        .collect::<ItemSprites>();

    println!("{:?}", item_sprites);
    commands.insert_resource(item_sprites);
}

fn item_spawn_system(
    mut commands: Commands,
    item_sprites: Res<ItemSprites>,
    query: Query<(Entity, &SpawnItem)>,
) {
    for (entity, spawn_item) in query.iter() {
        if let Some(item_img) = item_sprites.get(&spawn_item.item_name) {
            commands.spawn_bundle(SpriteBundle {
                texture: item_img.clone(),
                transform: Transform {
                    translation: Vec3::new(spawn_item.position.x, spawn_item.position.y, 0.0),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, SPRITE_SCALE),
                    ..Default::default()
                },
                ..Default::default()
            });
            commands.entity(entity).despawn();
        } else {
            eprintln!("Tried to spawn undefined item: {}", spawn_item.item_name);
        }
    }
}
