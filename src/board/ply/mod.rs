use derive_more::Constructor;
use std::fmt;

use super::square::Square;

#[derive(Constructor, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ply {
    pub start: Square,
    pub dest: Square,
}
impl fmt::Display for Ply {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.dest)
    }
}
