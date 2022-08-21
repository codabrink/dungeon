use bevy::prelude::*;
use bevy_turborand::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

mod cell;
use cell::*;

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Coord {
  pub x: i16,
  pub z: i16,
}

impl From<(i16, i16)> for Coord {
  fn from(c: (i16, i16)) -> Self {
    Self { x: c.1, z: c.0 }
  }
}

#[derive(Component)]
pub struct Building {
  cells: HashMap<Coord, Cell>,
}

impl Building {
  pub fn setup(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut rng: ResMut<GlobalRng>,
  ) {
    let mut builder = Builder::new();
    for _ in 0..10 {
      builder.insert_random_cell(&mut rng);
    }
    let mut building = Building {
      cells: builder.finish(),
    };
    building.fabricate(commands, asset_server, meshes, materials);
  }

  fn fabricate(
    &mut self,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
  ) {
    commands
      .spawn()
      .insert(Transform::from_xyz(0., 0., 0.))
      .with_children(|b| {
        for (coord, cell) in &self.cells {
          println!("Fabricating coord: {:?}", coord);
          cell.fabricate(b, &mut meshes, &mut materials, &asset_server);
        }
      });
  }
}

#[derive(Default)]
pub struct Builder {
  outer: Vec<Coord>,
  cells: HashMap<Coord, Rc<RefCell<Cell>>>,
}
impl Builder {
  fn new() -> Self {
    let mut builder = Self::default();
    builder.new_cell(Coord { x: 0, z: 0 });
    builder
  }

  fn new_cell(&mut self, coord: Coord) {
    let cell = Cell::new(coord, self);
    self.cells.insert(coord, cell);
    self.rebuild_meta();
  }

  fn insert_random_cell(&mut self, rng: &mut ResMut<GlobalRng>) {
    let i = rng.usize(0..self.outer.len());
    let coord = self
      .outer
      .get(i)
      .expect("There should always be at least one outer coord.");
    self.new_cell(*coord);
  }

  fn rebuild_meta(&mut self) {
    let mut outer = HashSet::new();
    for (_, cell) in &self.cells {
      let empty_adj = (**cell).borrow().adj_empty(self);
      // println!("Emtpy adj to {:?}: {:?}", coord, empty_adj);
      outer.extend(empty_adj);
      // println!("Outer hashset: {:?}", outer);
    }
    self.outer = outer.into_iter().collect();
    // println!("Outer vec: {:?}", self.outer);
    // println!("============Rebuilt============");
  }

  fn finish(&mut self) -> HashMap<Coord, Cell> {
    let mut output = HashMap::new();
    for (coord, cell) in self.cells.drain() {
      let cell = Rc::try_unwrap(cell).expect("Something else is holding a ref of this cell.");
      output.insert(coord, cell.into_inner());
    }
    output
  }
}
