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
      let coord = (self.coord.0 + z, self.coord.1 + x);
      if let None = builder.cells.get(&coord) {
        empty.push(coord);
      }
    }
    empty
  }
  pub fn check_open(list: &mut Vec<Dir>, dir: Dir) {}
}
