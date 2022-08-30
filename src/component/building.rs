use crate::*;

mod cell;
use cell::*;
mod wall;
use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};
use wall::*;
mod room;
use room::*;

use crate::CommonMaterials;

#[derive(Component, Default)]
pub struct Building {
  cells: HashMap<Coord, Arc<Cell>>,
  rooms: HashMap<usize, Arc<Room>>,
  origin: Vec3,
}

impl Building {
  pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    // mut rng: ResMut<GlobalRng>,
    mut common_materials: ResMut<CommonMaterials>,
  ) {
    let mut building = Building::new();
    for _ in 0..10 {
      building.seed_random_room();
    }

    building.join_rooms();

    let _id = commands
      .spawn_bundle(PbrBundle { ..default() })
      .with_children(|child_builder| {
        // fabricate..
        for (coord, cell) in &building.cells {
          // println!("Fabricating coord: {:?}", coord);

          cell.fabricate(
            child_builder,
            &mut meshes,
            &mut materials,
            &asset_server,
            &mut common_materials,
          );
        }
      })
      .insert(building)
      .id();
  }

  fn new() -> Self {
    let mut building = Self::default();
    building.seed_room(Coord { x: 0, z: 0 });
    building
  }

  pub fn retain_empty(&self, coords: &mut Vec<Coord>) {
    coords.retain(|c| self.cells.get(c).is_none())
  }

  fn seed_room(&mut self, coord: Coord) {
    ArcRoom::create(self, coord);
  }

  fn join_rooms(&mut self) {
    // iterate through rooms
    for room in self.rooms.values() {
      // println!("Joining room: {}", room.id);
      room.join_rooms(self);
    }
  }

  fn seed_random_room(&mut self) {
    let coord = *self
      .outer()
      .choose(&mut thread_rng())
      .expect("There should always be at least one outer coord.");

    self.seed_room(coord);
  }

  fn outer(&self) -> Vec<Coord> {
    let mut outer = HashSet::new();
    for cell in self.cells.values() {
      outer.extend(cell.adj_empty(self));
    }
    outer.into_iter().collect()
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
