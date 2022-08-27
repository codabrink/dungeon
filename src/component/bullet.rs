use super::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::time::{Duration, Instant};

#[derive(Component)]
pub struct Bullet {
  created_at: Instant,
}

const RAD: f32 = 0.5;

impl Bullet {
  pub fn spawn(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&Transform, &Player), With<Player>>,
  ) {
    let (t, player) = query.single();
    let theta = player.angle;

    if !input.just_pressed(KeyCode::Space) {
      return;
    }
    let translation = Vec3::new(2. * theta.sin(), 0., 2. * theta.cos());

    let mesh = Mesh::from(shape::Cube { size: RAD });
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap();

    commands
      .spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Self::material()),
        transform: Transform::from_translation(t.translation + translation),
        ..default()
      })
      .insert(RigidBody::Dynamic)
      .insert(Velocity::linear(translation * 100.))
      .insert(collider)
      .insert(Bullet {
        created_at: Instant::now(),
      });
  }

  pub fn despawn(mut commands: Commands, query: Query<(Entity, &Bullet)>) {
    let now = Instant::now();
    for (e, bullet) in query.iter() {
      if now.duration_since(bullet.created_at) > Duration::from_secs(1) {
        commands.entity(e).despawn();
      }
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
