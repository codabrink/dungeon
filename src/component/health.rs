use crate::*;

#[derive(Component, Clone, Copy)]
pub struct Health {
  pub health: f32,
}

impl Default for Health {
  fn default() -> Self {
    Self { health: 1. }
  }
}

impl Health {
  #[inline]
  pub fn is_dead(&self) -> bool {
    self.health <= 0.
  }
}

#[derive(Component)]
pub struct Dead {
  timeout: Instant,
}
