use bevy::{prelude::*, render::camera::Camera};

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 600.0,
            height: 400.0,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_snake.system()))
        .add_plugins(DefaultPlugins)
        .run();
}

struct SnakeHead;
struct Materials {
    head_material: Handle<ColorMaterial>,
}
fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn().insert(Camera::default());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
    });
}

fn spawn_snake(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(SnakeHead);
}
