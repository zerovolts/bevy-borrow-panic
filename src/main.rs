use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(velocity_system.system())
        .add_system(acceleration_system.system())
        .add_system(emitter_system.system())
        .run();
}

struct Emitter;
struct Velocity;
struct Acceleration;

fn setup(mut commands: Commands) {
    commands.spawn((Emitter,));
}

fn velocity_system(velocity: &Velocity) { }

fn acceleration_system(mut velocity: Mut<Velocity>) { }

fn emitter_system(mut commands: Commands, emitter: &Emitter) {
    // Spawn projectile
    commands.spawn((Velocity, Acceleration));
}