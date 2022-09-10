use crate::*;

#[derive(Component, Clone, Copy)]
pub struct Health {
  health: f32,
  color: Color,
  changed: bool,
}

impl Health {
  pub fn new(color: Color) -> Self {
    Self {
      health: 1.,
      color,
      changed: false,
    }
  }

  #[inline]
  pub fn is_dead(&self) -> bool {
    self.health <= 0.
  }

  pub fn damage(&mut self, amt: f32) {
    self.health -= amt;
    self.changed = true;
  }

  pub fn is_changed(&self) -> bool {
    self.changed
  }

  pub fn reset_changed(&mut self) -> bool {
    let r = self.changed;
    self.changed = false;
    r
  }

  #[inline]
  pub fn health(&self) -> f32 {
    self.health
  }

  pub fn color(&self) -> Color {
    Color::rgb(
      self.color.r() * self.health,
      self.color.g() * self.health,
      self.color.b() * self.health,
    )
  }
}

#[derive(Component)]
pub struct Dead {
  timeout: Instant,
}
