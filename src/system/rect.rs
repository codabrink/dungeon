use rand::{thread_rng, Rng};

use crate::*;

#[derive(Debug)]
pub struct Rect {
  z_min: f32,
  z_max: f32,
  x_min: f32,
  x_max: f32,
  w: f32,
  h: f32,
}

pub struct RectBuilder {
  w: f32,
  h: f32,
}

impl Rect {
  pub fn new(z_min: f32, z_max: f32, x_min: f32, x_max: f32) -> Self {
    Self {
      z_min,
      z_max,
      x_min,
      x_max,
      w: z_max - z_min,
      h: x_max - x_min,
    }
  }

  pub fn build(w: f32, h: f32) -> RectBuilder {
    RectBuilder { w, h }
  }

  pub fn contains(&self, t: &Vec3) -> bool {
    t.z >= self.z_min && t.z <= self.z_max && t.x >= self.x_min && t.x <= self.x_max
  }

  fn center(&self) -> Vec3 {
    Vec3::new(
      (self.x_min + self.x_max) / 2.,
      0.,
      (self.z_min + self.z_max) / 2.,
    )
  }

  pub fn random(&self, padding: f32) -> Vec3 {
    if padding * 2. >= self.w && self.w <= self.h {
      return self.center();
    }
    let mut rng = thread_rng();
    let z = rng.gen_range((self.z_min + padding)..(self.z_max - padding));
    let x = rng.gen_range((self.x_min + padding)..(self.x_max - padding));
    Vec3::new(x, 0., z)
  }

  pub fn fabricate_debug_walls(
    &self,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
  ) {
    let c1 = Vec3::new(self.x_max, 0., self.z_min);
    let c2 = Vec3::new(self.x_max, 0., self.z_max);
    let c3 = Vec3::new(self.x_min, 0., self.z_max);
    let c4 = Vec3::new(self.x_min, 0., self.z_min);
    Wall::build(c1.clone(), c2.clone(), wall::State::Solid).fabricate(commands, meshes, materials);
    Wall::build(c2, c3.clone(), wall::State::Solid).fabricate(commands, meshes, materials);
    Wall::build(c3, c4.clone(), wall::State::Solid).fabricate(commands, meshes, materials);
    Wall::build(c4, c1, wall::State::Solid).fabricate(commands, meshes, materials);
  }
}

impl RectBuilder {
  pub fn enter_south_middle_at(&self, t: &Vec3) -> Rect {
    let x_min = t.x - CELL_SIZE_2;
    let z_min = t.z - self.w / 2.;

    Rect {
      x_min,
      x_max: x_min + self.h,
      z_min,
      z_max: z_min + self.w,
      w: self.w,
      h: self.h,
    }
  }

  pub fn center_at(&self, t: &Vec3) -> Rect {
    let x_min = t.x - self.h / 2.;
    let z_min = t.z - self.w / 2.;

    Rect {
      x_min,
      x_max: x_min + self.h,
      z_min,
      z_max: z_min + self.w,
      w: self.w,
      h: self.h,
    }
  }
}
