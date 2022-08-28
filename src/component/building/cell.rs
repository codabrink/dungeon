use super::{room::ArcRoom, wall, Builder, Building, Coord, Room, Wall};
use crate::*;

#[derive(Debug)]
pub enum Dir {
  N,
  S,
  E,
  W,
}

use rand::{seq::SliceRandom, thread_rng};
use Dir::*;
pub const D: f32 = 30.;
const D2: f32 = D / 2.;
pub const CARDINAL: [(i16, i16, Dir); 4] = [(0, 1, N), (0, -1, S), (1, 0, E), (-1, 0, W)];
const WALL: [[Vec3; 2]; 4] = [
  [Vec3::new(D2, 0., -D2), Vec3::new(D2, 0., D2)],
  [Vec3::new(D2, 0., D2), Vec3::new(-D2, 0., D2)],
  [Vec3::new(-D2, 0., D2), Vec3::new(-D2, 0., -D2)],
  [Vec3::new(-D2, 0., -D2), Vec3::new(D2, 0., -D2)],
];

#[derive(Debug)]
pub struct Cell {
  pub building: Arc<Building>,
  pub room: Option<ArcRoom>,
  pub coord: Coord,
  pub wall_state: [wall::State; 4],
  pub walls: [Option<Entity>; 4],
  pub entity: Option<Entity>,
  origin: Vec3,
}

#[derive(Component)]
pub struct CellComponent {
  cell: Arc<Cell>,
}

impl Cell {
  pub fn new(coord: Coord, builder: &Builder) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      coord,
      origin: builder.origin,
      building: builder.building.clone(),
      wall_state: [wall::State::Solid; 4],
      walls: [None; 4],
      entity: None,
      room: None,
    }))
  }

  pub fn finish(mut self) -> ArcCell {
    for (i, coord) in self.adj().iter().enumerate() {
      match self.building.cells.read().get(coord) {
        Some(cell) => {
          if cell.room.as_ref().unwrap().id == self.room.as_ref().unwrap().id {
            self.wall_state[i] = wall::State::None;
          } else {
            self.wall_state[i] = wall::State::Solid;
          }
        }
        None => {
          self.wall_state[i] = wall::State::Solid;
        }
      }
    }
    Arc::new(self)
  }

  fn adj(&self) -> Vec<Coord> {
    let mut result = Vec::with_capacity(4);
    for (z, x, _) in CARDINAL {
      result.push(Coord {
        z: self.coord.z + z,
        x: self.coord.x + x,
      });
    }
    result
  }

  /// returns a list of adjacent coordinates that are blank
  pub fn adj_empty(&self, builder: &Builder) -> Vec<Coord> {
    self
      .adj()
      .into_iter()
      .filter(|adj| builder.cells.get(adj).is_none())
      .collect()
  }

  pub fn adj_unroomed(&self, builder: &Builder) -> Vec<Coord> {
    self
      .adj()
      .into_iter()
      .filter(|adj| {
        if let Some(cell) = builder.cells.get(adj) {
          return cell.borrow().room.is_none();
        }
        false
      })
      .collect()
  }

  pub fn adj_empty_shuffled(&self, builder: &Builder) -> Vec<Coord> {
    let mut result = self.adj_empty(builder);
    result.shuffle(&mut thread_rng());
    result
  }

  fn fabricate_walls(&self, commands: &mut Commands) {}

  pub fn check_open(list: &mut Vec<Dir>, dir: Dir) {}
}

pub type ArcCell = Arc<Cell>;
pub trait ArcCellExt {
  fn fabricate(
    &self,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    common_materials: &mut ResMut<CommonMaterials>,
  ) -> Entity;
}

impl ArcCellExt for ArcCell {
  fn fabricate(
    &self,
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

    for (i, (z, x, _)) in CARDINAL.iter().enumerate() {}

    for i in 0..4 {
      let w = WALL[i];
      Wall::build(w[0] + translation, w[1] + translation, self.wall_state[i]).fabricate(
        commands,
        meshes,
        materials,
        asset_server,
      );
    }

    let mesh = Mesh::from(shape::Plane { size: D });
    let collider = Collider::cuboid(D / 2., 0.1, D / 2.);

    commands
      .spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material,
        transform,
        ..default()
      })
      .insert(collider)
      .insert(CellComponent { cell: self.clone() })
      .id()
  }
}
