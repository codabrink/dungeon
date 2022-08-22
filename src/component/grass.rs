use bevy::prelude::*;

#[derive(Component)]
pub struct Grass;

impl Grass {
  pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
  ) {
    let texture_handle = asset_server.load("grass.png");
    let aspect = 0.25;

    let material = materials.add(StandardMaterial {
      base_color_texture: Some(texture_handle.clone()),
      alpha_mode: AlphaMode::Blend,
      reflectance: 0.0,
      // unlit: true,
      ..Default::default()
    });

    let size = 50.;

    commands
      .spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size })),
        material,
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
      })
      .insert(Self);
  }
}
