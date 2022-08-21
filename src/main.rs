use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_turborand::*;

mod component;

fn main() {
  App::new()
    .add_plugin(RngPlugin::default())
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_startup_system(component::Player::setup)
    .add_startup_system(component::Camera::setup)
    .add_startup_system(component::Grass::setup)
    .add_startup_system(component::Building::setup)
    .add_system(component::Camera::follow_player)
    .add_system(component::Player::update)
    .run();
}
