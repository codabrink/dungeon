use crate::*;

pub mod cell;
use cell::*;
pub mod wall;
use rand::{seq::SliceRandom, thread_rng, Rng};
pub mod room;
use room::*;

#[derive(Component, Default)]
pub struct Building {
  cells: HashMap<Coord, Arc<Cell>>,
  rooms: HashMap<usize, Arc<Room>>,
  bounds: Option<Rect>,
  origin: Transform,
}

const PROPERTY_WIDTH: f32 = 100.;
const PROPERTY_WIDTH_2: f32 = PROPERTY_WIDTH / 2.;
const PROPERTY_HEIGHT: f32 = PROPERTY_WIDTH;
const PROPERTY_HEIGHT_2: f32 = PROPERTY_HEIGHT / 2.;

impl Building {
  pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
  ) {
    let origin = Transform::default();
    let bounds =
      Rect::build(PROPERTY_WIDTH, PROPERTY_HEIGHT).enter_south_middle_at(&origin.translation);

    bounds.fabricate_debug_walls(&mut commands, &mut meshes, &mut materials);

    Self::fabricate(commands, meshes, materials, ass, origin, Some(bounds));
  }

  fn fabricate(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
    origin: Transform,
    bounds: Option<Rect>,
  ) {
    let mut building = Building::new(origin, bounds);
    for _ in 0..10 {
      building.seed_random_room();
    }

    building.join_rooms();
    building.create_outside_doors();

    let _id = commands
      .spawn_bundle(PbrBundle {
        transform: origin,
        ..default()
      })
      .with_children(|child_builder| {
        for cell in building.cells.values() {
          cell.fabricate(child_builder, &mut meshes, &mut materials, &ass);
        }
      })
      .insert(building)
      .id();

    for _ in 0..5 {
      ENTITIES
        .standing_lamp
        .spawn(Transform::from_xyz(3., 1., 0.), &mut commands, &ass);
      ENTITIES
        .sofa
        .spawn(Transform::from_xyz(3., 1., 0.), &mut commands, &ass);
      ENTITIES
        .fridge
        .spawn(Transform::from_xyz(3., 1., 0.), &mut commands, &ass);
    }
  }

  fn new(origin: Transform, bounds: Option<Rect>) -> Self {
    let mut building = Self {
      origin,
      bounds,
      ..default()
    };
    building.seed_room(Coord { x: 0, z: 0 });
    building
  }

  pub fn retain_empty(&self, coords: &mut Vec<Coord>) {
    coords.retain(|c| self.cells.get(c).is_none())
  }

  fn seed_room(&mut self, coord: Coord) {
    ArcRoom::create(self, coord);
  }

  fn join_rooms(&self) {
    // iterate through rooms
    for room in self.rooms.values() {
      // println!("Joining room: {}", room.id);
      room.join_rooms(self);
    }
  }

  fn create_outside_doors(&self) {
    let mut rng = thread_rng();
    let mut count = [(0, rng.gen_range(2..5)); 4];
    for cell in self.cells.values() {
      cell.create_outside_door(self, &mut count);
    }
  }

  fn seed_random_room(&mut self) {
    let coord = match self.outer().choose(&mut thread_rng()) {
      Some(coord) => *coord,
      _ => return,
    };

    self.seed_room(coord);
  }

  fn coord_to_pos(&self, coord: &Coord) -> Vec3 {
    self.origin.translation + Vec3::new(coord.x as f32 * CELL_SIZE, 0., coord.z as f32 * CELL_SIZE)
  }

  fn outer(&self) -> Vec<Coord> {
    let mut outer = HashSet::new();
    for cell in self.cells.values() {
      outer.extend(cell.adj_empty(self));
    }
    outer
      .into_iter()
      .filter(|coord| {
        self
          .bounds
          .as_ref()
          .map(|b| b.is_inside(self.coord_to_pos(coord)))
          .unwrap_or(true)
      })
      .collect()
  }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Coord {
  pub x: i16,
  pub z: i16,
}

impl Coord {
  pub fn adj(&self) -> Vec<Self> {
    CARDINAL
      .into_iter()
      .map(|(z, x, _)| Self {
        z: self.z + z,
        x: self.x + x,
      })
      .collect()
  }

  fn adj_rand(&self) -> Vec<Self> {
    let mut adj = self.adj();
    adj.shuffle(&mut thread_rng());
    adj
  }
}

impl From<(i16, i16)> for Coord {
  fn from(c: (i16, i16)) -> Self {
    Self { x: c.1, z: c.0 }
  }
}
