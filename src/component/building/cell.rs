use crate::{nav::NavNodeComponent, *};
use Dir::*;

#[derive(Debug)]
pub enum Dir {
  N,
  S,
  E,
  W,
}

pub const CELL_SIZE: f32 = 30.;
pub const CELL_SIZE_2: f32 = CELL_SIZE / 2.;
pub const CARDINAL: [(i16, i16, Dir); 4] = [(0, 1, N), (1, 0, E), (0, -1, S), (-1, 0, W)];
const WALL: [[Vec3; 2]; 4] = [
  [
    Vec3::new(CELL_SIZE_2, 0., -CELL_SIZE_2),
    Vec3::new(CELL_SIZE_2, 0., CELL_SIZE_2),
  ],
  [
    Vec3::new(CELL_SIZE_2, 0., CELL_SIZE_2),
    Vec3::new(-CELL_SIZE_2, 0., CELL_SIZE_2),
  ],
  [
    Vec3::new(-CELL_SIZE_2, 0., CELL_SIZE_2),
    Vec3::new(-CELL_SIZE_2, 0., -CELL_SIZE_2),
  ],
  [
    Vec3::new(-CELL_SIZE_2, 0., -CELL_SIZE_2),
    Vec3::new(CELL_SIZE_2, 0., -CELL_SIZE_2),
  ],
];
const WALL_NAV: [Vec3; 4] = [
  Vec3::new(CELL_SIZE_2, 0., 0.),
  Vec3::new(0., 0., CELL_SIZE_2),
  Vec3::new(-CELL_SIZE_2, 0., 0.),
  Vec3::new(0., 0., -CELL_SIZE_2),
];

#[derive(Debug)]
pub struct Cell {
  pub room: Arc<Room>,
  pub coord: Coord,
  pub wall_state: RwLock<[wall::State; 4]>,
  pub walls: RwLock<[Option<Entity>; 4]>,
  // 0-3: doors, 4: self, 5: outside
  pub nav_nodes: RwLock<[Option<Arc<NavNode>>; 6]>,
  pub pos: Vec3,
}
#[derive(Component)]
pub struct CellComponent {
  cell: ArcCell,
}

impl Cell {
  pub fn new(coord: Coord, room: Arc<Room>, building: &mut Building) -> Arc<Self> {
    let cell = Arc::new(Self {
      coord,
      room,
      wall_state: RwLock::default(),
      walls: RwLock::default(),
      nav_nodes: RwLock::default(),
      pos: building.coord_to_pos_global(&coord),
    });

    building.cells.insert(coord, cell.clone());
    cell.room.cells.write().insert(coord);
    cell
  }

  pub fn collapse_walls(&self, building: &Building) {
    let mut wall_state = self.wall_state.write();
    for (i, coord) in self.adj().iter().enumerate() {
      wall_state[i] = match building.cells.get(coord) {
        Some(cell) if cell.room.id == self.room.id => wall::State::None,
        Some(_) if i >= 1 && i <= 2 => wall::State::Solid,
        None => wall::State::Solid,
        _ => wall::State::None,
      }
    }
  }

  fn adj(&self) -> [Coord; 4] {
    let mut result = [Coord::default(); 4];
    for (i, (z, x, _)) in CARDINAL.iter().enumerate() {
      result[i] = Coord {
        z: self.coord.z + z,
        x: self.coord.x + x,
      };
    }
    result
  }

  pub fn create_door(&self, other: &Cell, cardinal_dir: usize) {
    self.wall_state.write()[cardinal_dir] = wall::State::Door;
    other.wall_state.write()[(cardinal_dir + 2) % 4] = wall::State::Door;
  }

  pub fn create_outside_door(&self, building: &Building, count: &mut [(u8, u8); 4]) {
    for (i, coord) in self.adj().iter().enumerate() {
      if building.cells.get(coord).is_some() {
        continue;
      }

      if count[i].0 < count[i].1 {
        self.wall_state.write()[i] = wall::State::Door;
        count[i].0 += 1;
        return;
      }
    }
  }

  pub fn fabricate_nav(
    &self,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
  ) {
    for nav_node in &*self.nav_nodes.read() {
      if let Some(nav_node) = nav_node {
        NavNodeComponent {
          node: nav_node.clone(),
        }
        .fabricate(commands, meshes, materials);
      }
    }
  }

  /// returns a list of adjacent coordinates that are blank
  pub fn adj_empty(&self, building: &Building) -> Vec<Coord> {
    self
      .adj()
      .into_iter()
      .filter(|adj| building.cells.get(adj).is_none())
      .collect()
  }
}

pub type ArcCell = Arc<Cell>;
pub trait ArcCellExt {
  fn gen_navigation(&self, building: &Building);
  fn fabricate(
    &self,
    building: &Building,
    child_builder: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
  ) -> Entity;
}

impl ArcCellExt for ArcCell {
  fn gen_navigation(&self, building: &Building) {
    let adj = self.adj();
    let wall_state = self.wall_state.read();

    let pos = building.coord_to_pos_global(&self.coord);
    let area = Rect::build(CELL_SIZE, CELL_SIZE).center_at(&pos);
    let mut nav_nodes = self.nav_nodes.write();
    let cell_nav = NavNode::new(pos, NavNodeType::Cell, area, HashSet::new());

    for (i, state) in wall_state.iter().enumerate() {
      let adj_cell = building.cells.get(&adj[i]);

      match state {
        wall::State::Door => {
          let pos = pos + WALL_NAV[i];
          let area = Rect::build(1., 1.).center_at(&pos);
          let door_nav = NavNode::new(
            pos,
            NavNodeType::Door,
            area,
            HashSet::from([cell_nav.clone()]), // link the cell to the door
          );

          cell_nav.adj.write().insert(door_nav.clone()); // link the door back to the cell

          // outside...
          if adj_cell.is_none() {
            let pos = pos + (WALL_NAV[i] * 2.);
            let area = Rect::build(1., 1.).center_at(&pos);
            let outside_nav = NavNode::new(
              pos,
              NavNodeType::Outside,
              area,
              HashSet::from([door_nav.clone()]),
            );

            // link the outside back to the door
            door_nav.adj.write().insert(door_nav.clone());

            nav_nodes[5] = Some(outside_nav.clone());
            let _ = ZONE_TX.send(ZItem::Nav(outside_nav));
          }

          // TODO: This may not be needed due to the next match condition below?
          // if let Some(adj_cell) = adj_cell {
          // if let Some(node) = &adj_cell.nav_nodes.read()[4] {
          // node.adj.write().insert(door_nav.clone()); // link adj cell to door
          // }
          // }

          nav_nodes[i] = Some(door_nav.clone());
          let _ = ZONE_TX.send(ZItem::Nav(door_nav));
        }
        wall::State::None => {
          // if there is a cell in this direction...
          if let Some(adj_cell) = adj_cell {
            // let the adj cell's door know that this cell now exists
            if let Some(door_nav) = &adj_cell.nav_nodes.read()[i.opposite()] {
              door_nav.adj.write().insert(cell_nav.clone());
            }
          }
        }
        _ => {}
      }
    }

    nav_nodes[4] = Some(cell_nav.clone());
    let _ = ZONE_TX.send(ZItem::Nav(cell_nav));
  }

  fn fabricate(
    &self,
    building: &Building,
    child_builder: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
  ) -> Entity {
    let texture = asset_server.load("wood_floor.jpg");
    let material = materials.add(StandardMaterial {
      base_color_texture: Some(texture),
      alpha_mode: AlphaMode::Blend,
      unlit: false,
      ..default()
    });

    let translation = building.coord_to_pos_rel(&self.coord);
    let transform = Transform::from_translation(translation);

    // println!("Floor transform: {:?}", &transform);

    let mesh = Mesh::from(shape::Plane { size: CELL_SIZE });
    let collider = Collider::cuboid(CELL_SIZE / 2., 0.1, CELL_SIZE / 2.);

    child_builder
      .spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material,
        transform,
        ..default()
      })
      .insert(collider)
      .insert(CellComponent { cell: self.clone() })
      .with_children(|child_builder| {
        let wall_state = self.wall_state.read();
        for i in 0..4 {
          let w = WALL[i];
          Wall::build(w[0], w[1], wall_state[i]).fabricate_as_child(
            child_builder,
            meshes,
            materials,
          );
        }
      })
      .id()
  }
}

trait DoorIndex {
  fn opposite(&self) -> usize;
}
impl DoorIndex for usize {
  fn opposite(&self) -> usize {
    (self + 2) % 4
  }
}
