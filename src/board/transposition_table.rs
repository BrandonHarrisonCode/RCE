use std::sync::RwLock;

use nohash_hasher::IntMap;

use super::{zkey::ZKey, Ply};

extern crate nohash_hasher;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Bounds {
    Exact,
    Lower,
    Upper,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TTEntry {
    pub score: i64,
    pub depth: u16,
    pub bound: Bounds,
    pub bestmove: Ply,
}

/// A hashmap that does no hashing to the `ZKey`.
pub type TranspositionTable = IntMap<ZKey, TTEntry>;

pub static TRANSPOSITION_TABLE: RwLock<TranspositionTable> = RwLock::new(
    TranspositionTable::with_hasher(nohash_hasher::BuildNoHashHasher::new()),
);

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_ttable_entry() {
        let mut ttable: TranspositionTable = TranspositionTable::default();

        let entry = TTEntry {
            score: 1,
            depth: 2,
            bound: Bounds::Exact,
            bestmove: Ply::default(),
        };

        assert!(ttable.is_empty());
        ttable.insert(ZKey::new(), entry);
        assert!(!ttable.is_empty());
        assert!(ttable.contains_key(&ZKey::new()));
    }

    #[test]
    fn test_ttable_overwrite() {
        let mut ttable: TranspositionTable = TranspositionTable::default();

        let entry0 = TTEntry {
            score: 1,
            depth: 2,
            bound: Bounds::Exact,
            bestmove: Ply::default(),
        };
        let entry1 = TTEntry {
            score: 3,
            depth: 4,
            bound: Bounds::Lower,
            bestmove: Ply::default(),
        };

        assert!(ttable.is_empty());
        ttable.insert(ZKey::new(), entry0);
        assert!(!ttable.is_empty());
        assert_eq!(ttable.len(), 1);

        assert!(ttable.contains_key(&ZKey::new()));
        assert_eq!(ttable.get(&ZKey::new()).unwrap(), &entry0);

        ttable.insert(ZKey::new(), entry1);
        assert_eq!(ttable.len(), 1);
        assert_eq!(ttable.get(&ZKey::new()).unwrap(), &entry1);
    }
}
