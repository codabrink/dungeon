use crate::*;

#[derive(Component)]
pub struct DebugLine {
  len: f32,
  translation: Vec3,
  rotation: Quat,
  color: Color,
}

const DIAMETER: f32 = 0.5;

impl DebugLine {
  pub fn build(from: Vec3, to: Vec3, color: Color) -> Self {
    let dz = (from.z - to.z).abs();
    let dx = (from.x - to.x).abs();
    let angle = (dx / dz).atan();
    let len = (dz * dz + dx * dx).sqrt();

    Self {
      len,
      color,
      translation: Vec3::new((from.x + to.x) / 2., 5., (from.z + to.z) / 2.),
      rotation: Quat::from_axis_angle(Vec3::Y, angle),
    }
  }

  pub fn fabricate(
    self,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
  ) -> Entity {
    self._fabricate(commands.spawn(), meshes, materials)
  }

  fn _fabricate(
    self,
    mut ec: EntityCommands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
  ) -> Entity {
    let mesh = meshes.add(Mesh::from(shape::Box::new(DIAMETER, DIAMETER, self.len)));
    let material = materials.add(StandardMaterial {
      base_color: self.color,
      unlit: true,
      ..default()
    });

    ec.insert_bundle(PbrBundle {
      mesh,
      material,
      transform: Transform::from_translation(self.translation).with_rotation(self.rotation),
      ..default()
    })
    .insert(self)
    .id()
  }
}
