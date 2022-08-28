use crate::*;

mod cell;
use cell::*;
mod wall;
use rand::{seq::SliceRandom, thread_rng};
use wall::*;
mod room;
use room::*;

use crate::CommonMaterials;

impl Building {
  pub fn setup(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut rng: ResMut<GlobalRng>,
    common_materials: ResMut<CommonMaterials>,
  ) {
    let mut builder = Building::new();
    for _ in 0..20 {
      builder.insert_random_cell(&mut rng);
    }
    builder.build_rooms();
    let cells = builder.finish();

    Self::fabricate(
      cells,
      commands,
      asset_server,
      meshes,
      materials,
      common_materials,
    );
  }

  fn fabricate(
    cells: HashMap<Coord, Cell>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut common_materials: ResMut<CommonMaterials>,
  ) -> Entity {
    let building = Building::default();
    let mut building_cells = building.cells.write();

    for (coord, cell) in cells {
      println!("Fabricating coord: {:?}", coord);

      let cell = cell.finish();

      cell.fabricate(
        &mut commands,
        &mut meshes,
        &mut materials,
        &asset_server,
        &mut common_materials,
      );
      building_cells.insert(coord, cell);
    }

    drop(building_cells);

    commands.spawn().insert(building).id()
  }
}

#[derive(Default, Component)]
pub struct Building {
  outer: Vec<Coord>,
  cells: HashMap<Coord, Arc<Mutex<Cell>>>,
  origin: Vec3,
  building: Arc<Building>,
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

  fn build_rooms(&mut self) {
    for (coord, cell) in &self.cells {
      {
        if cell.borrow().room.is_some() {
          continue;
        }
      }

      ArcRoom::create().fill(coord, self);
    }
  }

  fn finish(&mut self) -> Building {
    let mut cells = HashMap::new();
    for (coord, cell) in self.cells.drain() {
      let cell = Rc::try_unwrap(cell).expect("Something else is holding a ref of this cell.");
      cells.insert(coord, cell.into_inner());
    }

    Building { cells, rooms }
  }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Default)]
pub struct Coord {
  pub x: i16,
  pub z: i16,
}

impl Coord {
  fn adj(&self) -> Vec<Self> {
    let mut adj = Vec::with_capacity(4);
    for (z, x, _) in CARDINAL {
      adj.push(Self {
        z: self.z + z,
        x: self.x + x,
      });
    }
    adj
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
