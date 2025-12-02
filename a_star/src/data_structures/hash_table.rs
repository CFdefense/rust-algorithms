/*
    HashTable
        use universal hashing
        resolve collisions using your choice of either chaining or open-addressing

    Note
        Each class must have a static function that performs a unit test of the class by instantiating and and calling the methods of the class.
*/

use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


#[derive(Clone)]
#[allow(dead_code)]
enum Slot<K, V> {
    Empty,          // Never used
    Tombstone,      // Deleted
    Occupied(K, V), // In-use
}


#[derive(Debug, PartialEq)]
pub enum HashTableError {
    TableFull,
    KeyNotFound,
}


pub struct HashTable<K, V> {
    table: Vec<Slot<K, V>>,
    a: u64,     // Randomly Generated Number
    b: u64,     // Randomly Generated Number
    p: u64,     // Number Larger Than Largest Key (Use prime 64 bit)
    m: usize,   // Max Table Size (Will use 101)
    len: usize, // Items in the table
}


impl<K: Hash + Clone + PartialEq, V: Clone> HashTable<K, V> {
    /// new()
    ///
    /// Creates a new instance of the HashTable.
    ///
    /// Returns an instance of the hashTable.
    ///
    pub fn new() -> Self {
        // Randomly Generate a and b
        let p = 2305843009213693951; // 2^61 - 1 (64 bit prime)
        let mut rng = rand::thread_rng();
        let m = 101;
        let len = 0;

        Self {
            table: vec![Slot::Empty; m],
            a: rng.gen_range(1..p), // must not be 0
            b: rng.gen_range(0..p),
            p,
            m,
            len,
        }
    }


    /// insert()
    ///
    /// Insert a new key into the hashTable.
    /// On collision use Linear Probing to find a slot.
    /// Tombstone values treated as Empty.
    ///
    /// Returns Result of success or error.
    ///
    pub fn insert(&mut self, key: K, value: V) -> Result<(), HashTableError> {
        // Hash the key
        let mut hash_key = self.universal_hash(&key);
        let start = hash_key;

        // Begin Linear probing
        loop {
            match &self.table[hash_key] {
                Slot::Empty | Slot::Tombstone => {
                    // Found a free slot
                    self.table[hash_key] = Slot::Occupied(key.clone(), value.clone());
                    self.len += 1;
                    return Ok(());
                }
                Slot::Occupied(existing_key, _) => {
                    if existing_key == &key {
                        // Key already exists, update value
                        self.table[hash_key] = Slot::Occupied(key.clone(), value.clone());
                        return Ok(());
                    }

                    // Move to next slot
                    hash_key = (hash_key + 1) % self.m;

                    // Checked all slots, table is full
                    if hash_key == start {
                        return Err(HashTableError::TableFull);
                    }
                }
            }
        }
    }


    /// get()
    ///
    /// Will hash the key and look for the Slot.
    /// If Slot Occupied and key matches we found it and return Value.
    /// If Slot found is Empty, the key does not exist.
    ///
    /// Returns the found value or an error.
    ///
    pub fn get(&self, key: &K) -> Result<&V, HashTableError> {
        // Hash the key
        let mut hash_key = self.universal_hash(key);
        let start = hash_key;

        // Begin Linear probing
        loop {
            match &self.table[hash_key] {
                Slot::Occupied(existing_key, value) => {
                    if existing_key == key {
                        // Key found, return the value
                        return Ok(value);
                    }

                    // Move to next slot
                    hash_key = (hash_key + 1) % self.m;

                    // Checked all slots
                    if hash_key == start {
                        return Err(HashTableError::TableFull);
                    }
                }
                Slot::Tombstone => {
                    // Move to next slot
                    hash_key = (hash_key + 1) % self.m;

                    // Checked all slots
                    if hash_key == start {
                        return Err(HashTableError::KeyNotFound);
                    }
                }
                Slot::Empty => {
                    // Empty value not found
                    return Err(HashTableError::KeyNotFound);
                }
            }
        }
    }


    /// get_mut()
    ///
    /// Mutable version of get()
    /// Will hash the key and look for the Slot.
    /// If Slot Occupied and key matches we found it and return Value.
    /// If Slot found is Empty, the key does not exist.
    ///
    /// Returns mutable value
    ///
    pub fn get_mut(&mut self, key: &K) -> Result<&mut V, HashTableError> {
        let mut hash_key = self.universal_hash(key);
        let start = hash_key;

        loop {
            match self.table[hash_key] {
                Slot::Empty => return Err(HashTableError::KeyNotFound),
                Slot::Occupied(ref existing_key, _) if existing_key == key => {
                    // We do a second match to get a mutable reference
                    match &mut self.table[hash_key] {
                        Slot::Occupied(_, value) => return Ok(value),
                        _ => unreachable!(),
                    }
                }
                _ => {
                    hash_key = (hash_key + 1) % self.m;
                    if hash_key == start {
                        return Err(HashTableError::KeyNotFound);
                    }
                }
            }
        }
    }


    /// remove()
    ///
    /// Will hash the key and look for the Slot.
    /// If slot is found, set it to Tombstone.
    /// If slot not found return Error.
    ///
    /// Returns Result of Success or Error.
    ///
    #[allow(dead_code)]
    pub fn remove(&mut self, key: &K) -> Result<(), HashTableError> {
        // Hash the key
        let mut hash_key = self.universal_hash(key);
        let start = hash_key;

        // Begin Linear probing
        loop {
            match &self.table[hash_key] {
                Slot::Occupied(existing_key, _) => {
                    if existing_key == key {
                        // Key found, remove the value
                        self.table[hash_key] = Slot::Tombstone;
                        self.len -= 1;
                        return Ok(());
                    }

                    // Move to next slot
                    hash_key = (hash_key + 1) % self.m;

                    // Checked all slots
                    if hash_key == start {
                        return Err(HashTableError::KeyNotFound);
                    }
                }
                Slot::Tombstone => {
                    // Move to next slot
                    hash_key = (hash_key + 1) % self.m;

                    // Checked all slots
                    if hash_key == start {
                        return Err(HashTableError::KeyNotFound);
                    }
                }
                Slot::Empty => {
                    // Empty value not found
                    return Err(HashTableError::KeyNotFound);
                }
            }
        }
    }


    /// contains()
    ///
    /// Will hash the key and look for the Slot.
    /// If slot is found, return True.
    /// If slot not found return False.
    ///
    /// Returns bool True if found, False otherwise.
    ///
    pub fn contains(&self, key: &K) -> bool {
        // Hash the key
        let mut hash_key = self.universal_hash(&key);
        let start = hash_key;

        // Begin Linear probing
        loop {
            match &self.table[hash_key] {
                Slot::Occupied(existing_key, _) => {
                    if existing_key == key {
                        // Key found
                        return true;
                    }

                    // Move to next slot
                    hash_key = (hash_key + 1) % self.m;

                    // Checked all slots, table is full
                    if hash_key == start {
                        return false;
                    }
                }
                Slot::Tombstone => {
                    // Move to next slot
                    hash_key = (hash_key + 1) % self.m;

                    // Checked all slots, table is full
                    if hash_key == start {
                        return false;
                    }
                }
                Slot::Empty => {
                    // Empty value not found
                    return false;
                }
            }
        }
    }


    /// len()
    ///
    /// Returns the total HashTable entries.
    ///
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.len
    }


    /// is_empty()
    ///
    /// Returns True if the HashTable entries are 0, False otherwise.
    ///
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }


    /// key_to_u64
    ///
    /// Converts a key of generic type K into a u64 using Rust's DefaultHasher.
    /// We do this because universal hashing requires the key to be a number,
    /// and since K can be anything (string, struct, etc.), we map it to a fixed
    /// 64-bit integer. This also ensures the key space is well-defined.
    ///
    /// Returns a u64 number.
    ///
    fn key_to_u64(&self, key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    
    /// universal_hash
    ///
    /// Hashes a key using our universal hashing function.
    ///
    /// Returns the hashed value
    ///
    fn universal_hash(&self, k: &K) -> usize {
        let key = self.key_to_u64(k);
        let hash_val = (self.a.wrapping_mul(key).wrapping_add(self.b)) % self.p;
        (hash_val as usize) % self.m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_table() {
        let mut table = HashTable::<i32, &str>::new();

        // Table should be empty initially
        assert!(table.is_empty());
        assert_eq!(table.len(), 0);

        // Insert a key-value pair
        assert!(table.insert(1, "one").is_ok());
        assert_eq!(table.len(), 1);
        assert!(!table.is_empty());

        // Insert another key-value
        assert!(table.insert(2, "two").is_ok());

        // Test get()
        assert_eq!(table.get(&1).unwrap(), &"one");
        assert_eq!(table.get(&2).unwrap(), &"two");

        // Test contains()
        assert!(table.contains(&1));
        assert!(table.contains(&2));
        assert!(!table.contains(&3));

        // Test updating existing key
        assert!(table.insert(1, "uno").is_ok());
        assert_eq!(table.get(&1).unwrap(), &"uno");

        // Test remove
        assert!(table.remove(&1).is_ok());
        assert!(!table.contains(&1));
        assert_eq!(table.remove(&1).unwrap_err(), HashTableError::KeyNotFound);

        // Test get for a non-existent key
        assert_eq!(table.get(&99).unwrap_err(), HashTableError::KeyNotFound);

        // Test for duplicate key
        assert!(table.insert(1, "uno").is_ok());
        assert!(table.insert(1, "dos").is_ok());
    }
}
