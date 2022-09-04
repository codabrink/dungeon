use crate::*;

const ROAD_WIDTH: f32 = 40.;
const ROAD_WIDTH_2: f32 = ROAD_WIDTH / 2.;
const ROAD_DEPTH: f32 = 0.2;
const ROAD_LINE_LEN: f32 = 8.;
const ROAD_LINE_LEN_2: f32 = ROAD_LINE_LEN / 2.;
const ROAD_LINE_GAP: f32 = ROAD_LINE_LEN * 2.;
const ROAD_LINE_WIDTH: f32 = ROAD_LINE_LEN / 6.;
const SIDEWALK_WIDTH: f32 = 5.;

const STREET_LAMP_GAP: f32 = 70.;

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
      translation: Vec3::new((from.x + to.x) / 2., 0.1, (from.z + to.z) / 2.),
      rotation: Quat::from_axis_angle(Vec3::Y, angle),
    }
  }

  pub fn fabricate(
    self,
    child_builder: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    ass: &Res<AssetServer>,
  ) -> Entity {
    println!("New road");
    let mesh = Mesh::from(shape::Box::new(ROAD_WIDTH, ROAD_DEPTH, self.len));
    let material = materials.add(StandardMaterial {
      base_color: Color::BLACK,
      ..default()
    });

    child_builder
      .spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material,
        transform: Transform::from_translation(self.translation).with_rotation(self.rotation),
        ..default()
      })
      .with_children(|child_builder| {
        let len_2 = self.len / 2.;

        // white lines
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
            transform: Transform::from_xyz(0., 0.05, z - len_2),
            ..default()
          });
          z += ROAD_LINE_LEN + ROAD_LINE_GAP;
        }

        // street lamps
        let mut z = -len_2;
        while z < len_2 {
          let transform = Transform::from_xyz(ROAD_WIDTH_2 + SIDEWALK_WIDTH, 0., z);
          StreetLamp::fabricate(transform, child_builder, ass);

          z += STREET_LAMP_GAP;
        }

        // sidewalks
        let mesh = meshes.add(Mesh::from(shape::Box::new(SIDEWALK_WIDTH, 0.1, self.len)));
        let material = materials.add(StandardMaterial {
          base_color: Color::GRAY,
          ..default()
        });
        child_builder.spawn_bundle(PbrBundle {
          mesh: mesh.clone(),
          material: material.clone(),
          transform: Transform::from_xyz(ROAD_WIDTH_2, 0.1, 0.),
          ..default()
        });
        child_builder.spawn_bundle(PbrBundle {
          mesh: mesh,
          material: material,
          transform: Transform::from_xyz(-ROAD_WIDTH_2, 0.1, 0.),
          ..default()
        });
      })
      .id()
  }
}

struct StreetLamp;

impl StreetLamp {
  pub fn fabricate(
    transform: Transform,
    child_builder: &mut ChildBuilder,
    ass: &Res<AssetServer>,
  ) -> Entity {
    let scene = ass.load("models/street.glb#Scene0");
    child_builder
      .spawn_bundle(SceneBundle {
        scene,
        transform,
        ..default()
      })
      .with_children(|child_builder| {
        child_builder.spawn_bundle(PointLightBundle {
          point_light: PointLight {
            range: 40.,
            intensity: 10000.,
            ..default()
          },
          transform: Transform::from_xyz(0., 15., 0.),
          ..default()
        });
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
    ass: Res<AssetServer>,
  ) {
    if grid.last_ran.elapsed() < WAIT {
      return;
    }

    let coord = Self::coord(query.single());
    grid
      .grid
      .entry(coord)
      .or_insert_with(|| RoadCell::new(coord, &mut commands, &mut meshes, &mut materials, &ass));

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
    ass: &Res<AssetServer>,
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
        .fabricate(child_builder, meshes, materials, ass);
        roads.insert(road);
      });

    Self { roads }
  }
}
