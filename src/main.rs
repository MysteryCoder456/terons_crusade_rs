use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Teron's Crusade".to_owned(),
            width: 1080.,
            height: 720.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    // Add camera bundles
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Green rectangle
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::new(50., 50., 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
