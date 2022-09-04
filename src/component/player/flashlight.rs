use bevy::prelude::*;

#[derive(Component)]
pub struct Flashlight;

impl Flashlight {
  pub fn setup(
    commands: &mut ChildBuilder,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
  ) {
    const S: f32 = 0.75;
    const S2: f32 = S / 2.;
    let top = Mesh::from(shape::Box::new(S, 0.01, S));
    let side = Mesh::from(shape::Box::new(0.01, S, S));
    let back = Mesh::from(shape::Box::new(S, S, 0.01));
    let material = materials.add(StandardMaterial {
      base_color: Color::rgba(0., 0., 0., 0.),
      unlit: true,
      ..default()
    });

    commands
      .spawn()
      // top
      .insert_bundle(PbrBundle {
        mesh: meshes.add(top.clone()),
        transform: Transform::from_xyz(0.5, -0.5, 0.95),
        material: material.clone(),
        ..default()
      })
      // bottom
      .with_children(|f| {
        f.spawn_bundle(PbrBundle {
          mesh: meshes.add(top),
          transform: Transform::from_xyz(0., -S2, 0.),
          material: material.clone(),
          ..default()
        });
      })
      // left
      .with_children(|f| {
        f.spawn_bundle(PbrBundle {
          mesh: meshes.add(side.clone()),
          transform: Transform::from_xyz(S2, 0., 0.),
          material: material.clone(),
          ..default()
        });
      })
      // right
      .with_children(|f| {
        f.spawn_bundle(PbrBundle {
          mesh: meshes.add(side),
          transform: Transform::from_xyz(-S2, 0., 0.),
          material: material.clone(),
          ..default()
        });
      })
      // back
      .with_children(|f| {
        f.spawn_bundle(PbrBundle {
          mesh: meshes.add(back),
          transform: Transform::from_xyz(0., 0., -S2),
          material: material.clone(),
          ..default()
        });
      })
      .with_children(|f| {
        f.spawn_bundle(PointLightBundle {
          point_light: PointLight {
            range: 100.,
            intensity: 10000.,
            shadows_enabled: true,
            ..Default::default()
          },
          transform: Transform::from_xyz(0., 0., 0.),
          ..default()
        });
      });
  }
}
