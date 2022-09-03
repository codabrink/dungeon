use bevy::{
  asset::AssetLoader,
  gltf::{self, GltfLoader},
  prelude::*,
  reflect::erased_serde::private::serde::__private::de,
};
use bevy_rapier3d::{prelude::*, rapier::prelude::ColliderBuilder};
use lazy_static::lazy_static;

#[derive(Component)]
pub struct Entity {
  asset: &'static str,
  colliders: Vec<(Collider, Transform)>,
  scale: f32,
  density: f32,
}

impl Entity {
  pub fn spawn(&self, mut transform: Transform, commands: &mut Commands, ass: &Res<AssetServer>) {
    let scene = ass.load(self.asset);
    transform.scale *= Vec3::splat(self.scale);

    commands
      .spawn_bundle(SceneBundle {
        scene,
        transform,
        ..default()
      })
      .insert(RigidBody::Dynamic)
      .insert(ColliderMassProperties::Density(self.density))
      .with_children(|cbuild| {
        for (col, t) in &self.colliders {
          let mut t = t.clone();
          // t.scale = Vec3::splat(self.scale);

          cbuild
            .spawn()
            .insert(col.clone())
            .insert_bundle(TransformBundle::from(t));
        }
      });
  }
}

pub struct Entities {
  pub sofa: Entity,
  pub fridge: Entity,
}

lazy_static! {
  pub static ref ENTITIES: Entities = init_entities();
}

fn init_entities() -> Entities {
  Entities {
    sofa: Entity {
      asset: "models/furniture.glb#Scene0",
      colliders: vec![(
        Collider::cuboid(3.75, 4., 8.),
        Transform::from_xyz(0., 4., 0.),
      )],
      scale: 0.9,
      density: 0.,
    },
    fridge: Entity {
      asset: "models/furniture.glb#Scene1",
      colliders: vec![(
        Collider::cuboid(0.5, 0.5, 1.1),
        Transform::from_xyz(0., 0.5, 0.),
      )],
      scale: 1.,
      density: 0.,
    },
  }
}
