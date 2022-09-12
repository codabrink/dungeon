#![feature(let_chains)]

pub use bevy::ecs::system::EntityCommands;
pub use bevy::prelude::*;
use bevy::{
  diagnostic::FrameTimeDiagnosticsPlugin,
  render::{RenderApp, RenderStage},
};
pub use bevy_rapier3d::prelude::*;
use bevy_turborand::*;
pub use component::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
pub use lazy_static::lazy_static;
pub use parking_lot::{Mutex, RwLock};
use rand::Rng;
pub use std::{
  cell::RefCell,
  collections::{HashMap, HashSet},
  rc::Rc,
  sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
  },
  time::{Duration, Instant},
};
pub use system::*;

mod component;
mod system;

fn main() {
  let mut app = App::new();

  app
    .insert_resource(CommonMaterials::default())
    .insert_resource(Zones::default())
    .insert_resource(road::RoadGrid::default())
    .add_plugin(RngPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(MaterialPlugin::<ZombieMaterial>::default())
    // .add_plugin(RapierDebugRenderPlugin::default())
    .add_startup_system_to_stage(
      StartupStage::PreStartup,
      component::building::Room::load_materials,
    )
    .add_startup_system(component::Player::setup)
    .add_startup_system(component::Camera::setup)
    .add_startup_system(component::Grass::setup)
    .add_startup_system(component::Building::spawn)
    .add_startup_system(component::DebugText::spawn)
    .add_system(component::Camera::follow_player)
    .add_system(component::Player::update)
    .add_system(component::Zombie::update_normal)
    .add_system(component::Zombie::update_aggressive)
    .add_system(component::Zombie::update_impact)
    .add_system(component::Bullet::spawn)
    .add_system(component::Bullet::update)
    .add_system(Zones::update)
    .add_system(road::RoadGrid::update)
    .add_system(component::DebugText::update);

  app
    .sub_app_mut(RenderApp)
    .add_system_to_stage(RenderStage::Extract, Zombie::extract_health)
    .add_system_to_stage(RenderStage::Prepare, Zombie::prepare_health);

  app.run();
}

pub type CommonMaterials = HashMap<String, Handle<StandardMaterial>>;
