use super::{Builder, Coord};
use bevy::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum Dir {
  N,
  S,
  E,
  W,
}

use Dir::*;
const CARDINAL: [(i16, i16, Dir); 4] = [(0, 1, N), (0, -1, S), (1, 0, E), (-1, 0, W)];
pub const SIZE: f32 = 30.;

#[derive(Component, Debug)]
pub struct Cell {
  coord: Coord,
}

impl Cell {
  pub fn new(coord: Coord, builder: &Builder) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self { coord }))
  }

  /// returns a list of adjacent coordinates that are blank
  pub fn adj_empty(&self, builder: &Builder) -> Vec<Coord> {
    let mut empty = Vec::new();
    for (z, x, _) in CARDINAL {
      let coord = Coord {
        z: self.coord.z + z,
        x: self.coord.x + x,
      };
      if builder.cells.get(&coord).is_none() {
        empty.push(coord);
      }
    }
    empty
  }

  pub fn fabricate(
    &self,
    commands: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
  ) {
    let texture = asset_server.load("wood_floor.png");
    let material = materials.add(StandardMaterial {
      base_color_texture: Some(texture),
      alpha_mode: AlphaMode::Blend,
      unlit: false,
      ..default()
    });

    let transform =
      Transform::from_xyz(self.coord.x as f32 * SIZE, 0.2, self.coord.z as f32 * SIZE);

    commands.spawn_bundle(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Plane { size: SIZE })),
      material,
      transform,
      ..default()
    });
    // .insert(Self);
  }

  pub fn check_open(list: &mut Vec<Dir>, dir: Dir) {}
}
