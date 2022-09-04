use crate::*;
use rand::{thread_rng, Rng};
use Dir::*;

#[derive(Debug)]
pub enum Dir {
  N,
  S,
  E,
  W,
}

pub const CELL_SIZE: f32 = 30.;
pub const CELL_SIZE_2: f32 = CELL_SIZE / 2.;
pub const CARDINAL: [(i16, i16, Dir); 4] = [(0, 1, N), (1, 0, E), (0, -1, S), (-1, 0, W)];
const WALL: [[Vec3; 2]; 4] = [
  [
    Vec3::new(CELL_SIZE_2, 0., -CELL_SIZE_2),
    Vec3::new(CELL_SIZE_2, 0., CELL_SIZE_2),
  ],
  [
    Vec3::new(CELL_SIZE_2, 0., CELL_SIZE_2),
    Vec3::new(-CELL_SIZE_2, 0., CELL_SIZE_2),
  ],
  [
    Vec3::new(-CELL_SIZE_2, 0., CELL_SIZE_2),
    Vec3::new(-CELL_SIZE_2, 0., -CELL_SIZE_2),
  ],
  [
    Vec3::new(-CELL_SIZE_2, 0., -CELL_SIZE_2),
    Vec3::new(CELL_SIZE_2, 0., -CELL_SIZE_2),
  ],
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

  pub fn collapse_walls(&self, building: &Building) {
    let mut wall_state = self.wall_state.write();
    for (i, coord) in self.adj().iter().enumerate() {
      wall_state[i] = match building.cells.get(coord) {
        Some(cell) if cell.room.id == self.room.id => wall::State::None,
        Some(_) if i >= 1 && i <= 2 => wall::State::Solid,
        None => wall::State::Solid,
        _ => wall::State::None,
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

  pub fn create_door(&self, other: &Cell, cardinal_dir: usize) {
    self.wall_state.write()[cardinal_dir] = wall::State::Door;
    other.wall_state.write()[(cardinal_dir + 2) % 4] = wall::State::Door;
  }

  pub fn create_outside_door(&self, building: &Building, count: &mut [(u8, u8); 4]) {
    for (i, coord) in self.adj().iter().enumerate() {
      if building.cells.get(coord).is_some() {
        continue;
      }

      if count[i].0 < count[i].1 {
        self.wall_state.write()[i] = wall::State::Door;
        count[i].0 += 1;
        return;
      }
    }
  }

  /// returns a list of adjacent coordinates that are blank
  pub fn adj_empty(&self, building: &Building) -> Vec<Coord> {
    self
      .adj()
      .into_iter()
      .filter(|adj| building.cells.get(adj).is_none())
      .collect()
  }
}

pub type ArcCell = Arc<Cell>;
pub trait ArcCellExt {
  fn fabricate(
    &self,
    child_builder: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
  ) -> Entity;
}

impl ArcCellExt for ArcCell {
  fn fabricate(
    &self,
    child_builder: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
  ) -> Entity {
    let texture = asset_server.load("wood_floor.jpg");
    let material = materials.add(StandardMaterial {
      base_color_texture: Some(texture),
      alpha_mode: AlphaMode::Blend,
      unlit: false,
      ..default()
    });

    let translation = Vec3::new(
      self.coord.x as f32 * CELL_SIZE,
      0.2,
      self.coord.z as f32 * CELL_SIZE,
    );
    let transform = Transform::from_translation(translation);

    // println!("Floor transform: {:?}", &transform);

    let mesh = Mesh::from(shape::Plane { size: CELL_SIZE });
    let collider = Collider::cuboid(CELL_SIZE / 2., 0.1, CELL_SIZE / 2.);

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
          Wall::build(w[0], w[1], wall_state[i]).fabricate_as_child(
            child_builder,
            meshes,
            materials,
          );
        }
      })
      .id()
  }
}
