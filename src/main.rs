use bevy::{prelude::*, render::camera::Camera};

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn().insert(Camera::default());
}
