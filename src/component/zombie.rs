use crate::*;
use rand::{thread_rng, Rng};

#[derive(Component)]
pub struct Zombie {
  next_spot_update: Instant,
  dest: Vec3,
}

#[derive(Component)]
pub struct Aggressive;

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
      .insert(Zombie {
        next_spot_update: Instant::now(),
        dest: pos,
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
  ) {
    let now = Instant::now();
    for (t, mut ef, mut z) in query.iter_mut() {
      if z.next_spot_update < now {
        z.dest = z.find_new_spot(t, &zones);
      }

      ef.force = (z.dest - t.translation).normalize() * 1000.;
    }
  }

  fn find_new_spot(&mut self, t: &Transform, zones: &Res<Zones>) -> Vec3 {
    let mut rng = thread_rng();
    self.next_spot_update = Instant::now() + Duration::from_millis(rng.gen_range(1000..3000));

    if let Some(zone) = zones.zone(&t.translation) {
      for building in &zone.buildings {
        if let Some(cell) = building.pos_global_to_cell(t.translation) {
          return cell.random_pos();
        }
      }
    }

    let mut pos = t.translation.clone();
    pos.z += rng.gen_range(-CELL_SIZE_2..CELL_SIZE_2);
    pos.x += rng.gen_range(-CELL_SIZE_2..CELL_SIZE_2);
    pos
  }
}
