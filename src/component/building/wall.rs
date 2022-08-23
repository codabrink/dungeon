use super::cell::D;
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier3d::prelude::*;

const W: f32 = 0.1;
const DOOR_WIDTH: f32 = 1.;
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
    mut self,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
  ) -> Option<Entity> {
    let (mut ec, cols) = match self.state {
      State::Wall => self.fabricate_wall(commands, meshes, materials),
      State::Door => self.fabricate_door(commands, meshes, materials),
      _ => return None,
    };

    let ec = ec.insert(self);
    for col in cols {
      ec.insert(col);
    }

    Some(ec.id())
  }

  fn fabricate_wall<'w, 's, 'a>(
    &mut self,
    commands: &'a mut Commands<'w, 's>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
  ) -> (EntityCommands<'w, 's, 'a>, Vec<Collider>) {
    let mesh = Mesh::from(shape::Box::new(0.1, D / 2., self.len));
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh)
      .expect("Could not create wall collider from mesh.");

    self.translation += Vec3::new(0., D / 4., 0.);

    let ec = commands.spawn_bundle(PbrBundle {
      mesh: meshes.add(mesh),
      transform: Transform::from_translation(self.translation).with_rotation(self.rotation),
      material: materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
      }),
      ..default()
    });

    (ec, vec![collider])
  }

  fn fabricate_door<'w, 's, 'a>(
    &self,
    commands: &'a mut Commands<'w, 's>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
  ) -> (EntityCommands<'w, 's, 'a>, Vec<Collider>) {
    let mesh = Mesh::from(shape::Box {
      min_x: -W / 2.,
      max_x: W / 2.,
      min_y: 0.,
      max_y: D / 2.,
      min_z: -DOOR_WIDTH / 2.,
      max_z: self.len / 2.,
    });

    println!("Box?: {:?}", shape::Box::new(0.1, D / 2., self.len));
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh)
      .expect("Could not create wall collider from mesh.");

    let ec = commands.spawn_bundle(PbrBundle {
      mesh: meshes.add(mesh),
      transform: Transform::from_translation(self.translation).with_rotation(self.rotation),
      material: materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
      }),
      ..default()
    });

    (ec, vec![collider])
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
