use super::cell::D;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const WALL_W: f32 = 0.5;
const DOOR_W: f32 = 8.;
const DOOR_W_2: f32 = DOOR_W / 2.;
const WALL_H: f32 = D / 2.;
const WALL_H_2: f32 = WALL_H / 2.;
const WALL_H_4: f32 = WALL_H_2 / 2.;
const WALL_H_8: f32 = WALL_H_4 / 2.;
const FRAME_W: f32 = WALL_W * 2.;

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
    child_builder: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    // asset_server: &Res<AssetServer>,
  ) -> Option<Entity> {
    let result = match self.state {
      State::Solid => self.fabricate_wall(),
      State::Door => self.fabricate_door(),
      _ => return None,
    };

    let mut entity = child_builder.spawn();
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
    let mesh = Mesh::from(shape::Box::new(WALL_W, WALL_H, self.len));
    let collider = Collider::cuboid(WALL_W, WALL_H_2, self.len / 2.);

    let transform = Transform::from_translation(Vec3::new(0., WALL_H_2, 0.));

    let material = StandardMaterial {
      base_color: Color::WHITE,
      ..default()
    };

    vec![(mesh, transform, material, Some(collider))]
  }

  fn fabricate_door(&self) -> Vec<(Mesh, Transform, StandardMaterial, Option<Collider>)> {
    let mut result = vec![];
    let width = self.len / 2. - DOOR_W_2;
    let mesh = Mesh::from(shape::Box::new(WALL_W, WALL_H, width));

    // left side
    let collider = Collider::cuboid(WALL_W, WALL_H_2, width / 2.);
    let transform = Transform::from_translation(Vec3::new(0., WALL_H_2, -(width / 2. + DOOR_W_2)));
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
    let transform = Transform::from_translation(Vec3::new(0., WALL_H_2, width / 2. + DOOR_W_2));
    result.push((mesh, transform, material.clone(), Some(collider)));

    // above door
    let mesh = Mesh::from(shape::Box::new(WALL_W, WALL_H_4, DOOR_W));
    let transform = Transform::from_translation(Vec3::new(0., D * (7. / 16.), 0.));
    result.push((mesh, transform, material, None));

    // trim
    let material = StandardMaterial {
      base_color: Color::rgb_u8(165, 42, 42),
      ..default()
    };
    // right trim
    let mesh = Mesh::from(shape::Box::new(FRAME_W, WALL_H - WALL_H_4, FRAME_W));
    let transform = Transform::from_translation(Vec3::new(0., WALL_H_2 - WALL_H_8, DOOR_W_2));
    result.push((mesh.clone(), transform, material.clone(), None));

    // left trim
    let transform = Transform::from_translation(Vec3::new(0., WALL_H_2 - WALL_H_8, -DOOR_W_2));
    result.push((mesh, transform, material.clone(), None));

    // top trim
    let mesh = Mesh::from(shape::Box::new(FRAME_W, FRAME_W, DOOR_W + FRAME_W));
    let transform = Transform::from_translation(Vec3::new(0., WALL_H_2 + WALL_H_4, 0.));
    result.push((mesh, transform, material, None));

    result
  }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum State {
  #[default]
  None,
  Solid,
  Door,
  Window,
}
