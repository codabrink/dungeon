use crate::*;

#[derive(Component)]
pub struct Zombie {
  last_spot_update: Instant,
}

#[derive(Component)]
pub struct Aggressive;

impl Zombie {
  pub fn fabricate() {}

  pub fn update(
    zones: Res<Zones>,
    mut query: Query<(&Transform, &mut ExternalForce, &mut Self), Without<Aggressive>>,
  ) {
    for (t, mut ef, z) in query.iter_mut() {}
  }

  fn find_new_spot(&mut self, t: &Transform, zones: &Res<Zones>) {
    self.last_spot_update = Instant::now();

    let zone = zones.zone(&t.translation);
  }
}
