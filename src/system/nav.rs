use crate::*;
use std::hash::{Hash, Hasher};

static NAV_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct NavNode {
  id: usize,
  pub adj: RwLock<HashSet<Arc<Self>>>,
  pub pos: Vec3,
  pub r#type: NavNodeType, // not really used now, but might be useful later
  pub area: Option<Rect>,
}

impl NavNode {
  pub fn new(
    pos: Vec3,
    r#type: NavNodeType,
    area: Option<Rect>,
    adj: HashSet<Arc<Self>>,
  ) -> Arc<Self> {
    Arc::new(Self {
      id: NAV_ID.fetch_add(1, Ordering::SeqCst),
      pos,
      r#type,
      area,
      adj: RwLock::new(adj),
    })
  }
}

impl PartialEq for NavNode {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
  fn ne(&self, other: &Self) -> bool {
    self.id != other.id
  }
}

impl Eq for NavNode {}

impl Hash for NavNode {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
  }
}

#[derive(Component)]
pub struct NavNodeComponent {
  pub node: Arc<NavNode>,
}

impl NavNodeComponent {
  pub fn fabricate(
    self,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
  ) -> Entity {
    commands
      .spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
          radius: 2.,
          subdivisions: 10,
        })),
        material: materials.add(StandardMaterial {
          base_color: Color::ORANGE,
          ..default()
        }),
        transform: Transform::from_translation(self.node.pos + Vec3::new(0., 3., 0.)),
        ..default()
      })
      .insert(self)
      .id()
  }
}

#[derive(Debug, Default)]
pub enum NavNodeType {
  #[default]
  Other,
  Cell,
  Door,
  Outside,
}
