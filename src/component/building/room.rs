use super::Coord;
use crate::building::Cell;
use crate::*;
use rand::{seq::SliceRandom, thread_rng, Rng};

pub const MAX_SIZE: usize = 8;

static ROOM_COUNT: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug)]
pub struct Room {
  pub id: usize,
  pub cells: RwLock<HashSet<Coord>>,
  pub connected_to: RwLock<HashSet<usize>>,
  size: usize,
}

impl Default for Room {
  fn default() -> Self {
    Self {
      id: ROOM_COUNT.fetch_add(1, Ordering::SeqCst),
      cells: RwLock::default(),
      connected_to: RwLock::default(),
      size: thread_rng().gen_range(0..MAX_SIZE) + 2,
    }
  }
}

impl Room {
  fn cells(&self) -> &RwLock<HashSet<Coord>> {
    &self.cells
  }
  fn len(&self) -> usize {
    self.cells.read().len()
  }
  fn is_empty(&self) -> bool {
    self.cells.read().is_empty()
  }
}

pub type ArcRoom = Arc<Room>;
pub trait ArcRoomExt {
  fn create(building: &mut Building, start_coord: Coord) -> Self;
  fn join_rooms(&self, building: &Building);
}

impl ArcRoomExt for ArcRoom {
  fn join_rooms(&self, building: &Building) {
    let cells = &*self.cells.read();

    for cell in cells {
      for (i, adj_coord) in cell.adj().iter().enumerate() {
        if let Some(adj_cell) = building.cells.get(adj_coord) {
          let mut connected_to = self.connected_to.write();
          let adj_room = adj_cell.room.id;

          if adj_room != self.id && !connected_to.contains(&adj_room) {
            let cell = building.cells.get(cell).unwrap();
            let adj_room = building.rooms.get(&adj_room).unwrap();

            connected_to.insert(adj_room.id);
            adj_room.connected_to.write().insert(self.id);

            cell.create_door(adj_cell, i);
          }
        }
      }
    }
  }

  fn create(building: &mut Building, start_coord: Coord) -> Self {
    let room = Arc::new(Room::default());

    while room.len() < room.size {
      // get empty adj coords
      let mut empty_coords = HashSet::new();
      for c in &*room.cells.read() {
        let mut adj = c.adj();
        building.retain_empty(&mut adj);
        empty_coords.extend(adj);
      }

      let empty_coords: Vec<Coord> = empty_coords.into_iter().collect();

      let coord = match empty_coords.choose(&mut thread_rng()) {
        Some(coord) => *coord,
        None if room.is_empty() => start_coord,
        None => {
          println!("Ran out of open spaces to fill a room with.");
          break;
        }
      };

      println!("New cell at: {:?}, room: {}", coord, room.id);
      Cell::new(coord, room.clone(), building);
    }

    for coord in &*room.cells.read() {
      if let Some(cell) = building.cells.get(coord) {
        cell.collapse_walls(building);
      }
    }

    building.rooms.insert(room.id, room.clone());
    room
  }
}
