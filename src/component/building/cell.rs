use super::{wall, Building, Coord, Room, Wall};
use crate::*;
use rand::{seq::SliceRandom, thread_rng};
use Dir::*;

#[derive(Debug)]
pub enum Dir {
  N,
  S,
  E,
  W,
}

pub const D: f32 = 30.;
const D2: f32 = D / 2.;
pub const CARDINAL: [(i16, i16, Dir); 4] = [(0, 1, N), (1, 0, E), (0, -1, S), (-1, 0, W)];
const WALL: [[Vec3; 2]; 4] = [
  [Vec3::new(D2, 0., -D2), Vec3::new(D2, 0., D2)],
  [Vec3::new(D2, 0., D2), Vec3::new(-D2, 0., D2)],
  [Vec3::new(-D2, 0., D2), Vec3::new(-D2, 0., -D2)],
  [Vec3::new(-D2, 0., -D2), Vec3::new(D2, 0., -D2)],
];

#[derive(Debug)]
pub struct Cell {
  pub room: Arc<Room>,
  pub coord: Coord,
  pub wall_state: RwLock<[wall::State; 4]>,
  pub walls: RwLock<[Option<Entity>; 4]>,
}
#[derive(Component)]
pub struct CellComponent {
  cell: ArcCell,
}

impl Cell {
  pub fn new(coord: Coord, room: Arc<Room>, building: &mut Building) -> Arc<Self> {
    let cell = Arc::new(Self {
      coord,
      room,
      wall_state: RwLock::new([wall::State::Solid; 4]),
      walls: RwLock::new([None; 4]),
    });

    building.cells.insert(coord, cell.clone());
    cell.room.cells.write().insert(coord);
    cell
  }

  pub fn finish(&self, building: &Building) {
    for (i, coord) in self.adj().iter().enumerate() {
      self.wall_state.write()[i] = match building.cells.get(coord) {
        Some(cell) if cell.room.id == self.room.id => wall::State::None,
        Some(cell) => wall::State::Solid,
        None => wall::State::Solid,
      }
    }
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
  pub fn adj_empty(&self, building: &Building) -> Vec<Coord> {
    self
      .adj()
      .into_iter()
      .filter(|adj| building.cells.get(adj).is_none())
      .collect()
  }

  pub fn adj_empty_shuffled(&self, building: &Building) -> Vec<Coord> {
    let mut result = self.adj_empty(building);
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
    child_builder: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    common_materials: &mut ResMut<CommonMaterials>,
  ) -> Entity;
}

impl ArcCellExt for ArcCell {
  fn fabricate(
    &self,
    child_builder: &mut ChildBuilder,
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

    let translation = Vec3::new(self.coord.x as f32 * D, 0.2, self.coord.z as f32 * D);
    let transform = Transform::from_translation(translation);

    // println!("Floor transform: {:?}", &transform);

    let mesh = Mesh::from(shape::Plane { size: D });
    let collider = Collider::cuboid(D / 2., 0.1, D / 2.);

    child_builder
      .spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material,
        transform,
        ..default()
      })
      .insert(collider)
      .insert(CellComponent { cell: self.clone() })
      .with_children(|child_builder| {
        let wall_state = self.wall_state.read();
        for i in 0..4 {
          let w = WALL[i];
          Wall::build(w[0], w[1], wall_state[i]).fabricate(
            child_builder,
            meshes,
            materials,
            asset_server,
          );
        }
      })
      .id()
  }
}
