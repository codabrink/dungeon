use crate::*;

const SIZE: f32 = 100.;
static WAIT: Duration = Duration::from_secs(5);

pub struct Zones {
  pub entities: HashMap<(i16, i16), HashSet<Entity>>,
  pub last_ran: Instant,
}

impl Default for Zones {
  fn default() -> Self {
    Self {
      entities: HashMap::default(),
      last_ran: Instant::now() - WAIT,
    }
  }
}

impl Zones {
  #[inline]
  fn coord(t: &GlobalTransform) -> (i16, i16) {
    let translation = t.translation();
    ((translation.z / SIZE) as i16, (translation.x / SIZE) as i16)
  }

  pub fn update(mut zones: ResMut<Self>, query: Query<(Entity, &GlobalTransform)>) {
    if zones.last_ran.elapsed() < WAIT {
      return;
    }

    zones.entities = HashMap::new();
    for (e, t) in query.iter() {
      zones.entities.entry(Self::coord(t)).or_default().insert(e);
    }
    // for (k, v) in &zones.entities {
    // println!("{:?}: {}", k, v.len());
    // }

    zones.last_ran = Instant::now();
  }
}
