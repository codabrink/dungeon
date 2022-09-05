use crate::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use lazy_static::lazy_static;

const SIZE: f32 = 100.;
static WAIT: Duration = Duration::from_secs(5);

lazy_static! {
  static ref ZONE: (Sender<ZItem>, Receiver<ZItem>) = unbounded();
  pub static ref ZONE_TX: Sender<ZItem> = ZONE.0.clone();
}

pub enum ZItem {
  Building(Arc<Building>),
  Nav(Arc<NavNode>),
}

pub struct Zones {
  pub zones: HashMap<(i16, i16), Zone>,
  pub last_ran: Instant,
}

#[derive(Default)]
pub struct Zone {
  pub entities: HashSet<Entity>,
  pub nav_nodes: HashSet<Arc<NavNode>>,
  pub buildings: HashSet<Arc<Building>>,
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

  #[inline]
  pub fn zone(&self, t: &Vec3) -> Option<&Zone> {
    self.zones.get(&Self::translation_to_coord(t))
  }

  #[inline]
  fn zone_or_create_mut(&mut self, t: &Vec3) -> &mut Zone {
    self.zones.entry(Self::translation_to_coord(t)).or_default()
  }

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

    for item in ZONE.1.try_iter() {
      match item {
        ZItem::Building(b) => {
          let zone = this.zone_or_create_mut(&b.origin.translation);
          zone.buildings.insert(b);
        }
        ZItem::Nav(n) => {
          let zone = this.zone_or_create_mut(&n.pos);
          zone.nav_nodes.insert(n);
        }
      }
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
