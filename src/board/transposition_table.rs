use std::sync::OnceLock;

use nohash_hasher::IntMap;

use super::{zkey::ZKey, Ply};

extern crate nohash_hasher;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Bounds {
    EXACT,
    LOWER,
    UPPER,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct TTEntry {
    score: i64,
    depth: u16,
    bound: Bounds,
    bestmove: Ply,
}

/// A hashmap that does no hashing to the ZKey.
pub type TranspositionTable = IntMap<ZKey, TTEntry>;

static TABLE: OnceLock<TranspositionTable> = OnceLock::new();

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
            bound: Bounds::EXACT,
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
            bound: Bounds::EXACT,
            bestmove: Ply::default(),
        };
        let entry1 = TTEntry {
            score: 3,
            depth: 4,
            bound: Bounds::LOWER,
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
