use super::{zkey::ZKey, Ply};
use crate::search::{Depth, Score};

extern crate nohash_hasher;

const BYTES_TO_MEGABYTES: u64 = 1024 * 1024;

pub const DEFAULT_SIZE_IN_MB: u64 = 8;
const BUCKET_SIZE: usize = 4;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Bounds {
    Exact,
    Lower,
    Upper,
    Invalid,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TTEntry {
    pub score: Score,
    pub depth: Depth,
    pub bound: Bounds,
    pub best_ply: Ply,
}

impl TTEntry {
    pub const fn new(score: Score, depth: Depth, bound: Bounds, best_ply: Ply) -> Self {
        Self {
            score,
            depth,
            bound,
            best_ply,
        }
    }

    /// Creates an invalid entry with default values.
    /// This entry is used to indicate that the entry is not valid or has not been set.
    ///
    /// # Returns
    ///
    /// A new `TTEntry` instance with "invalid" values.
    pub fn invalid() -> Self {
        Self {
            score: 0,
            depth: 0,
            bound: Bounds::Invalid,
            best_ply: Ply::default(),
        }
    }

    /// Checks if the entry is invalid.
    ///
    /// # Returns
    ///
    /// `true` if the entry is invalid, `false` otherwise.
    pub fn is_invalid(&self) -> bool {
        self.bound == Bounds::Invalid
    }
}

impl Ord for TTEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.depth.cmp(&other.depth)
    }
}

impl PartialOrd for TTEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BucketEntry {
    pub key: ZKey,
    pub entry: TTEntry,
}

impl BucketEntry {
    pub const fn new(key: ZKey, entry: TTEntry) -> Self {
        Self { key, entry }
    }

    pub fn invalid() -> Self {
        Self {
            key: ZKey::new(),
            entry: TTEntry::invalid(),
        }
    }

    /// Checks if the entry is invalid.
    ///
    /// # Returns
    ///
    /// `true` if the entry is invalid, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// let entry = BucketEntry::new(ZKey::new(), TTEntry::invalid());
    /// assert!(entry.is_invalid());
    /// ```
    pub fn is_invalid(&self) -> bool {
        self.entry.is_invalid()
    }
}

impl Ord for BucketEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.entry.depth.cmp(&other.entry.depth)
    }
}

impl PartialOrd for BucketEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy)]
pub struct Bucket([BucketEntry; BUCKET_SIZE]);

impl Bucket {
    pub fn new() -> Self {
        Self([BucketEntry::invalid(); BUCKET_SIZE])
    }

    /// Stores a new entry in the bucket.
    /// If the bucket is full, it will evict an entry based on a heuristic.
    ///
    /// # Arguments
    ///
    /// * `entry` - The entry to be stored in the bucket.
    ///
    /// # Returns
    ///
    /// An `Option` containing the evicted entry if the bucket was full, or `None` if the entry was stored successfully.
    ///
    /// # Example
    /// ```
    /// let mut bucket = Bucket::new();
    /// let entry = TTEntry::new(100, 5, Bounds::Exact, Ply::default());
    /// bucket.store(entry);
    /// ```
    pub fn insert(&mut self, entry: &BucketEntry) -> Option<BucketEntry> {
        let mut replace_idx = 0;

        for (i, bucket_entry) in self.0.iter().enumerate() {
            if bucket_entry.is_invalid() {
                self.0[i] = *entry;
                return None;
            }
            if bucket_entry < entry {
                replace_idx = i;
            }
        }

        let evicted_entry = self.0[replace_idx];
        self.0[replace_idx] = *entry;

        Some(evicted_entry)
    }

    /// Retrieves an entry from the bucket.
    /// If the entry is not found, it returns `None`.
    ///
    /// # Returns
    ///
    /// An `Option` containing the entry if found, or `None` if not found.
    ///
    /// # Example
    /// ```
    /// let mut bucket = Bucket::new();
    /// let entry = TTEntry::new(100, 5, Bounds::Exact, Ply::default());
    /// bucket.store(entry);
    /// let retrieved_entry = bucket.get();
    /// assert_eq!(retrieved_entry, Some(entry));
    /// ```
    pub fn get(&self, key: ZKey) -> Option<&TTEntry> {
        self.0
            .iter()
            .find(|entry| entry.key == key)
            .map(|entry| &entry.entry)
    }
}

pub struct TranspositionTable {
    pub table: Vec<Bucket>,
    pub entry_count: usize,
    pub bucket_count: usize,
}

impl TranspositionTable {
    /// Creates a new transposition table with the specified size in megabytes.
    ///
    /// # Arguments
    ///
    /// * `megabytes` - The size of the transposition table in megabytes.
    ///
    /// # Returns
    ///
    /// A new `TranspositionTable` instance with the specified size.
    ///
    /// # Example
    /// ```
    /// let ttable = TranspositionTable::with_size(8);
    /// ```
    pub fn with_size(megabytes: u64) -> Self {
        let bucket_count = Self::calculate_size(megabytes);

        Self {
            table: vec![Bucket::new(); bucket_count],
            entry_count: 0,
            bucket_count,
        }
    }

    /// Resizes the transposition table to the specified size in megabytes.
    /// This will clear the existing entries in the table.
    ///
    /// # Arguments
    ///
    /// * `megabytes` - The new size of the transposition table in megabytes.
    ///
    /// # Example
    /// ```
    /// let mut ttable = TranspositionTable::default();
    /// ttable.resize(16);
    /// ```
    pub fn resize(&mut self, megabytes: u64) {
        let bucket_count = Self::calculate_size(megabytes);

        self.table = vec![Bucket::new(); bucket_count];
        self.entry_count = 0;
        self.bucket_count = bucket_count;
    }

    /// Calculates the size of the transposition table based on the specified size in megabytes.
    ///
    /// # Arguments
    ///
    /// * `megabytes` - The size of the transposition table in megabytes.
    ///
    /// # Returns
    ///
    /// The number of buckets needed to store the transposition table.
    ///
    /// # Example
    /// ```
    /// let bucket_count = TranspositionTable::calculate_size(8);
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    const fn calculate_size(megabytes: u64) -> usize {
        let entry_size = std::mem::size_of::<TTEntry>();
        let bucket_size = entry_size * BUCKET_SIZE;

        (megabytes * BYTES_TO_MEGABYTES) as usize / bucket_size
    }

    /// Inserts a new entry into the transposition table.
    /// Potentially evicts an existing entry if the table is full.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the entry.
    /// * `entry` - The entry to be inserted.
    ///
    /// # Example
    /// ```
    /// let mut ttable = TranspositionTable::default();
    /// let entry = TTEntry::new(100, 5, Bounds::Exact, Ply::default());
    /// ttable.insert(ZKey::new(), entry);
    /// ```
    pub fn insert(&mut self, key: ZKey, entry: TTEntry) {
        let bucket_index = self.calculate_bucket_index(key);

        if self.table[bucket_index]
            .insert(&BucketEntry::new(key, entry))
            .is_none()
        {
            self.entry_count += 1;
        }
    }

    /// Retrieves an entry from the transposition table based on the provided key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key associated with the entry.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the entry if found, or `None` if not found.
    ///
    /// # Example
    /// ```
    /// let mut ttable = TranspositionTable::default();
    /// let entry = TTEntry::new(100, 5, Bounds::Exact, Ply::default());
    /// ttable.insert(ZKey::new(), entry);
    /// let retrieved_entry = ttable.get(&ZKey::new());
    /// assert_eq!(retrieved_entry, Some(&entry));
    /// ```
    pub fn get(&self, key: ZKey) -> Option<&TTEntry> {
        if self.bucket_count == 0 {
            return None;
        }

        let bucket_index = self.calculate_bucket_index(key);
        self.table[bucket_index].get(key)
    }

    /// Calculates the bucket index for a given key.
    /// This is used to determine where to store or retrieve entries in the table.
    ///
    /// # Arguments
    ///
    /// * `key` - The key for which to calculate the bucket index.
    ///
    /// # Returns
    ///
    /// The index of the bucket where the entry should be stored or retrieved.
    ///
    /// # Example
    /// ```
    /// let ttable = TranspositionTable::default();
    /// let bucket_index = ttable.calculate_bucket_index(ZKey::new());
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    fn calculate_bucket_index(&self, key: ZKey) -> usize {
        (u64::from(key) % self.bucket_count as u64) as usize
    }

    /// Checks if the transposition table contains an entry for the specified key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check for existence in the table.
    ///
    /// # Returns
    ///
    /// `true` if the key exists in the table, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// let mut ttable = TranspositionTable::default();
    /// let entry = TTEntry::new(100, 5, Bounds::Exact, Ply::default());
    /// ttable.insert(ZKey::new(), entry);
    /// assert!(ttable.contains_key(&ZKey::new()));
    /// ```
    #[allow(dead_code)]
    pub fn contains_key(&self, key: ZKey) -> bool {
        self.get(key).is_some()
    }

    /// Clears the transposition table, removing all entries.
    ///
    /// # Example
    /// ```
    /// let mut ttable = TranspositionTable::default();
    /// let entry = TTEntry::new(100, 5, Bounds::Exact, Ply::default());
    /// ttable.insert(ZKey::new(), entry);
    /// assert!(!ttable.is_empty());
    /// ttable.clear();
    /// assert!(ttable.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.table = vec![Bucket::new(); self.bucket_count];
        self.entry_count = 0;
    }

    /// Checks if the transposition table is empty.
    ///
    /// # Returns
    ///
    /// `true` if the table is empty, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// let ttable = TranspositionTable::default();
    /// assert!(ttable.is_empty());
    /// ```
    #[allow(dead_code)]
    pub const fn is_empty(&self) -> bool {
        self.entry_count == 0
    }

    /// Returns the number of entries in the transposition table.
    ///
    /// # Returns
    ///
    /// The number of entries in the table.
    ///
    /// # Example
    /// ```
    /// let mut ttable = TranspositionTable::default();
    /// let entry = TTEntry::new(100, 5, Bounds::Exact, Ply::default());
    /// ttable.insert(ZKey::new(), entry);
    /// assert_eq!(ttable.len(), 1);
    /// ```
    #[allow(dead_code)]
    pub const fn len(&self) -> usize {
        self.entry_count
    }

    /// Returns the capacity used in the transposition table in permille.
    ///
    /// # Returns
    ///
    /// The capacity used in the table in permille.
    ///
    /// # Example
    /// ```
    /// let ttable = TranspositionTable::default();
    /// assert_eq!(ttable.capacity_used(), 0);
    /// ```
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    pub fn capacity_used(&self) -> u16 {
        if self.entry_count == 0 {
            return 0;
        }
        ((self.entry_count as f64 * 1000f64) / (self.bucket_count as f64 * BUCKET_SIZE as f64))
            .floor() as u16
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::with_size(DEFAULT_SIZE_IN_MB)
    }
}

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
            best_ply: Ply::default(),
        };

        assert!(ttable.is_empty());
        ttable.insert(ZKey::new(), entry);
        assert!(!ttable.is_empty());
        assert!(ttable.contains_key(ZKey::new()));
    }

    #[test]
    fn test_ttable_overwrite() {
        let mut ttable: TranspositionTable = TranspositionTable {
            table: vec![Bucket::new(); 1],
            entry_count: 0,
            bucket_count: 1,
        };

        let zkey0 = ZKey::new();
        let mut zkey1 = ZKey::new();
        zkey1.change_turn();

        let entry0 = TTEntry {
            score: 1,
            depth: 2,
            bound: Bounds::Exact,
            best_ply: Ply::default(),
        };
        let entry1 = TTEntry {
            score: 3,
            depth: 4,
            bound: Bounds::Lower,
            best_ply: Ply::default(),
        };

        assert!(ttable.is_empty());
        for i in 0..BUCKET_SIZE {
            ttable.insert(zkey0, entry0);
            assert!(!ttable.is_empty());
            assert_eq!(ttable.len(), i + 1);
        }

        assert!(ttable.contains_key(ZKey::new()));
        assert_eq!(ttable.get(zkey0).unwrap(), &entry0);

        ttable.insert(zkey1, entry1);
        assert_eq!(ttable.len(), BUCKET_SIZE);
        assert_eq!(ttable.get(zkey1).unwrap(), &entry1);
        assert_eq!(ttable.get(zkey0).unwrap(), &entry0);
    }
}
