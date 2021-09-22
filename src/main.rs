use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

fn main() {
    println!("[ ECS ]");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ControlPlugin)
        .add_startup_system(setup_system)
        //.add_system(print_entity_system)
        //.add_system_to_stage(CoreStage::Update, move_system)
        .run();
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Health(u8);

#[derive(Component, Default, Debug)]
struct Position {
    x: f32,
    y: f32,
}

struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(input_keybord_system);
    }
}

fn input_keybord_system(input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::A) {
        println!("press  KeyCode::A");
    }
}

fn setup_system(mut commands: Commands) {
    println!("[ ECS ] -- < setup >");
    commands.spawn_bundle((Name(format!("Jade")), Health(60), Position::default()));
    commands.spawn_bundle((Name(format!("Judi")), Health(90), Position::default()));
    commands.spawn_bundle((Name(format!("Poke")), Health(12), Position::default()));
}

fn print_entity_system(query: Query<(&Name, &Health, &Position)>) {
    for (name, health, position) in query.iter() {
        println!(
            "name: {}, health: {}, position: {:?}",
            name.0, health.0, position
        );
    }
}

fn move_system(mut query: Query<(&mut Position,)>) {
    for (mut position,) in query.iter_mut() {
        position.x += 2.0;
        position.y += 4.0;
    }
}
