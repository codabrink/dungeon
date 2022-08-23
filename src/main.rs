pub use bevy::prelude::*;
pub use bevy_rapier3d::prelude::*;
use bevy_turborand::*;
pub use parking_lot::Mutex;
pub use std::{
  cell::RefCell,
  collections::{HashMap, HashSet},
  rc::Rc,
  sync::Arc,
};

mod component;

fn main() {
  App::new()
    .insert_resource(CommonMaterials::default())
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

pub type CommonMaterials = HashMap<String, Handle<StandardMaterial>>;
