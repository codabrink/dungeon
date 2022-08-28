use rand::{seq::SliceRandom, thread_rng, Rng};

use super::{Builder, Coord};
use crate::*;

pub const MAX_SIZE: usize = 7;

static RoomCount: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct Room {
  pub id: usize,
  cells: Mutex<HashSet<Coord>>,
  size: usize,
}

impl Default for Room {
  fn default() -> Self {
    Self {
      id: RoomCount.fetch_add(1, Ordering::SeqCst),
      ..default()
    }
  }
}

impl Room {
  fn cells(&self) -> &Mutex<HashSet<Coord>> {
    &self.cells
  }
  fn len(&self) -> usize {
    self.cells.lock().len()
  }
}

pub type ArcRoom = Arc<Room>;
pub trait ArcRoomExt {
  fn create() -> Self;
  fn fill(&self, start_coord: &Coord, builder: &Builder);
  fn add_cell(&self, coord: &Coord, builder: &Builder) -> bool;
  fn adj_unroomed_cells(&self, builder: &Builder) -> Vec<Coord>;
  fn random_adj_unroomed_cell(&self, builder: &Builder) -> Option<Coord>;
}

impl ArcRoomExt for ArcRoom {
  fn create() -> Self {
    Arc::new(Room {
      size: thread_rng().gen_range(0..MAX_SIZE),
      ..default()
    })
  }

  fn add_cell(&self, coord: &Coord, builder: &Builder) -> bool {
    let mut cell = match builder.cells.get(coord) {
      Some(cell) => cell.borrow_mut(),
      _ => return false,
    };
    cell.room = Some(self.clone());
    self.cells.lock().insert(cell.coord);
    true
  }

  fn fill(&self, start_coord: &Coord, builder: &Builder) {
    if !self.add_cell(start_coord, builder) {
      return;
    }

    while let Some(coord) = self.random_adj_unroomed_cell(builder) {
      self.add_cell(&coord, builder);

      if self.len() >= self.size {
        break;
      }
    }
  }

  fn adj_unroomed_cells(&self, builder: &Builder) -> Vec<Coord> {
    let mut adj = HashSet::new();
    for coord in &*self.cells.lock() {
      let cell = builder.cells.get(coord);
      if let Some(cell) = cell {
        for coord in cell.borrow().adj_unroomed(builder) {
          adj.insert(coord);
        }
      }
    }
    adj.drain().collect()
  }

  fn random_adj_unroomed_cell(&self, builder: &Builder) -> Option<Coord> {
    let mut coords = self.adj_unroomed_cells(builder);
    coords.shuffle(&mut thread_rng());
    coords.first().map(|c| *c)
  }
}
