use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod flashlight;

#[derive(Component)]
pub struct Player {
  pub angle: f32,
}

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
        ..default()
      })
      .with_children(|player| {
        player.spawn_bundle(PbrBundle {
          mesh: meshes.add(Self::gun_mesh()),
          material: materials.add(Self::gun_material()),
          transform: Transform::from_xyz(-1., 0., 0.75),
          ..default()
        });
      })
      .with_children(|player| {
        flashlight::Flashlight::setup(player, meshes, materials);
      })
      .insert(RigidBody::Dynamic)
      .insert(ExternalForce::default())
      .insert(Velocity::default())
      .insert(Damping {
        linear_damping: 10.,
        angular_damping: 1.,
      })
      // .insert(GravityScale(0.))
      .insert(Collider::cuboid(rad / 2., rad / 2., rad / 2.))
      .insert(ColliderMassProperties::Density(6.))
      .insert(Restitution {
        coefficient: 0.,
        combine_rule: CoefficientCombineRule::Min,
      })
      .insert(Self { angle: 0. });
  }

  pub fn update(
    input: Res<Input<KeyCode>>,
    window: Res<Windows>,
    mut query: Query<(&Velocity, &mut ExternalForce, &mut Transform, &mut Player)>,
  ) {
    if input.pressed(KeyCode::Q) {
      std::process::exit(1);
    }

    let (_vel, mut force, mut pos, mut player) = query.single_mut();
    let up = input.any_pressed([KeyCode::W, KeyCode::Up]);
    let down = input.any_pressed([KeyCode::S, KeyCode::Down]);
    let left = input.any_pressed([KeyCode::A, KeyCode::Left]);
    let right = input.any_pressed([KeyCode::D, KeyCode::Right]);
    let x = (-(down as i8) + up as i8) as f32;
    let z = (-(left as i8) + right as i8) as f32;

    let scale = 25000.;
    force.force = Vec3::new(x * scale, 0., z * scale);

    let window = window.get_primary().unwrap();
    if let Some(cur_pos) = window.cursor_position() {
      let cx = window.width() / 2.;
      let cy = window.height() / 2.;

      let angle = (cur_pos.y - cy).atan2(cur_pos.x - cx);
      player.angle = angle;
      pos.rotation = Quat::from_axis_angle(Vec3::Y, angle);
    }
  }

  fn angle_from_velocity(vel: &Velocity) -> Option<f32> {
    if vel.linvel.length() > 1. {
      let angle = vel.linvel.x.atan2(vel.linvel.z);
      if !angle.is_nan() {
        return Some(angle);
      }
    }
    None
  }

  fn material() -> StandardMaterial {
    StandardMaterial {
      base_color: Color::rgb(0.0, 1., 0.0),
      unlit: true,
      ..default()
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
      ..default()
    }
  }
}
