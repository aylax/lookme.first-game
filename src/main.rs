use bevy::prelude::*;

fn main() {
    println!("[ ECS ]");
    App::new()
    .add_system(log_system)
    .run();
}


#[derive(Component)]
struct Name;

#[derive(Component)]
struct Health(u8);

// Entity
#[derive(Default)]
struct Human;


fn log_system() {
    trace!("very noisy");
    debug!("helpful for debugging");
    info!("helpful information that is worth printing by default");
    warn!("something bad happened that isn't a failure, but thats worth calling out");
    error!("something failed");
}