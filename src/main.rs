use bevy::prelude::*;

fn main() {
    println!("[ ECS ]");
    App::new().run();
}


#[derive(Component)]
struct Name;

#[derive(Component)]
struct Health(u8);

// Entity
#[derive(Default)]
struct Human;
