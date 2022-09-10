use super::Coord;
use crate::building::Cell;
use crate::*;
use crossbeam_channel::bounded;
use rand::{distributions::Standard, prelude::Distribution, seq::SliceRandom, thread_rng, Rng};

pub const MAX_SIZE: usize = 8;
static ROOM_COUNT: AtomicUsize = AtomicUsize::new(1);

type FloorMatHash = HashMap<RoomType, Handle<StandardMaterial>>;
lazy_static! {
  static ref FMC: (Sender<FloorMatHash>, Receiver<FloorMatHash>) = bounded(1);
  static ref FLOOR_MAT: FloorMatHash = FMC.1.recv().unwrap();
}

#[derive(Debug)]
pub struct Room {
  pub id: usize,
  pub cells: RwLock<HashSet<Coord>>,
  pub connected_to: RwLock<HashSet<usize>>,
  size: usize,
  r#type: RoomType,
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum RoomType {
  Bedroom,
  Kitchen,
}

impl Distribution<RoomType> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RoomType {
    match rng.gen_range(0..=1) {
      0 => RoomType::Bedroom,
      _ => RoomType::Kitchen,
    }
  }
}

impl Default for Room {
  fn default() -> Self {
    Self {
      id: ROOM_COUNT.fetch_add(1, Ordering::SeqCst),
      cells: RwLock::default(),
      connected_to: RwLock::default(),
      size: thread_rng().gen_range(0..MAX_SIZE) + 2,
      r#type: rand::random(),
    }
  }
}

impl Room {
  fn len(&self) -> usize {
    self.cells.read().len()
  }
  fn is_empty(&self) -> bool {
    self.cells.read().is_empty()
  }

  pub fn floor_mat(&self) -> Handle<StandardMaterial> {
    FLOOR_MAT.get(&self.r#type).unwrap().clone()
  }

  pub fn load_materials(ass: Res<AssetServer>, mut mat: ResMut<Assets<StandardMaterial>>) {
    let mut floor_mat = HashMap::new();
    let texture = ass.load("room/floor/wood_floor_2x.jpg");
    floor_mat.insert(
      RoomType::Bedroom,
      mat.add(StandardMaterial {
        base_color_texture: Some(texture),
        alpha_mode: AlphaMode::Blend,
        ..default()
      }),
    );
    let texture = ass.load("room/floor/kitchen.jpg");
    floor_mat.insert(
      RoomType::Kitchen,
      mat.add(StandardMaterial {
        base_color_texture: Some(texture),
        alpha_mode: AlphaMode::Blend,
        ..default()
      }),
    );

    let _ = FMC.0.send(floor_mat);
  }
}

pub type ArcRoom = Arc<Room>;
pub trait ArcRoomExt {
  fn create(building: &mut Building, arc_building: &Arc<Building>, start_coord: Coord) -> Self;
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

  fn create(building: &mut Building, arc_building: &Arc<Building>, start_coord: Coord) -> Self {
    let room = Arc::new(Room::default());

    while room.len() < room.size {
      // get empty adj coords
      let mut empty_coords = HashSet::new();
      for c in &*room.cells.read() {
        let mut adj = c.adj();
        building.retain_empty_and_valid(&mut adj);
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

      Cell::new(coord, room.clone(), building, arc_building);
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
