use bevy::{core::Zeroable, prelude::*};
use bevy_rapier3d::{prelude::*, rapier::prelude::ColliderBuilder};

#[derive(Component)]
pub struct Tree;

impl Tree {
  pub fn setup(mut commands: Commands, ass: Res<AssetServer>) {
    let tree = ass.load("models.glb#Scene0");

    let collider = ColliderBuilder::cylinder(5., 2.)
      .translation(Vec3::new(0., 2.5, 0.).into())
      .build();
    let ball = ColliderBuilder::ball(6.)
      .translation(Vec3::new(0., 10., 0.).into())
      .build();

    commands
      .spawn_bundle(SceneBundle {
        scene: tree,
        transform: Transform::from_xyz(6., 2., 0.).with_scale(Vec3::splat(3.)),
        ..default()
      })
      .insert(RigidBody::Fixed)
      .with_children(|children| {
        children
          .spawn()
          .insert(Collider::ball(2.5))
          .insert_bundle(TransformBundle::from(Transform::from_xyz(0., 6., 0.)));
        children
          .spawn()
          .insert(Collider::cylinder(2., 0.5))
          .insert(Transform::from_xyz(0., 1.25, 0.));
      });
  }
}
