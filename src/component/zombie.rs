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
use rand::thread_rng;

#[derive(Component)]
pub struct Zombie {
  dest: Vec3,
  nav: Vec<Arc<NavNode>>,
  debug_square: Option<Entity>,
  nav_timeout: Instant,
  achievement_timeout: Instant,
  stunned_until: Option<Instant>,
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "cc8705a9-9189-43e5-8a5b-c28b57bd6234"]
pub struct ZombieMaterial {
  #[uniform(0)]
  color: Color,
}

impl Material for ZombieMaterial {
  fn fragment_shader() -> ShaderRef {
    "shaders/zombie.wgsl".into()
  }
}

#[derive(Clone, ShaderType)]
struct ZombieMaterialUniformData {
  color: Color,
}

#[derive(Component)]
pub struct Aggressive;

static ZOMBIE_COUNT: AtomicUsize = AtomicUsize::new(0);
static NAV_TIMEOUT: Duration = Duration::from_secs(3);
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

    let health = Health::new(Color::rgb(1., 0., 0.));

    let id = commands
      .spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: SIZE })),
        material: materials.add(ZombieMaterial {
          color: Color::rgb(health.health(), 0., 0.),
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
      .insert(Aggressive)
      .insert(Zombie {
        dest: pos,
        nav: vec![],
        debug_square: None,
        stunned_until: None,
        nav_timeout: Instant::now(),
        achievement_timeout: Instant::now() + NAV_TIMEOUT,
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

  pub fn update_normal(
    zones: Res<Zones>,
    mut query: Query<(&Transform, &mut ExternalForce, &mut Self), Without<Aggressive>>,
    player_query: Query<&Transform, With<Player>>,
  ) {
    for (t, mut ef, mut z) in &mut query {
      z.travel(t, &mut ef, &zones);
    }
  }

  pub fn update_aggressive(
    zones: Res<Zones>,
    mut query: Query<(&Transform, &mut ExternalForce, &mut Self), With<Aggressive>>,
    player_query: Query<&Transform, With<Player>>,
  ) {
    let now = Instant::now();
    if player_query.is_empty() {
      return;
    }
    let player_transform = player_query.single();

    for (t, mut ef, mut z) in &mut query {
      // stun
      if let Some(stunned_until) = z.stunned_until {
        if stunned_until > now {
          continue;
        }

        z.stunned_until = None;
      }

      z.create_path_to_player(t, player_transform, &zones);
      z.travel(t, &mut ef, &zones);
    }
  }

  pub fn update_impact(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Velocity, &Impact, &mut Health)>,
  ) {
    for (entity, mut velocity, impact, mut health) in query.iter_mut() {
      velocity.linvel = impact.force;
      health.damage(impact.damage);

      if health.is_dead() {
        commands.entity(entity).despawn_recursive();
        continue;
      }

      commands.entity(entity).remove::<Impact>();
    }
  }

  pub fn prepare_health(
    materials: Res<RenderMaterials<ZombieMaterial>>,
    mut health_query: Query<(&mut Health, &Handle<ZombieMaterial>)>,
    render_queue: Res<RenderQueue>,
  ) {
    for (mut health, handle) in &mut health_query {
      if !health.reset_changed() {
        continue;
      }

      if let Some(material) = materials.get(handle) {
        for binding in material.bindings.iter() {
          if let OwnedBindingResource::Buffer(cur_buffer) = binding {
            let mut buffer = encase::UniformBuffer::new(Vec::new());
            buffer
              .write(&ZombieMaterialUniformData {
                color: health.color(),
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

  fn wander(&mut self, t: &Transform, zones: &Res<Zones>) {
    if self.nav_timeout > Instant::now() {
      return;
    }
    self.reset_timer();

    if let Some(cell) = zones.cell_at(&t.translation) {
      self.dest = cell.random_pos();
    } else {
      let mut rng = thread_rng();
      let var = 10.;
      self.dest = t.translation + Vec3::new(rng.gen_range(-var..var), 0., rng.gen_range(-var..var));
    }
  }

  fn reset_timer(&mut self) {
    self.nav_timeout =
      Instant::now() + Duration::from_millis(rand::thread_rng().gen_range(0..3000));
  }

  fn travel(&mut self, t: &Transform, ef: &mut ExternalForce, zones: &Res<Zones>) {
    if let Some(dest) = self.nav.last() {
      if dest.area.contains(&t.translation) {
        self.nav.pop();
        if let Some(dest) = self.nav.last() {
          self.dest = dest.pos;
          self.reset_timer();
        }

        return;
      }
    } else if self.nav.is_empty() {
      self.wander(t, zones);
    }

    ef.force = (self.dest - t.translation).normalize() * 6000.;
  }

  fn create_path_to_player(&mut self, t: &Transform, pt: &Transform, zones: &Res<Zones>) {
    if self.nav_timeout > Instant::now() {
      return;
    }
    self.reset_timer();

    if let Some(zombie_cell) = zones.cell_at(&t.translation) {
      // check if player is in same building
      if let Some(player_cell) = zombie_cell.building.pos_global_to_cell(&pt.translation) {
        let mut nav = Navigator::new(&zombie_cell.nav_node(), &player_cell.nav_node());
        nav.nav();
        nav.path.reverse();
        self.nav = nav.path;
        return;
      }
    }

    self.nav.clear();
  }
}
