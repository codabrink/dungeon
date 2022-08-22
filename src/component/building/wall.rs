use bevy::prelude::*;

use super::cell::D;

#[derive(Component)]
pub struct Wall;

pub struct WallBuilder {
  len: f32,
  transform: Transform,
  angle: f32,
}

impl Wall {
  pub fn build(from: Vec3, to: Vec3) -> WallBuilder {
    let dz = (from.z - to.z).abs();
    let dx = (from.x - to.x).abs();
    let angle = (dx / dz).atan();
    let len = (dz * dz + dx * dx).sqrt();

    let transform = Transform::from_xyz((from.x + to.x) / 2., 0., (from.z + to.z) / 2.)
      .with_rotation(Quat::from_axis_angle(Vec3::Y, angle));

    WallBuilder {
      len,
      transform,
      angle,
    }
  }
}

impl WallBuilder {
  pub fn fabricate(
    self,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
  ) -> Entity {
    commands
      .spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(0.1, D, self.len))),
        transform: self.transform,
        material: materials.add(StandardMaterial {
          base_color: Color::WHITE,
          ..default()
        }),
        ..default()
      })
      .insert(Wall)
      .id()
  }
}

#[derive(Debug, Default)]
pub enum State {
  #[default]
  None,
  Wall,
  Door,
  Window,
}
