use crate::*;

#[derive(Component)]
pub struct Grass;

impl Grass {
  pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
  ) {
    let texture_handle = asset_server.load("grass.jpg");
    let aspect = 0.25;

    let material = materials.add(StandardMaterial {
      base_color_texture: Some(texture_handle.clone()),
      alpha_mode: AlphaMode::Blend,
      reflectance: 0.0,
      perceptual_roughness: 1.,
      // unlit: true,
      ..Default::default()
    });

    const size: f32 = 50.;
    const size_2: f32 = size / 2.;

    let r = 10;
    for z in -r..r {
      for x in -r..r {
        let transform = Transform::from_xyz(size * x as f32, 0., size * z as f32);
        commands
          .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size })),
            material: material.clone(),
            transform,
            ..default()
          })
          .insert(RigidBody::Fixed)
          .insert(Collider::cuboid(size_2, 0.2, size_2))
          .insert(Self);
      }
    }
  }
}
