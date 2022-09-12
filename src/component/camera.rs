use super::Player;
use bevy::{input::mouse::MouseWheel, prelude::*};

#[derive(Component)]
pub struct Camera {
  zoom: f32,
}

impl Camera {
  pub fn setup(mut commands: Commands) {
    commands
      .spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0., 20., 0.).looking_at(Vec3::ZERO, Vec3::X),
        ..Default::default()
      })
      .insert(Self { zoom: 100. });
  }

  pub fn follow_player(
    mut scroll_evr: EventReader<MouseWheel>,
    player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_transform: Query<(&mut Camera, &mut Transform), (With<Camera>, Without<Player>)>,
  ) {
    if player_transform.is_empty() {
      return;
    }

    let player_transform = player_transform.single();
    let (mut camera, mut camera_transform) = camera_transform.single_mut();

    for evt in scroll_evr.iter() {
      camera.zoom -= evt.y * 10.;
    }

    camera_transform.translation = camera_transform
      .translation
      .lerp(player_transform.translation + Vec3::Y * camera.zoom, 0.2);
  }
}
