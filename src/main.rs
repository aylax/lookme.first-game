use bevy::prelude::*;

fn main() {
    println!("[ ECS ]");
    App::new()
        .add_startup_system(setup_system)
        .add_system(print_system)
        .run();
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Health(u8);

#[derive(Component)]
struct Position;

fn setup_system(mut commands: Commands) {
    println!("[ ECS ] -- < setup >");
    commands.spawn_bundle((Name(format!("Jade")), Health(60)));
    commands.spawn_bundle((Name(format!("Judi")), Health(90)));
    commands.spawn_bundle((Name(format!("None")), Health(40)));
}

fn print_system(query: Query<(&Name, &Health)>) {
    for (name, health) in query.iter() {
        println!("name: {}, health: {}", name.0, health.0);
    }
}
