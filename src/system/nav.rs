use rand::{thread_rng, Rng};

use crate::*;
use std::hash::{Hash, Hasher};

static NAV_ID: AtomicUsize = AtomicUsize::new(0);

pub struct Navigator {
  traversed: HashSet<usize>,
  dest: Arc<NavNode>,
  pub path: Vec<Arc<NavNode>>,
}

impl Navigator {
  pub fn new(from: &Arc<NavNode>, to: &Arc<NavNode>) -> Self {
    Self {
      traversed: HashSet::new(),
      dest: to.clone(),
      path: vec![from.clone()],
    }
  }

  pub fn nav(&mut self) {
    let last = match self.path.last() {
      Some(last) if last.id == self.dest.id => return, // we made it
      Some(last) => last,
      None => {
        // no path?
        println!("No path?");
        return;
      }
    };

    let mut choice = None;
    let mut choice_dist = f32::MAX;
    for adj in &*last.adj.read() {
      if self.traversed.contains(&adj.id) {
        continue;
      }

      let dist = adj.pos.distance(self.dest.pos);
      if dist < choice_dist {
        choice = Some(adj.clone());
        choice_dist = dist;
      }
    }

    if let Some(choice) = choice {
      self.traversed.insert(choice.id);
      self.path.push(choice);
      self.nav();
    } else {
      // no new choice found - walk it back
      self.path.pop();
    }
  }
}

#[derive(Debug)]
pub struct NavNode {
  id: usize,
  pub adj: RwLock<HashSet<Arc<Self>>>,
  pub pos: Vec3,
  pub r#type: NavNodeType, // not really used now, but might be useful later
  pub area: Rect,
}

impl NavNode {
  pub fn new(pos: Vec3, r#type: NavNodeType, area: Rect, adj: HashSet<Arc<Self>>) -> Arc<Self> {
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
    let mut rng = thread_rng();
    let r: u8 = rng.gen();
    let g: u8 = rng.gen();
    let b: u8 = rng.gen();
    let t = self.node.pos;

    let var = 1.;

    let line_up = Vec3::new(rng.gen_range(-var..var), 8., rng.gen_range(-var..var));

    for adj in &*self.node.adj.read() {
      DebugLine::build(t + line_up, adj.pos + line_up, Color::rgb_u8(r, g, b))
        .fabricate(commands, meshes, materials);
    }

    commands
      .spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
          radius: 2.,
          subdivisions: 10,
        })),
        material: materials.add(StandardMaterial {
          base_color: Color::rgb_u8(r, g, b),
          unlit: true,
          ..default()
        }),
        transform: Transform::from_translation(t + Vec3::new(0., 3., 0.)),
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
