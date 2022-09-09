use crate::*;
use bevy::{
  pbr::RenderMaterials,
  reflect::TypeUuid,
  render::{
    render_resource::{encase, AsBindGroup, OwnedBindingResource, ShaderRef, ShaderType},
    renderer::RenderQueue,
    Extract,
  },
};
use rand::{thread_rng, Rng};

#[derive(Component)]
pub struct Zombie {
  nav_timeout: Instant,
  dest: Option<Vec3>,
  nav: Vec<Arc<NavNode>>,
  debug_square: Option<Entity>,
  last_achievement: Instant,
  stunned_until: Option<Instant>,
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "cc8705a9-9189-43e5-8a5b-c28b57bd6234"]
pub struct ZombieMaterial {
  #[uniform(0)]
  health: f32,
}

impl Material for ZombieMaterial {
  fn fragment_shader() -> ShaderRef {
    "shaders/zombie.wgsl".into()
  }
}

#[derive(Clone, ShaderType)]
struct ZombieMaterialUniformData {
  health: f32,
}

#[derive(Component)]
pub struct Aggressive;

static ZOMBIE_COUNT: AtomicUsize = AtomicUsize::new(0);
static ACHIEVEMENT_TIMEOUT: Duration = Duration::from_secs(3);
const ZOMBIE_LIMIT: usize = 50;

const SIZE: f32 = 2.;
const SIZE_2: f32 = SIZE / 2.;

impl Zombie {
  pub fn fabricate(
    pos: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ZombieMaterial>>,
  ) -> Option<Entity> {
    if ZOMBIE_COUNT.load(Ordering::SeqCst) >= ZOMBIE_LIMIT {
      return None;
    }

    let health = Health::default();

    let id = commands
      .spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: SIZE })),
        material: materials.add(ZombieMaterial {
          health: health.health,
        }),
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
      .insert(Velocity::default())
      .insert(health)
      .insert(Zombie {
        nav_timeout: Instant::now(),
        dest: None,
        nav: vec![],
        debug_square: None,
        last_achievement: Instant::now(),
        stunned_until: None,
      })
      .id();

    ZOMBIE_COUNT.fetch_add(1, Ordering::SeqCst);

    Some(id)
  }

  fn material() -> StandardMaterial {
    StandardMaterial {
      base_color: Color::RED,
      unlit: true,
      ..default()
    }
  }

  pub fn update(
    zones: Res<Zones>,
    mut query: Query<(&Transform, &mut ExternalForce, &mut Self), Without<Aggressive>>,
    player_query: Query<&Transform, With<Player>>,

    commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
  ) {
    let now = Instant::now();
    let player_transform = player_query.single();

    for (t, mut ef, mut z) in query.iter_mut() {
      // stun
      if let Some(stunned_until) = z.stunned_until {
        if stunned_until > now {
          continue;
        }

        z.stunned_until = None;
      }

      z.nav(t, player_transform, &zones);

      z.check_arrived(t);
      z.update_dest(t);

      if let Some(dest) = z.dest {
        ef.force = (dest - t.translation).normalize() * 6000.;
      }
    }
  }

  pub fn update_impact(
    mut commands: Commands,

    mut query: Query<(
      Entity,
      &mut Zombie,
      &mut Velocity,
      &BulletImpact,
      &mut Health,
      &Handle<ZombieMaterial>,
    )>,
  ) {
    for (entity, mut zombie, mut velocity, bullet_impact, mut health, material) in query.iter_mut()
    {
      zombie.stunned_until = Some(Instant::now() + Duration::from_secs(5));
      velocity.linvel = bullet_impact.force;
      // external_force.force = bullet_impact.force;
      health.health -= bullet_impact.damage;

      if health.is_dead() {
        commands.entity(entity).despawn_recursive();
        continue;
      }

      commands.entity(entity).remove::<BulletImpact>();
    }
  }

  pub fn prepare_health(
    materials: Res<RenderMaterials<ZombieMaterial>>,
    health_query: Query<(&Health, &Handle<ZombieMaterial>)>,
    render_queue: Res<RenderQueue>,
  ) {
    for (health, handle) in &health_query {
      if let Some(material) = materials.get(handle) {
        for binding in material.bindings.iter() {
          if let OwnedBindingResource::Buffer(cur_buffer) = binding {
            let mut buffer = encase::UniformBuffer::new(Vec::new());
            buffer
              .write(&ZombieMaterialUniformData {
                health: health.health,
              })
              .unwrap();
            render_queue.write_buffer(cur_buffer, 0, buffer.as_ref());
          }
        }
      }
    }
  }

  pub fn extract_health(
    mut commands: Commands,
    health_query: Extract<Query<(Entity, &Health, &Handle<ZombieMaterial>)>>,
  ) {
    for (entity, health, handle) in health_query.iter() {
      commands
        .get_or_spawn(entity)
        .insert(*health)
        .insert(handle.clone());
    }
  }

  fn check_arrived(&mut self, t: &Transform) {
    if let Some(dest) = self.nav.last() {
      if dest.area.contains(&t.translation) {
        self.nav.pop();
        self.dest = None;
        self.last_achievement = Instant::now();
      }
    }
  }

  fn update_dest(&mut self, t: &Transform) {
    if let Some(dest) = self.nav.last() {
      if dest.area.contains(&t.translation) {
        self.nav.pop();
        self.dest = None;

        self.update_dest(t);
        return;
      }

      if self.dest.is_none() {
        self.dest = Some(dest.area.random(SIZE_2));
      }
    }
  }

  fn nav(&mut self, t: &Transform, pt: &Transform, zones: &Res<Zones>) {
    if !self.nav.is_empty() && self.last_achievement + ACHIEVEMENT_TIMEOUT > Instant::now() {
      return;
    }

    let mut rng = thread_rng();
    self.nav_timeout = Instant::now() + Duration::from_millis(rng.gen_range(1000..3000));

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
              self.dest = None;
              // println!("Nav len: {}", self.nav.len());
            }
            _ => {
              self.dest = Some(cell.random_pos());
              return;
            }
          }
        }
      }
    }
  }
}
