use crate::*;

const ROAD_WIDTH: f32 = 20.;
const ROAD_DEPTH: f32 = 0.2;
const ROAD_LINE_LEN: f32 = 1.;
const ROAD_LINE_LEN_2: f32 = ROAD_LINE_LEN / 2.;
const ROAD_LINE_GAP: f32 = 1.;
const ROAD_LINE_WIDTH: f32 = ROAD_LINE_LEN / 8.;

#[derive(Component)]
pub struct Road {
  len: f32,
  translation: Vec3,
  rotation: Quat,
}

impl Road {
  pub fn build(from: Vec3, to: Vec3) -> Self {
    let dz = (from.z - to.z).abs();
    let dx = (from.x - to.x).abs();
    let angle = (dx / dz).atan();
    let len = (dz * dz + dx * dx).sqrt();

    Self {
      len,
      translation: Vec3::new((from.x + to.x) / 2., 0.3, (from.z + to.z) / 2.),
      rotation: Quat::from_axis_angle(Vec3::Y, angle),
    }
  }

  pub fn fabricate(
    self,
    child_builder: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
  ) -> Entity {
    println!("New road");
    let mesh = Mesh::from(shape::Box::new(ROAD_WIDTH, ROAD_DEPTH, self.len));
    let material = StandardMaterial {
      base_color: Color::BLACK,
      ..default()
    };

    child_builder
      .spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(material),
        transform: Transform::from_translation(self.translation).with_rotation(self.rotation),
        ..default()
      })
      .with_children(|child_builder| {
        let mut z = ROAD_LINE_LEN_2;
        let mesh = Mesh::from(shape::Box::new(ROAD_LINE_WIDTH, 0.1, ROAD_LINE_LEN));
        let mesh = meshes.add(mesh);
        let material = StandardMaterial {
          base_color: Color::WHITE,
          ..default()
        };
        let material = materials.add(material);
        while z < self.len {
          child_builder.spawn_bundle(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(0., 0.01, z),
            ..default()
          });
          z += ROAD_LINE_LEN + ROAD_LINE_GAP;
        }
      })
      .id()
  }
}

pub const GRID_SIZE: f32 = 800.;
pub const GRID_SIZE_2: f32 = GRID_SIZE / 2.;
static WAIT: Duration = Duration::from_secs(5);
pub struct RoadGrid {
  pub grid: HashMap<Coord, RoadCell>,
  last_ran: Instant,
}
impl Default for RoadGrid {
  fn default() -> Self {
    Self {
      grid: HashMap::new(),
      last_ran: Instant::now() - WAIT,
    }
  }
}

impl RoadGrid {
  #[inline]
  fn coord(t: &Transform) -> Coord {
    Coord {
      z: (t.translation.z / GRID_SIZE) as i16,
      x: (t.translation.x / GRID_SIZE) as i16,
    }
  }
  pub fn update(
    mut grid: ResMut<Self>,
    query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
  ) {
    if grid.last_ran.elapsed() < WAIT {
      return;
    }

    let coord = Self::coord(query.single());
    grid
      .grid
      .entry(coord)
      .or_insert_with(|| RoadCell::new(coord, &mut commands, &mut meshes, &mut materials));

    grid.last_ran = Instant::now();
  }
}

pub struct RoadCell {
  pub roads: HashSet<Entity>,
}

impl RoadCell {
  fn new(
    coord: Coord,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
  ) -> Self {
    println!("New road cell");
    let translation = Vec3::new(coord.x as f32 * GRID_SIZE, 0., coord.z as f32 * GRID_SIZE);

    let mut roads = HashSet::new();
    commands
      .spawn_bundle(PbrBundle {
        transform: Transform::from_translation(translation),
        ..default()
      })
      .with_children(|child_builder| {
        let road = Road::build(
          translation + Vec3::new(-GRID_SIZE_2, 0., -GRID_SIZE_2),
          translation + Vec3::new(-GRID_SIZE_2, 0., GRID_SIZE_2),
        )
        .fabricate(child_builder, meshes, materials);
        roads.insert(road);
      });

    Self { roads }
  }
}
