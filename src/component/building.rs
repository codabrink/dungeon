use bevy::prelude::*;
use bevy_turborand::*;
use rand::seq::SliceRandom;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;

mod cell;
use cell::*;

type Coord = (i16, i16);

#[derive(Component)]
pub struct Building {
  cells: HashMap<Coord, Cell>,
}

impl Building {
  pub fn setup(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<GlobalRng>,
  ) {
    let mut builder = Builder::new();
    for _ in 0..10 {
      builder.insert_random_cell(&mut rng);
    }
    let mut building = Building {
      cells: builder.finish(),
    };
    building.fabricate(commands, meshes, materials);
  }

  fn fabricate(
    &mut self,
    commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
  ) {
    for (coord, cell) in &self.cells {}
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
    builder.new_cell((0, 0));
    builder
  }

  fn new_cell(&mut self, coords: Coord) {
    let cell = Cell::new((0, 0), self);
    self.cells.insert((0, 0), cell);
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
      outer.extend(empty_adj);
    }
    self.outer = outer.into_iter().collect();
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
