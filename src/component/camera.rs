use super::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct Camera;

impl Camera {
  pub fn setup(mut commands: Commands) {
    commands
      .spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0., 20., 0.).looking_at(Vec3::ZERO, Vec3::X),
        ..Default::default()
      })
      .insert(Self);
  }

  pub fn follow_player(
    player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_transform: Query<&mut Transform, (With<Camera>, Without<Player>)>,
  ) {
    let player_transform = player_transform.single();
    let mut camera_transform = camera_transform.single_mut();

    camera_transform.translation = camera_transform
      .translation
      .lerp(player_transform.translation + Vec3::Y * 100., 0.2);
  }
}
