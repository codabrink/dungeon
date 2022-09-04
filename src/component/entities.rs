use crate::*;
use lazy_static::lazy_static;

#[derive(Component, Default)]
pub struct Entity {
  asset: &'static str,
  colliders: Vec<(Collider, Transform)>,
  scale: f32,
  density: f32,
  point_lights: Vec<(PointLight, Transform)>,
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
          cbuild
            .spawn()
            .insert(col.clone())
            .insert_bundle(TransformBundle::from(*t));
        }

        for (point_light, transform) in &self.point_lights {
          cbuild
            .spawn_bundle(PointLightBundle {
              point_light: point_light.clone(),
              ..default()
            })
            .insert_bundle(TransformBundle::from(*transform));
        }
      });
  }
}

pub struct Entities {
  pub sofa: Entity,
  pub fridge: Entity,
  pub standing_lamp: Entity,
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
      ..default()
    },
    fridge: Entity {
      asset: "models/furniture.glb#Scene1",
      colliders: vec![(
        Collider::cuboid(2., 6., 2.),
        Transform::from_xyz(0., 6., 0.),
      )],
      scale: 1.,
      density: 0.,
      ..default()
    },
    standing_lamp: Entity {
      asset: "models/furniture.glb#Scene2",
      colliders: vec![(
        Collider::cuboid(2., 7., 2.),
        Transform::from_xyz(0., 7., 0.),
      )],
      scale: 1.,
      density: 0.,
      point_lights: vec![(
        PointLight {
          range: 30.,
          intensity: 5000.,
          // shadows_enabled: true,
          ..default()
        },
        Transform::from_xyz(0., 13., 0.),
      )],
    },
  }
}
