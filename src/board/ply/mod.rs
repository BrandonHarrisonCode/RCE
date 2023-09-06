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

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derived_traits() {
        let start = Square::new("f4");
        let dest = Square::new("d6");
        let ply = Ply::new(start, dest);
        dbg!(&ply);

        assert_eq!(ply, ply.clone());
    }

    #[test]
    fn test_display() {
        let start = Square::new("f4");
        let dest = Square::new("d6");
        let ply = Ply::new(start, dest);

        let result = ply.to_string();
        let correct = format!("{} -> {}", start, dest);

        assert_eq!(result, correct);
    }
}
