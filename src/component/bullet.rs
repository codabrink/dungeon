use super::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::time::{Duration, Instant};

#[derive(Component)]
pub struct Bullet {
  created_at: Instant,
  vel: Vec3,
}

#[derive(Component)]
pub struct Impact {
  pub force: Vec3,
  pub damage: f32,
}

const RAD: f32 = 0.5;
const VEL: f32 = 500.;

const COLLISION_WIDTH_2: f32 = 3.5;
const COLLISION_HEIGHT_2: f32 = 5.;
const COLLISION_LENGTH_2: f32 = 15.;

impl Bullet {
  pub fn spawn(
    input: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    rapier_context: Res<RapierContext>,
    query: Query<(Entity, &Transform, &Player)>,
  ) {
    if query.is_empty() {
      return;
    }

    let (player_entity, t, player) = query.single();
    let theta = player.angle;

    if !input.just_pressed(KeyCode::Space) && !mouse.just_pressed(MouseButton::Left) {
      return;
    }

    let direction = Vec3::new(theta.sin(), 0., theta.cos());
    let transform = Transform::from_translation(t.translation + direction * 2.);

    let shape = Collider::cuboid(COLLISION_WIDTH_2, COLLISION_HEIGHT_2, COLLISION_LENGTH_2);
    let shape_pos = t.translation + direction * COLLISION_LENGTH_2;
    let shape_rot = Quat::from_axis_angle(Vec3::Y, theta);

    commands.entity(player_entity).insert(Impact {
      force: direction * -20.,
      damage: 0.,
    });

    rapier_context.intersections_with_shape(
      shape_pos,
      shape_rot,
      &shape,
      QueryFilter::default().exclude_collider(player_entity),
      |entity| {
        commands.entity(entity).insert(Impact {
          force: direction * 50.,
          damage: 0.4,
        });
        true
      },
    );

    let mesh = Mesh::from(shape::Cube { size: RAD });

    commands
      .spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Self::material()),
        transform,
        ..default()
      })
      // .insert(Velocity::linear(translation * 100.))
      .insert(Bullet {
        created_at: Instant::now(),
        vel: direction * VEL,
      })
      .with_children(|cb| {
        cb.spawn_bundle(PointLightBundle { ..default() });
      });
  }

  pub fn update(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &Bullet, &mut Transform)>,
  ) {
    let now = Instant::now();
    for (e, bullet, mut t) in query.iter_mut() {
      if now.duration_since(bullet.created_at) > Duration::from_secs(1) {
        commands.entity(e).despawn_recursive();
      }

      t.translation += bullet.vel * time.delta_seconds();
    }
  }

  fn material() -> StandardMaterial {
    StandardMaterial {
      base_color: Color::WHITE,
      unlit: true,
      ..default()
    }
  }
}
