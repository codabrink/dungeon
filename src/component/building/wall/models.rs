use super::super::D;
use super::{State, Wall};
use bevy::prelude::*;

pub fn model(
  wall: &Wall,
  commands: &mut Commands,
  meshes: &mut ResMut<Assets<Mesh>>,
  materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Option<Mesh> {
  let m = Mesh::from(shape::Box::new(W, D, wall.len));

  match wall.state {
    State::None => None,
    State::Wall => Some(Mesh::from(shape::Box::new(W, D, wall.len))),
    // State::Door => Mesh::
    _ => unreachable!(),
  }
}
