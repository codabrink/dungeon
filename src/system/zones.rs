use crate::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use lazy_static::lazy_static;

const SIZE: f32 = 100.;
static WAIT: Duration = Duration::from_secs(5);

lazy_static! {
  static ref NAV: (Sender<Arc<NavNode>>, Receiver<Arc<NavNode>>) = unbounded();
  pub static ref NAV_TX: Sender<Arc<NavNode>> = NAV.0.clone();
  static ref NAV_RX: Receiver<Arc<NavNode>> = NAV.1.clone();
}

pub struct Zones {
  pub zones: HashMap<(i16, i16), Zone>,
  pub last_ran: Instant,
}

#[derive(Default)]
pub struct Zone {
  pub nav_nodes: HashSet<Arc<NavNode>>,
  pub entities: HashSet<Entity>,
  pub navigated: AtomicBool,
}

impl Zones {
  #[inline]
  pub fn transform_to_coord(t: &GlobalTransform) -> (i16, i16) {
    Self::translation_to_coord(&t.translation())
  }

  #[inline]
  pub fn translation_to_coord(t: &Vec3) -> (i16, i16) {
    ((t.z / SIZE) as i16, (t.x / SIZE) as i16)
  }

  // #[inline]
  // pub fn zone(&self, t: &Vec3) -> Option<&Zone> {
  // self.zones.entry(Self::translation_to_coord(t)).or_default()
  // }

  pub fn update(mut this: ResMut<Self>, query: Query<(Entity, &GlobalTransform)>) {
    if this.last_ran.elapsed() < WAIT {
      return;
    }
    this.last_ran = Instant::now();

    // clear out the entities
    for zone in this.zones.values_mut() {
      zone.entities = HashSet::new();
    }
    for (e, t) in query.iter() {
      this
        .zones
        .entry(Self::transform_to_coord(t))
        .or_default()
        .entities
        .insert(e);
    }

    // for (k, v) in &zones.entities {
    // println!("{:?}: {}", k, v.len());
    // }
  }
}

impl Default for Zones {
  fn default() -> Self {
    Self {
      zones: HashMap::default(),
      last_ran: Instant::now() - WAIT,
    }
  }
}
