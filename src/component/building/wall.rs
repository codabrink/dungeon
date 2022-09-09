use super::cell::CELL_SIZE;
use crate::*;

const WALL_W: f32 = 0.5;
pub const DOOR_W: f32 = 8.;
pub const DOOR_W_2: f32 = DOOR_W / 2.;
const WALL_H: f32 = CELL_SIZE / 2.;
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
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
  ) -> Option<Entity> {
    self._fabricate(commands.spawn(), meshes, materials)
  }
  pub fn fabricate_as_child(
    self,
    child_builder: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    // asset_server: &Res<AssetServer>,
  ) -> Option<Entity> {
    self._fabricate(child_builder.spawn(), meshes, materials)
  }

  fn _fabricate(
    self,
    mut ec: EntityCommands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
  ) -> Option<Entity> {
    let ec = ec.insert_bundle(PbrBundle {
      transform: Transform::from_translation(self.translation).with_rotation(self.rotation),
      ..default()
    });

    match self.state {
      State::Solid => self.fabricate_wall(ec, meshes, materials),
      State::Door => self.fabricate_door(ec, meshes, materials),
      _ => return None,
    };

    Some(ec.insert(self).id())
  }

  fn fabricate_wall(
    &self,
    ec: &mut EntityCommands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
  ) -> Entity {
    let mesh = meshes.add(Mesh::from(shape::Box::new(WALL_W, WALL_H, self.len)));
    let collider = Collider::cuboid(WALL_W, WALL_H_2, self.len / 2.);
    let material = materials.add(Self::white_material());

    ec.with_children(|child_builder| {
      child_builder
        .spawn_bundle(PbrBundle {
          mesh,
          material,
          transform: Transform::from_translation(Vec3::new(0., WALL_H_2, 0.)),
          ..default()
        })
        .insert(collider);
    })
    .id()
  }

  fn fabricate_door(
    &self,
    ec: &mut EntityCommands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
  ) -> Entity {
    let width = self.len / 2. - DOOR_W_2;
    let material = materials.add(Self::white_material());
    let mesh = meshes.add(Mesh::from(shape::Box::new(WALL_W, WALL_H, width)));
    let collider = Collider::cuboid(WALL_W, WALL_H_2, width / 2.);

    ec.with_children(|child_builder| {
      // left side
      child_builder
        .spawn_bundle(PbrBundle {
          mesh: mesh.clone(),
          material: material.clone(),
          transform: Transform::from_translation(Vec3::new(0., WALL_H_2, -(width / 2. + DOOR_W_2))),
          ..default()
        })
        .insert(collider.clone());

      // right side
      child_builder
        .spawn_bundle(PbrBundle {
          mesh,
          transform: Transform::from_translation(Vec3::new(0., WALL_H_2, width / 2. + DOOR_W_2)),
          material: material.clone(),
          ..default()
        })
        .insert(collider);

      // above door
      child_builder.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(WALL_W, WALL_H_4, DOOR_W))),
        transform: Transform::from_translation(Vec3::new(0., CELL_SIZE * (7. / 16.), 0.)),
        material,
        ..default()
      });

      let material = materials.add(Self::brown_material());
      let mesh = meshes.add(Mesh::from(shape::Box::new(
        FRAME_W,
        WALL_H - WALL_H_4,
        FRAME_W,
      )));
      // right trim
      child_builder.spawn_bundle(PbrBundle {
        mesh: mesh.clone(),
        transform: Transform::from_translation(Vec3::new(0., WALL_H_2 - WALL_H_8, DOOR_W_2)),
        material: material.clone(),
        ..default()
      });

      // left trim
      child_builder.spawn_bundle(PbrBundle {
        mesh,
        transform: Transform::from_translation(Vec3::new(0., WALL_H_2 - WALL_H_8, -DOOR_W_2)),
        material: material.clone(),
        ..default()
      });

      // top trim
      child_builder.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(
          FRAME_W,
          FRAME_W,
          DOOR_W + FRAME_W,
        ))),
        transform: Transform::from_translation(Vec3::new(0., WALL_H_2 + WALL_H_4, 0.)),
        material,
        ..default()
      });
    })
    .id()
  }

  fn white_material() -> StandardMaterial {
    StandardMaterial {
      base_color: Color::WHITE,
      ..default()
    }
  }

  fn brown_material() -> StandardMaterial {
    StandardMaterial {
      base_color: Color::rgb_u8(101, 67, 33),
      ..default()
    }
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum State {
  #[default]
  None,
  Solid,
  Door,
  Window,
}
