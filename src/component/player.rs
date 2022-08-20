use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Player;

impl Player {
  pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
  ) {
    let rad = 2.;

    commands
      .spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: rad })),
        material: materials.add(Self::material()),
        transform: Transform::from_xyz(0., rad, 0.),
        ..Default::default()
      })
      .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
          mesh: meshes.add(Self::gun_mesh()),
          material: materials.add(Self::gun_material()),
          transform: Transform::from_xyz(-1., 0., 0.75),
          ..Default::default()
        });
      })
      .insert(RigidBody::Dynamic)
      .insert(ExternalForce::default())
      .insert(Velocity::default())
      .insert(Damping {
        linear_damping: 0.5,
        angular_damping: 1.,
      })
      .insert(GravityScale(0.))
      .insert(Collider::cuboid(rad, rad, rad))
      .insert(Self);
  }

  pub fn update(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Velocity, &mut ExternalForce, &mut Transform)>,
  ) {
    let (vel, mut force, mut pos) = query.single_mut();
    let up = input.any_pressed([KeyCode::W, KeyCode::Up]);
    let down = input.any_pressed([KeyCode::S, KeyCode::Down]);
    let left = input.any_pressed([KeyCode::A, KeyCode::Left]);
    let right = input.any_pressed([KeyCode::D, KeyCode::Right]);
    let x = (-(down as i8) + up as i8) as f32;
    let z = (-(left as i8) + right as i8) as f32;

    let scale = 5000.;
    force.force = Vec3::new(x * scale, 0., z * scale);

    let angle = vel.linvel.x.atan2(vel.linvel.z);
    if !angle.is_nan() {
      pos.rotation = Quat::from_axis_angle(Vec3::Y, angle);
    }
  }

  fn material() -> StandardMaterial {
    StandardMaterial {
      base_color: Color::rgb(0.0, 1., 0.0),
      unlit: true,
      ..Default::default()
    }
  }

  fn gun_mesh() -> Mesh {
    Mesh::from(shape::Box {
      min_x: -0.5,
      max_x: 0.5,
      min_y: -0.5,
      max_y: 0.5,
      min_z: -1.2,
      max_z: 1.2,
    })
  }

  fn gun_material() -> StandardMaterial {
    StandardMaterial {
      base_color: Color::rgb(0.5, 0.5, 0.5),
      unlit: true,
      ..Default::default()
    }
  }
}
