use super::cell::D;
use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::RigidBodyBuilder};

const W: f32 = 0.1;
const DOOR_WIDTH: f32 = 8.;
#[derive(Component)]
pub struct Wall {
  len: f32,
  state: State,
  translation: Vec3,
  rotation: Quat,
}

impl Wall {
  pub fn build(from: Vec3, to: Vec3, state: State) -> Self {
    let dz = (from.z - to.z).abs();
    let dx = (from.x - to.x).abs();
    let angle = (dx / dz).atan();
    let len = (dz * dz + dx * dx).sqrt();

    Self {
      len,
      state,
      translation: Vec3::new((from.x + to.x) / 2., 0., (from.z + to.z) / 2.),
      rotation: Quat::from_axis_angle(Vec3::Y, angle),
    }
  }

  pub fn fabricate(
    self,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
  ) -> Option<Entity> {
    let result = match self.state {
      State::Wall => self.fabricate_wall(),
      State::Door => self.fabricate_door(),
      _ => vec![],
    };

    let mut entity = commands.spawn();
    let ec = entity
      .insert_bundle(PbrBundle {
        transform: Transform::from_translation(self.translation).with_rotation(self.rotation),
        ..default()
      })
      .insert(self);

    for (mesh, transform, material, collider) in result {
      ec.with_children(|builder| {
        let mut entity = builder.spawn_bundle(PbrBundle {
          mesh: meshes.add(mesh),
          transform,
          material: materials.add(material),
          ..default()
        });

        if let Some(collider) = collider {
          entity.insert(collider);
          // .insert(RigidBody::Dynamic)
          // .insert(Sleeping {
          // sleeping: true,
          // ..default()
          // });q
        }
      });
    }

    Some(ec.id())
  }

  fn fabricate_wall(&self) -> Vec<(Mesh, Transform, StandardMaterial, Option<Collider>)> {
    let mesh = Mesh::from(shape::Box::new(0.1, D / 2., self.len));
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh)
      .expect("Could not create wall collider from mesh.");

    let transform = Transform::from_translation(Vec3::new(0., D / 4., 0.));

    let material = StandardMaterial {
      base_color: Color::WHITE,
      ..default()
    };

    vec![(mesh, transform, material, Some(collider))]
  }

  fn fabricate_door(&self) -> Vec<(Mesh, Transform, StandardMaterial, Option<Collider>)> {
    let mut result = vec![];
    let width = self.len / 2. - DOOR_WIDTH / 2.;
    let mesh = Mesh::from(shape::Box::new(W, D / 2., width));

    // left side
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh)
      .expect("Could not create wall collider from mesh.");
    let transform =
      Transform::from_translation(Vec3::new(0., D / 4., -(width / 2. + DOOR_WIDTH / 2.)));
    let material = StandardMaterial {
      base_color: Color::WHITE,
      ..default()
    };
    result.push((
      mesh.clone(),
      transform,
      material.clone(),
      Some(collider.clone()),
    ));

    // right side
    let transform =
      Transform::from_translation(Vec3::new(0., D / 4., width / 2. + DOOR_WIDTH / 2.));
    result.push((mesh, transform, material.clone(), Some(collider)));

    // above door
    let mesh = Mesh::from(shape::Box::new(W, D / 8., DOOR_WIDTH));
    let transform = Transform::from_translation(Vec3::new(0., D * (7. / 16.), 0.));
    result.push((mesh, transform, material, None));

    result
  }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum State {
  #[default]
  None,
  Wall,
  Door,
  Window,
}
