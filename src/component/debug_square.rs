use crate::*;

#[derive(Component)]
pub struct DebugSquare {
  translation: Vec3,
  color: Color,
}

const DIAMETER: f32 = 1.;

impl DebugSquare {
  pub fn build(pos: Vec3, color: Color) -> Self {
    Self {
      color,
      translation: pos,
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
    let mesh = meshes.add(Mesh::from(shape::Box::new(DIAMETER, DIAMETER, DIAMETER)));
    let material = materials.add(StandardMaterial {
      base_color: self.color,
      unlit: true,
      ..default()
    });

    ec.insert_bundle(PbrBundle {
      mesh,
      material,
      transform: Transform::from_translation(self.translation),
      ..default()
    })
    .insert(self)
    .id()
  }
}
