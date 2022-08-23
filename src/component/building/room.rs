use super::Coord;
use crate::*;

pub const MAX_SIZE: usize = 7;

#[derive(Default, Debug)]
pub struct Room {
  pub cells: Mutex<HashSet<Coord>>,
}
