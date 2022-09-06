use crate::*;
use rand::{thread_rng, Rng};

#[derive(Component)]
pub struct Zombie {
  next_spot_update: Instant,
  last_nav: Instant,
  dest: Vec3,
  nav: Vec<Arc<NavNode>>,
  debug_square: Mutex<Option<Entity>>,
}

#[derive(Component)]
pub struct Aggressive;

static ZOMBIE_COUNT: AtomicUsize = AtomicUsize::new(0);

const SIZE: f32 = 2.;
const SIZE_2: f32 = SIZE / 2.;

impl Zombie {
  pub fn fabricate(
    pos: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
  ) -> Entity {
    commands
      .spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: SIZE })),
        material: materials.add(Self::material()),
        transform: Transform::from_xyz(pos.x, SIZE, pos.z),
        ..default()
      })
      .insert(ExternalForce::default())
      .insert(Collider::cuboid(SIZE_2, SIZE_2, SIZE_2))
      .insert(RigidBody::Dynamic)
      .insert(Damping {
        linear_damping: 10.,
        angular_damping: 1.,
      })
      .insert(Zombie {
        next_spot_update: Instant::now(),
        last_nav: Instant::now() - Duration::from_secs(60),
        dest: pos,
        nav: vec![],
        debug_square: Mutex::default(),
      })
      .id()
  }

  fn material() -> StandardMaterial {
    StandardMaterial {
      base_color: Color::RED,
      // unlit: true,
      ..default()
    }
  }

  pub fn update(
    zones: Res<Zones>,
    mut query: Query<(&Transform, &mut ExternalForce, &mut Self), Without<Aggressive>>,
    player_query: Query<&Transform, With<Player>>,

    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
  ) {
    let now = Instant::now();
    let player_transform = player_query.single();

    for (t, mut ef, mut z) in query.iter_mut() {
      if z.next_spot_update < now {
        z.nav(t, player_transform, &zones);
      }

      let dest = if let Some(dest) = z.nav.last() {
        if dest.area.contains(t.translation) {
          z.nav.pop();

          let mut debug_square = z.debug_square.lock();
          if let Some(entity) = *debug_square {
            commands.entity(entity).despawn_recursive();
            *debug_square = None;
          }

          continue;
        }

        let mut debug_square = z.debug_square.lock();
        if debug_square.is_none() {
          *debug_square = Some(
            DebugSquare::build(dest.pos + Vec3::new(0., 2., 0.), Color::ORANGE).fabricate(
              &mut commands,
              &mut meshes,
              &mut materials,
            ),
          );
        }

        &dest.pos
      } else {
        continue;
        // &z.dest
      };

      ef.force = (*dest - t.translation).normalize() * 2500.;
    }
  }

  fn nav(&mut self, t: &Transform, pt: &Transform, zones: &Res<Zones>) {
    if !self.nav.is_empty() {
      return;
    }

    let mut rng = thread_rng();
    self.next_spot_update = Instant::now() + Duration::from_millis(rng.gen_range(100000..300000));

    if let Some(zone) = zones.zone(&t.translation) {
      for building in &zone.buildings {
        if let Some(cell) = building.pos_global_to_cell(t.translation) {
          match building.pos_global_to_cell(pt.translation) {
            Some(pcell) => {
              let mut nav = Navigator::new(
                cell.nav_nodes.read()[4].as_ref().unwrap(),
                pcell.nav_nodes.read()[4].as_ref().unwrap(),
              );
              nav.nav();
              nav.path.reverse();
              self.nav = nav.path;
            }
            _ => {
              self.dest = cell.random_pos();
              return;
            }
          }
        }
      }
    }
  }
}
