use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
// use bevy_rapier2d::prelude::*;

use crate::{
    components::{Item, SpawnItem},
    SPRITE_SCALE,
};

const ITEMS_DIR: &str = "assets/items";
const ITEM_SPRITE_SCALE: f32 = SPRITE_SCALE * 0.17;

type Items = HashMap<String, ItemData>;

#[derive(Deserialize)]
struct ItemData {
    category: String,
    subcategory: Option<String>,
    stack_size: u32,
    damage: Option<f32>,
    knockback: Option<f32>,

    #[serde(skip)]
    sprite: Handle<Image>,
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, item_setup_system)
            .add_system(item_spawn_system);
    }
}

fn item_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let items = std::fs::read_dir(ITEMS_DIR)
        .unwrap()
        .filter_map(|dir| match dir {
            Ok(item_dir) => {
                if !item_dir.path().is_dir() {
                    return None;
                }

                let os_str = item_dir.file_name();
                let item_name = os_str.to_str().unwrap();

                let sprite_path = format!("items/{0}/{0}.png", item_name);
                let data_path = format!("assets/items/{0}/{0}.json", item_name);

                let img_handle: Handle<Image> = asset_server.load(&sprite_path);
                let item_data_str = std::fs::read_to_string(data_path).unwrap();

                let data = serde_json::from_str::<ItemData>(&item_data_str);

                if data.is_err() {
                    eprintln!("Error loading item data for {}", item_name);
                    return None;
                }

                let data = ItemData {
                    sprite: img_handle,
                    ..data.unwrap()
                };

                Some((item_name.to_owned(), data))
            }
            Err(e) => {
                eprintln!("Error while loading item: {}", e);
                None
            }
        })
        .collect::<Items>();

    commands.insert_resource(items);
}

fn item_spawn_system(
    mut commands: Commands,
    items: Res<Items>,
    query: Query<(Entity, &SpawnItem)>,
) {
    for (entity, spawn_item) in query.iter() {
        if let Some(item_data) = items.get(&spawn_item.item_name) {
            commands
                .spawn_bundle(SpriteBundle {
                    texture: item_data.sprite.clone(),
                    transform: Transform {
                        translation: Vec3::new(spawn_item.position.x, spawn_item.position.y, 0.0),
                        scale: Vec3::new(ITEM_SPRITE_SCALE, ITEM_SPRITE_SCALE, ITEM_SPRITE_SCALE),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::round_cuboid(
                    36.0 * ITEM_SPRITE_SCALE - 5.0,
                    36.0 * ITEM_SPRITE_SCALE - 5.0,
                    5.0,
                ))
                .insert(MassProperties {
                    mass: 2.0,
                    ..Default::default()
                })
                .insert(Velocity::zero())
                .insert(Damping {
                    linear_damping: 0.25,
                    angular_damping: 0.25,
                })
                .insert(Item::default());

            commands.entity(entity).despawn();
        } else {
            eprintln!("Tried to spawn undefined item: {}", spawn_item.item_name);
        }
    }
}
