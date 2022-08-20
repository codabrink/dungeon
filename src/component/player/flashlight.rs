use bevy::prelude::*;

#[derive(Component)]
pub struct Flashlight;

impl Flashlight {
  pub fn setup(commands: &mut ChildBuilder, mut meshes: ResMut<Assets<Mesh>>) {
    const S: f32 = 2.;
    const S2: f32 = S / 2.;
    let top = Mesh::from(shape::Box::new(S, 0.1, S));
    let side = Mesh::from(shape::Box::new(0.1, S, S));
    let back = Mesh::from(shape::Box::new(S, S, 0.1));

    commands
      .spawn()
      // top
      .insert_bundle(PbrBundle {
        mesh: meshes.add(top.clone()),
        transform: Transform::from_xyz(0., S2, S2),
        ..default()
      })
      .with_children(|f| {
        f.spawn_bundle(PointLightBundle {
          point_light: PointLight {
            range: 12000.,
            intensity: 10000.,
            shadows_enabled: true,
            ..Default::default()
          },
          transform: Transform::from_xyz(0., 0., 0.),
          ..default()
        });
      })
      // bottom
      .with_children(|f| {
        f.spawn_bundle(PbrBundle {
          mesh: meshes.add(top),
          transform: Transform::from_xyz(0., -S2, 0.),
          ..default()
        });
      })
      // left
      .with_children(|f| {
        f.spawn_bundle(PbrBundle {
          mesh: meshes.add(side.clone()),
          transform: Transform::from_xyz(S2, 0., 0.),
          ..default()
        });
      })
      // right
      .with_children(|f| {
        f.spawn_bundle(PbrBundle {
          mesh: meshes.add(side),
          transform: Transform::from_xyz(-S2, 0., 0.),
          ..default()
        });
      })
      // back
      .with_children(|f| {
        f.spawn_bundle(PbrBundle {
          mesh: meshes.add(back),
          transform: Transform::from_xyz(0., 0., -S2),
          ..default()
        });
      });
  }
}
