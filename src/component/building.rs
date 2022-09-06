use std::hash::{Hash, Hasher};

use crate::*;

pub mod cell;
use cell::*;
pub mod wall;
use rand::{seq::SliceRandom, thread_rng, Rng};
pub mod room;
use room::*;

static BUILDING_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Default)]
pub struct Building {
  id: usize,
  pub cells: HashMap<Coord, Arc<Cell>>,
  rooms: HashMap<usize, Arc<Room>>,
  bounds: Option<Rect>,
  pub origin: Transform,
  pub navigated: AtomicBool,
}

#[derive(Component)]
pub struct BuildingComponent {
  pub building: Arc<Building>,
}

const PROPERTY_WIDTH: f32 = 200.;
const PROPERTY_WIDTH_2: f32 = PROPERTY_WIDTH / 2.;
const PROPERTY_HEIGHT: f32 = PROPERTY_WIDTH;
const PROPERTY_HEIGHT_2: f32 = PROPERTY_HEIGHT / 2.;

impl Building {
  pub fn spawn(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
  ) {
    let origin = Transform::from_xyz(0., 0.1, 0.);
    let bounds =
      Rect::build(PROPERTY_WIDTH, PROPERTY_HEIGHT).enter_south_middle_at(&origin.translation);

    Self::fabricate(commands, meshes, materials, ass, origin, Some(bounds));
  }

  fn fabricate(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
    origin: Transform,
    bounds: Option<Rect>,
  ) -> Entity {
    let mut building = Building::new(origin, bounds);
    for _ in 0..10 {
      building.seed_random_room();
    }

    building.join_rooms();
    building.create_outside_doors();
    building.gen_navigation();
    building.spawn_zombies(&mut commands, &mut meshes, &mut materials);

    let building = Arc::new(building);
    let building_component = BuildingComponent {
      building: building.clone(),
    };

    let _ = ZONE_TX.send(ZItem::Building(building.clone()));

    // DEBUG
    building.fabricate_nav(&mut commands, &mut meshes, &mut materials);
    // DEBUG
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

    commands
      .spawn_bundle(PbrBundle {
        transform: origin,
        ..default()
      })
      .with_children(|child_builder| {
        for cell in building.cells.values() {
          cell.fabricate(&building, child_builder, &mut meshes, &mut materials, &ass);
        }
      })
      .insert(building_component)
      .id()
  }

  fn new(origin: Transform, bounds: Option<Rect>) -> Self {
    let mut building = Self {
      id: BUILDING_ID.fetch_add(1, Ordering::SeqCst),
      origin,
      bounds,
      ..default()
    };
    building.seed_room(Coord { x: 0, z: 0 });
    building
  }

  fn fabricate_nav(
    &self,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
  ) {
    for cell in self.cells.values() {
      cell.fabricate_nav(commands, meshes, materials);
    }
  }

  pub fn retain_empty_and_valid(&self, coords: &mut Vec<Coord>) {
    coords.retain(|c| {
      self.cells.get(c).is_none()
        && self
          .bounds
          .as_ref()
          .map(|b| b.contains(self.coord_to_pos_global(c)))
          .unwrap_or(true)
    })
  }

  fn seed_room(&mut self, coord: Coord) {
    ArcRoom::create(self, coord);
  }

  fn join_rooms(&self) {
    // iterate through rooms
    for room in self.rooms.values() {
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

  fn gen_navigation(&self) {
    for cell in self.cells.values() {
      cell.gen_navigation(self);
    }
  }

  fn spawn_zombies(
    &self,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
  ) {
    for cell in self.cells.values() {
      Zombie::fabricate(cell.random_pos(), commands, meshes, materials);
    }
  }

  fn seed_random_room(&mut self) {
    let coord = match self.outer().choose(&mut thread_rng()) {
      Some(coord) => *coord,
      _ => return,
    };

    self.seed_room(coord);
  }

  pub fn coord_to_pos_rel(&self, coord: &Coord) -> Vec3 {
    Vec3::new(coord.x as f32 * CELL_SIZE, 0., coord.z as f32 * CELL_SIZE)
  }

  pub fn coord_to_pos_global(&self, coord: &Coord) -> Vec3 {
    self.origin.translation + self.coord_to_pos_rel(coord)
  }

  pub fn pos_global_to_coord(&self, pos: Vec3) -> Coord {
    let pos = pos - self.origin.translation;
    Coord {
      z: ((pos.z + CELL_SIZE_2) / CELL_SIZE) as i16,
      x: ((pos.x + CELL_SIZE_2) / CELL_SIZE) as i16,
    }
  }

  pub fn pos_global_to_cell(&self, pos: Vec3) -> Option<&Arc<Cell>> {
    self.cells.get(&self.pos_global_to_coord(pos))
  }

  fn outer(&self) -> Vec<Coord> {
    let mut outer = HashSet::new();
    for cell in self.cells.values() {
      for coord in cell.adj_empty(self) {
        if let Some(bounds) = &self.bounds {
          if !bounds.contains(self.coord_to_pos_global(&coord)) {
            continue;
          }
          outer.insert(coord);
        }
      }
    }
    outer.into_iter().collect()
  }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Copy, Clone)]
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

impl Hash for Building {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
  }
}
impl PartialEq for Building {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
  fn ne(&self, other: &Self) -> bool {
    self.id != other.id
  }
}
impl Eq for Building {}
