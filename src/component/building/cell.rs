use super::wall::{self, Wall};
use super::{Builder, Building, Coord};
use crate::CommonMaterials;
use bevy::prelude::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub enum Dir {
  N,
  S,
  E,
  W,
}

use Dir::*;
pub const D: f32 = 30.;
const D2: f32 = D / 2.;
const CARDINAL: [(i16, i16, Dir); 4] = [(0, 1, N), (0, -1, S), (1, 0, E), (-1, 0, W)];
const WALL: [[Vec3; 2]; 4] = [
  [Vec3::new(D2, 0., -D2), Vec3::new(D2, 0., D2)],
  [Vec3::new(D2, 0., D2), Vec3::new(-D2, 0., D2)],
  [Vec3::new(-D2, 0., D2), Vec3::new(-D2, 0., -D2)],
  [Vec3::new(-D2, 0., -D2), Vec3::new(D2, 0., -D2)],
];

#[derive(Component, Debug, Default)]
pub struct Cell {
  pub coord: Coord,
  origin: Vec3,
  pub wall_state: [wall::State; 4],
  pub walls: [Option<Entity>; 4],
}

impl Cell {
  pub fn new(coord: Coord, building: &Builder) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      coord,
      origin: building.origin,
      wall_state: [
        wall::State::None,
        wall::State::Wall,
        wall::State::Wall,
        wall::State::None,
      ],
      ..default()
    }))
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
    self,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    common_materials: &mut ResMut<CommonMaterials>,
  ) -> Entity {
    let texture = asset_server.load("wood_floor.png");
    let material = materials.add(StandardMaterial {
      base_color_texture: Some(texture),
      alpha_mode: AlphaMode::Blend,
      unlit: false,
      ..default()
    });

    let translation =
      Vec3::new(self.coord.x as f32 * D, 0.2, self.coord.z as f32 * D) + self.origin;
    let transform = Transform::from_translation(translation);

    // println!("Floor transform: {:?}", &transform);

    for i in 0..4 {
      match self.wall_state[i] {
        wall::State::Wall => {
          let w = WALL[i];
          Wall::build(w[0] + translation, w[1] + translation).fabricate(
            commands,
            meshes,
            materials,
            asset_server,
          );
        }
        _ => {}
      }
    }

    commands
      .spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: D })),
        material,
        transform,
        ..default()
      })
      .insert(self)
      .id()
    // .with_children(|builder| {
    //   builder.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Box::new(0.1, D, D))),
    //     // material: wall_texture.clone(),
    //     transform: Transform::from_xyz(-D / 2., D / 2., 0.),
    //     ..default()
    //   });
    // })
    // .with_children(|builder| {
    //   builder.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Box::new(D, D, 0.1))),
    //     // material: wall_texture.clone(),
    //     transform: Transform::from_xyz(0., D / 2., D / 2.),
    //     ..default()
    //   });
    // });
  }

  fn fabricate_walls(&self, commands: &mut Commands) {}

  pub fn check_open(list: &mut Vec<Dir>, dir: Dir) {}
}
