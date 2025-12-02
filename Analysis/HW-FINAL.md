## Final Assignment - A* Search From Scratch


### Relevant Method Analysis

#### Annotate the relevant methods of your auxiliary structures with cost and times for each line.
##### (Relevant means used in the A* implementation.)

# Methods Used in `a_star()`

## WeightedDirectedGraph<T>
- `contains_vertex(&T)`
- `get_neighbors(T)`

## PriorityQueue<T>
- `new()`
- `push(T)`
- `pop()`
- `is_empty()`

## HashTable<K, V>
- `new()`
- `insert(K, V)`
- `get(&K)`

#### Note
The analysis below assumes that the generic type `T` provides only the minimal trait implementations required by the data structures used in `a_star()` (e.g., `Eq`, `Hash`, or cloning). Type K used for hashing is struct Location.


## WeightedDirectedGraph<T>

### contains_vertex(vertex)
```rs
pub fn contains_vertex(&self, vertex: &T) -> bool {                                                 
    self.adj_list.contains(vertex)                                                                  // C1: T = O(1)     Reason: Hash Lookup
}
```
Result: O(1)

##### See Hash Table .contains() @ HashTable<K, V> Below

### get_neighbors(vertex)
```rs
pub fn get_neighbors(&self, vertex: T) -> Vec<(T, f64)> {
    self.adj_list                                                                                   
        .get(&vertex)                                                                               // C1: T = O(1)     Reason: Hash Lookup
        .map(|v| v.clone())                                                                         // C2: T = O(k)     Reason: Neighbor count
        .unwrap_or_else(|_| Vec::new())                                                             // C3: T = O(1)     Reason: Unwrap operation and vector init both O(1)
}
```
Result: O(k) where k is v.neighbors

## PriorityQueue<T>

### new()
```rs
pub fn new() -> Self {
    PriorityQueue {
        heap: MinHeap { data: Vec::new() },                                                         // C1: T = O(1)     Reason: Simple Initialization.
    }
}
```
Result: O(1)

### push(value)
```rs
pub fn push(&mut self, value: T) {
    self.heap.push(value)                                                                           // C1: T = F(push()) = O(log n)
}
```
Worst: O(log n)
Average: O(log n)
Best: O(1)

##### Heap
```rs
pub fn push(&mut self, value: T) {
    self.data.push(value);                                                                          // C1: T = O(1)     Reason: Simple Operation
    self.bubble_up(self.data.len() - 1);                                                            // C2: T = F(bubble_up()) = O(log n)
}
```
Worst: O(log n)
Average: O(log n)
Best: O(1)


```rs
fn bubble_up(&mut self, mut idx: usize) {
    while idx > 0 {                                                                                 // C1: T = O(1..log n)   Reason: Height of heap
        let parent = (idx - 1) / 2;                                                                 // C2: T = O(1)     Reason: Simple Operation
        if self.data[idx] < self.data[parent] {                                                     // C3: T = O(1)     Reason: Simple Comparison
            self.data.swap(idx, parent);                                                            // C4: T = O(1)     Reason: Simple Operation
            idx = parent;                                                                           // C5: T = O(1)     Reason: Simple Operation
        } else {
            break;                                                                                  // C6: T = O(0..1)  Reason: Only when heap property aligns
        }
    }
}
```
Worst: O(log n)
Average: O(log n)
Best: O(1)

### pop()
```rs
pub fn pop(&mut self) -> Option<T> {
    self.heap.pop()                                                                                 // C1: T = O(F(pop()) = O(log n)
}
```
Worst: O(log n)
Average: O(log n)
Best: O(1)

##### Heap
```rs
pub fn pop(&mut self) -> Option<T> {
    if self.data.is_empty() {                                                                       // C1: T = O(1)  Reason: Simple check
        return None;                                                                                // C2: T = O(0..1)  Reason: Can happen at most once
    }
    let last = self.data.pop().unwrap();                                                            // C3: T = O(1)  Reason: Removing last element of Vec
    if self.data.is_empty() {                                                                       // C4: T = O(1)  Reason: Simple check
        return Some(last);                                                                          // C5: T = O(0..1)  Reason: Can happen at most once
    }
    let mut root = std::mem::replace(&mut self.data[0], last);                                      // C6: T = O(1)  Reason: Swap root with last element
    self.bubble_down(0);                                                                            // C7: T = F(bubble_down()) = O(log n)
    Some(root)                                                                                      // C1: T = O(1)  Reason: Simple return
}
```
Worst: O(log n)
Average: O(log n)
Best: O(1)

```rs
fn bubble_down(&mut self, mut idx: usize) {
    let len = self.data.len();                                                                      // C1: T = O(1)     Reason: Simple operation
    loop {                                                                                          // C2: T = O(1..log n)  Reason: At most height of heap
        let left = 2 * idx + 1;                                                                     // C3: T = O(1)     Reason: Simple operation
        let right = 2 * idx + 2;                                                                    // C4: T = O(1)     Reason: Simple operation

        if left >= len {                                                                            // C5: T = O(1)     Reason: Simple operation
            break;                                                                                  // C6: T = O(0..1)  Reason: Can happen at most once
        }

        let smallest =
            if right < len && self.data[right] < self.data[left] { right } else { left };           // C7: T = O(1) Reason: Comparison of two elements

        if self.data[smallest] < self.data[idx] {                                                   // C8: T = O(1)  Reason: Compare parent with child
            self.data.swap(idx, smallest);                                                          // C9: T = O(1)  Reason: Swap elements
            idx = smallest;                                                                         // C10: T = O(1) Reason: Assignment
        } else {
            break;                                                                                  // C11: T = O(0..1) Reason: Can happen at most once
        }
    }
}
```
Worst: O(log n)
Average: O(log n)
Best: O(1)

### is_empty()
```rs
pub fn is_empty(&self) -> bool {
    self.heap.is_empty()                                                                            // C1: T = F(is_empty()) = O(1)
}
```
Worst: O(1)
Average: O(1)
Best: O(1)

##### Heap
```rs
pub fn is_empty(&self) -> bool {
    self.data.is_empty()                                                                            // C1: T = O(1)     Reason: Simple len bit check
}
```
Worst: O(1)
Average: O(1)
Best: O(1)

## HashTable<K, V>

### new()
```rs
pub fn new() -> Self {
    let p = 2305843009213693951;                                                                    // C1: T = O(1)     Reason: Simple Var Creation
    let mut rng = rand::thread_rng();                                                               // C2: T = O(1)     Reason: Simple Var Creation
    let m = 101;                                                                                    // C3: T = O(1)     Reason: Simple Var Creation

    Self { 
        table: vec![Slot::Empty; m],                                                                // C4: T = O(m)     Reason: Initialize m Slots.
        a: rng.gen_range(1..p),                                                                     // C5: T = O(1)     Reason: Simple Constant Time Function.
        b: rng.gen_range(0..p),                                                                     // C6: T = O(1)     Reason: Simple Constant Time Function.
        p,                                                                                          // C7: T = O(1)     Reason: Simple Assignment.
        m,                                                                                          // C8: T = O(1)     Reason: Simple Assignment.
        len: 0,                                                                                     // C9: T = O(1)     Reason: Simple Assignment.
    }
}
```
Worst: O(m)
Average: O(m)
Best: O(m)

### universal_hash(key)
```rs
fn universal_hash(&self, k: &K) -> usize {
    let key = self.key_to_u64(k);                                                                   // C1: T = F(key_to_u64) = O(|Location|)
    let hash_val = (self.a.wrapping_mul(key).wrapping_add(self.b)) % self.p;                        // C2: T = O(1)     Reason: Simple operations
    (hash_val as usize) % self.m                                                                    // C3: T = O(1)     Reason: Simple operations
}
```
Worst: O(|Location|)
Average: O(|Location|)
Best: O(|Location|)

### key_to_u64(key)
```rs
fn key_to_u64(&self, key: &K) -> u64 {
    let mut hasher = DefaultHasher::new();                                                          // C1: T = O(1)     Reason: Simple struct initialization
    key.hash(&mut hasher);                                                                          // C2: T = O(|name|) + O(1) + O(1) aka O(|Location|)
                                                                                                    // Reason: This is type dependant. Since were hashing Location were
                                                                                                    // hashing the name of the location + long + lat
                                                                                                    
    hasher.finish()                                                                                 // C3: T = O(1)     Reason: Finalizes Hash
}
```
Worst: O(|Location|)
Average: O(|Location|)
Best: O(|Location|)

### insert(key, value)
```rs
pub fn insert(&mut self, key: K, value: V) -> Result<(), HashTableError> {
    let mut hash_key = self.universal_hash(&key);                                                   // C1: T = F(universal_hash) = O(|Location|)
    let start = hash_key;                                                                           // C2: T = O(1)     Reason: Simple assignment

    loop {                                                                                          // C3: T = O(1..m)      Reason: Atleast once, at most all m slots
        match &self.table[hash_key] {                                                               // C4: T = O(1)         Reason: Simple lookup operation
            Slot::Empty | Slot::Tombstone => {                                                      // C5: T = O(1)         Reason: Simple enum branch logic
                self.table[hash_key] = Slot::Occupied(key.clone(), value.clone());                  // C6: T = O(|Location| + d·|Location|) 
                                                                                                    // Reason: Cloning the key requires cloning a Location (O(|name|)),
                                                                                                    // and cloning the value requires cloning a Vec of d (Location, f64) 
                                                                                                    // pairs, each of which contains a Location (O(|name|)).
                self.len += 1;                                                                      // C7: T = O(1)      Reason: Simple Operation
                return Ok(());                                                                      // C8: T = O(1)      Reason: Simple Operation
            }
            Slot::Occupied(existing_key, _) => {                                                    // C9: T = O(1)         Reason: Simple enum branch logic
                if existing_key == &key {                                                           // C10: T = O(|Location|)        Reason: Comparison of Eq uses struct
                    self.table[hash_key] = Slot::Occupied(key.clone(), value.clone());              // C11: T = O(|Location| + d·|Location|) Reason: Refer to C6
                    return Ok(());                                                                  // C12: T = O(1)      Reason: Simple Operation
                }
                hash_key = (hash_key + 1) % self.m;                                                 // C13: T = O(1)      Reason: Simple Operatio
                if hash_key == start {                                                              // C14: T = O(|Location|)        Reason: Comparison of Eq uses struct
                    return Err(HashTableError::TableFull);                                          //  C14: T = O(0..1)      Reason: Can happen at most once
                }
            }
        }
    }
}
```
Worse: O(m · |Location| + d·|Location|)
Average: O((d + 1) · |Location|)
Best: O(|Location| + d·|Location|)

### get(key)
```rs
pub fn get(&self, key: &K) -> Result<&V, HashTableError> {
    let mut hash_key = self.universal_hash(key);                                                    // C1: T = F(universal_hash) = O(|Location|)
    let start = hash_key;                                                                           // C2: T = O(1)     Reason: Simple assignment

    loop {                                                                                          // C3: T = O(1..m)      Reason: At most all m slots
        match &self.table[hash_key] {                                                               // C4: T = O(1)         Reason: Simple lookup operation
            Slot::Occupied(existing_key, value) => {                                                // C5: T = O(1)         Reason: Simple enum branch logic
                if existing_key == key {                                                            // C6: T = O(|Location|)        Reason: Comparison of Eq uses struct
                    return Ok(value);                                                               // C7: T = O(0..1)      Reason: Can happen at most once
                }
                hash_key = (hash_key + 1) % self.m;                                                 // C8: T = O(1)      Reason: Simple Operation
                if hash_key == start {                                                              // C9: T = O(|Location|)        Reason: Comparison of Eq uses struct
                    return Err(HashTableError::TableFull);                                          // C10: T = O(0..1)      Reason: Can happen at most once
                }
            }
            Slot::Tombstone => {                                                                    // C11: T = O(1)         Reason: Simple enum branch logic
                hash_key = (hash_key + 1) % self.m;                                                 // C12: T = O(1)      Reason: Simple Operation
                if hash_key == start {                                                              // C13: T = O(|Location|)        Reason: Comparison of Eq uses struct
                    return Err(HashTableError::KeyNotFound);                                        // C14: T = O(0..1)      Reason: Can happen at most once
                }
            }
            Slot::Empty => {                                                                        // C15: T = O(1)         Reason: Simple enum branch logic
                return Err(HashTableError::KeyNotFound);                                            // C14: T = O(0..1)      Reason: Can happen at most once
            }
        }
    }
}
```
Worst: O(m · |Location|)
Average: Average: O(|Location|)
Best: O(|Location|)

### contains(key)
```rs
pub fn contains(&self, key: &K) -> bool {
    // Hash the key
    let mut hash_key = self.universal_hash(&key);                                                   // C1: T = F(universal_hash) = O(|Location|)
    let start = hash_key;                                                                           // C2: T = O(1)     Reason: Simple assignment

    // Begin Linear probing
    loop {                                                                                          // C3: T = O(1..m)      Reason: Atleast once, at most all m slots
        match &self.table[hash_key] {                                                               // C4: T = O(1)         Reason: Simple lookup operation
            Slot::Occupied(existing_key, _) => {                                                    // C5: T = O(1)         Reason: Simple enum branch logic
                if existing_key == key {                                                            // C6: T = O(|Location|)        Reason: Comparison of Eq uses struct
                    // Key found
                    return true;                                                                    // C7: T = O(0..1)      Reason: Can happen at most once
                }

                // Move to next slot
                hash_key = (hash_key + 1) % self.m;                                                 // C8: T = O(1)      Reason: Simple Operation

                // Checked all slots, table is full
                if hash_key == start {                                                              // C9: T = O(|Location|)        Reason: Comparison of Eq uses struct
                    return false;                                                                   // C10: T = O(0..1)      Reason: Can happen at most once
                }
            }
            Slot::Tombstone => {                                                                    // C11: T = O(1)         Reason: Simple enum branch logic
                // Move to next slot
                hash_key = (hash_key + 1) % self.m;                                                 // C12: T = O(1)      Reason: Simple Operation

                // Checked all slots, table is full
                if hash_key == start {                                                              // C13: T = O(|Location|)        Reason: Comparison of Eq uses struct
                    return false;                                                                   // C14: T = O(0..1)      Reason: Can happen at most once
                }
            }
            Slot::Empty => {                                                                        // C15: T = O(1)         Reason: Simple enum branch logic
                // Empty value not found
                return false;                                                                       // C16: T = O(0..1)      Reason: Can happen at most once
            }
        }
    }
}
```
Worst: O(m · |Location|)
Average: O(|Location|)
Best: O(|Location|)

#### Annotate your A* algorithm with cost and times for each line 
##### (For statements that call methods on your data structures, the costs will be those determined above.)

### a_star(graph, start, goal)
```rs
fn a_star(
    graph: &WeightedDirectedGraph<Location>,
    start: Location,
    goal: Location,
) -> Result<Vec<Location>, AStarError> {
    // Validate that start and goal exist in the graph
    if !graph.contains_vertex(&start) || !graph.contains_vertex(&goal) {
        return Err(AStarError::InvalidStartOrGoal);
    }

    // Create our queue frontier
    let mut frontier = PriorityQueue::<PriorityLocation>::new();

    // Add the first location with priority 0.0
    frontier.push(PriorityLocation {
        location: start.clone(),
        priority: OrderedFloat(0.0),
    });

    let mut came_from = HashTable::<Location, Location>::new();
    let mut cost_so_far = HashTable::<Location, f64>::new();

    // insert start cost = 0.0 (ignore insert error)
    cost_so_far.insert(start.clone(), 0.0).ok();

    while !frontier.is_empty() {
        let priority_loc = frontier.pop().ok_or(AStarError::NoPathFound)?;
        let current = priority_loc.location;

        // Goal reached
        if current == goal {
            return Ok(reconstruct_path(&came_from, start, goal));
        }

        // Explore neighbors
        let neighbors = graph.get_neighbors(current.clone());

        for (next, edge_cost) in neighbors {
            // current_cost: read from cost_so_far
            let current_cost = match cost_so_far.get(&current) {
                Ok(v) => *v,
                Err(_) => f64::INFINITY,
            };

            let new_cost = current_cost + edge_cost;

            // old_cost: if not present treat as +inf
            let old_cost = match cost_so_far.get(&next) {
                Ok(v) => *v,
                Err(_) => f64::INFINITY,
            };

            if new_cost < old_cost {
                // update cost_so_far and came_from (ignoring errors as not fatal here)
                cost_so_far.insert(next.clone(), new_cost).ok();
                came_from.insert(next.clone(), current.clone()).ok();

                // push
                frontier.push(PriorityLocation {
                    location: next.clone(),
                    priority: OrderedFloat(new_cost + heuristic(&goal, &next)),
                });
            }
        }
    }

    Err(AStarError::NoPathFound)
}
```
Result: TBD

### heuristic(goal, current)
```rs
fn heuristic(goal: &Location, current: &Location) -> f64 {
    let dx = (goal.lat as i64 - current.lat as i64).abs() as f64;                                   // C1: T = O(1)  Reason: Simple arithmetic
    let dy = (goal.long as i64 - current.long as i64).abs() as f64;                                 // C2: T = O(1)  Reason: Simple arithmetic
    (dx * dx + dy * dy).sqrt()                                                                      // C3: T = O(1)  Reason: Constant-time sqrt
}
```
Worst: O(1)
Average: O(1)
Best: O(1)

### reconstruct_path(came_from, start, goal)
```rs
fn reconstruct_path(
    came_from: &HashTable<Location, Location>,
    start: Location,
    goal: Location,
) -> Vec<Location> {
    let mut path = vec![goal.clone()];                                                              // C1: T = O(|Location|)  Reason: Clone Location
    let mut current = goal;                                                                         // C2: T = O(1)  Reason: Move operation

    while current != start {                                                                        // C3: T = O(1..p)  Reason: p = path length iterations
        let prev = came_from
            .get(&current)                                                                          // C4: T = O(|Location|)  Reason: Hash lookup
            .expect("Path should exist in came_from");                                              // C5: T = O(1)  Reason: Simple unwrap
        current = prev.clone();                                                                     // C6: T = O(|Location|)  Reason: Clone Location
        path.push(prev.clone());                                                                    // C7: T = O(|Location|)  Reason: Clone and push Location
    }

    path.reverse();                                                                                 // C8: T = O(p)  Reason: Reverse p elements
    path                                                                                            // C9: T = O(1)  Reason: Return operation
}
```
Worst: O(p · |Location|)
Average: O(p · |Location|)
Best: O(|Location|) 

#### Estimate best case, worst case, and expected running times for your A* implementation.
##### (Base this on your annotations and then summarize using asymptotic notation.)

### Best Case 

### Worst Case

### Average Case