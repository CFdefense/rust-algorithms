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
The analysis below assumes that the generic type `T` provides only the minimal trait implementations required by the data structures used in `a_star()` (e.g., `Eq`, `Hash`, or cloning). Type K used for hashing is struct Location. Let `|L|` denote the cost of hashing or comparing a `Location` struct, which is proportional to the length of its `name` string. Let `n` be the number of items in the Priority Queue and `m` be the capacity of the HashTable.


## WeightedDirectedGraph<T>

### contains_vertex(vertex)
```rs
pub fn contains_vertex(&self, vertex: &T) -> bool {                                                 
    self.adj_list.contains(vertex)                                                                  // C1: T = O(|L|) on average. Reason: HashTable contains lookup.
}
```
Result: O(|L|) on average.

##### See Hash Table .contains() @ HashTable<K, V> Below

### get_neighbors(vertex)
```rs
pub fn get_neighbors(&self, vertex: T) -> Vec<(T, f64)> {
    self.adj_list                                                                                   
        .get(&vertex)                                                                               // C1: T = O(|L|) on average. Reason: HashTable get lookup.
        .map(|v| v.clone())                                                                         // C2: T = O(k * |L|)     Reason: Cloning k neighbor locations.
        .unwrap_or_else(|_| Vec::new())                                                             // C3: T = O(1)     Reason: Unwrap operation and vector init both O(1)
}
```
Result: O(k * |L|) on average, where k is the number of neighbors.

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
    self.heap.push(value)                                                                           // C1: T = O(log n) where n is the number of items in the heap.
}
```
Result: O(log n)

##### Heap
```rs
pub fn push(&mut self, value: T) {
    self.data.push(value);                                                                          // C1: T = O(1) amortized. Reason: Vector push.
    self.bubble_up(self.data.len() - 1);                                                            // C2: T = O(log n). Reason: Heapify up.
}
```
Result: O(log n)

```rs
fn bubble_up(&mut self, mut idx: usize) {
    while idx > 0 {                                                                                 // Loop runs at most log(n) times (height of heap).
        let parent = (idx - 1) / 2;                                                                 // C1: T = O(1).
        if self.data[idx] < self.data[parent] {                                                     // C2: T = O(1) for comparison.
            self.data.swap(idx, parent);                                                            // C3: T = O(1) for swap.
            idx = parent;
        } else {
            break;
        }
    }
}
```
Result: O(log n)

### pop()
```rs
pub fn pop(&mut self) -> Option<T> {
    self.heap.pop()                                                                                 // C1: T = O(log n) where n is the number of items in the heap.
}
```
Result: O(log n)

##### Heap
```rs
pub fn pop(&mut self) -> Option<T> {
    if self.data.is_empty() {                                                                       // C1: T = O(1)
        return None;
    }
    let last = self.data.pop().unwrap();                                                            // C2: T = O(1) amortized.
    if self.data.is_empty() {
        return Some(last);                                                                          // C3: T = O(1)
    }
    let mut root = std::mem::replace(&mut self.data[0], last);                                      // C4: T = O(1)
    self.bubble_down(0);                                                                            // C5: T = O(log n). Reason: Heapify down.
    Some(root)
}
```
Result: O(log n)

```rs
fn bubble_down(&mut self, mut idx: usize) {
    let len = self.data.len();                                                                      // C1: T = O(1)     Reason: Simple operation
    loop {                                                                                          // Loop runs at most log(n) times (height of heap).
        let left = 2 * idx + 1;                                                                     // C2: T = O(1)
        let right = 2 * idx + 2;                                                                    // C3: T = O(1)

        if left >= len {                                                                            // C4: T = O(1)
            break;
        }

        let smallest =                                                                              // C5: T = O(1)
            if right < len && self.data[right] < self.data[left] { right } else { left };

        if self.data[smallest] < self.data[idx] {                                                   // C6: T = O(1) for comparison
            self.data.swap(idx, smallest);                                                          // C7: T = O(1) for swap
            idx = smallest;
        } else {
            break;
        }
    }
}
```
Result: O(log n)

### is_empty()
```rs
pub fn is_empty(&self) -> bool {
    self.heap.is_empty()                                                                            // C1: T = O(1)
}
```
Result: O(1)

##### Heap
```rs
pub fn is_empty(&self) -> bool {
    self.data.is_empty()                                                                            // C1: T = O(1)     Reason: Simple len bit check
}
```
Result: O(1)


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
Result: O(m)

### universal_hash(key)
```rs
fn universal_hash(&self, k: &K) -> usize {
    let key = self.key_to_u64(k);                                                                   // C1: T = F(key_to_u64) = O(|L|)
    let hash_val = (self.a.wrapping_mul(key).wrapping_add(self.b)) % self.p;                        // C2: T = O(1)     Reason: Simple operations
    (hash_val as usize) % self.m                                                                    // C3: T = O(1)     Reason: Simple operations
}
```
Result: O(|L|)

### key_to_u64(key)
```rs
fn key_to_u64(&self, key: &K) -> u64 {
    let mut hasher = DefaultHasher::new();                                                          // C1: T = O(1)     Reason: Simple struct initialization
    key.hash(&mut hasher);                                                                          // C2: T = O(|L|)
                                                                                                    // Reason: Hashing a Location involves hashing its string name,
                                                                                                    // which is proportional to the name's length.
                                                                                                    
    hasher.finish()                                                                                 // C3: T = O(1)     Reason: Finalizes Hash
}
```
Result: O(|L|)

### insert(key, value)
```rs
pub fn insert(&mut self, key: K, value: V) -> Result<(), HashTableError> {
    let mut hash_key = self.universal_hash(&key);                                                   // C1: T = O(|L|)
    let start = hash_key;                                                                           // C2: T = O(1)

    loop {                                                                                          // C3: Runs until an empty slot is found. Average O(1), Worst O(m).
        match &self.table[hash_key] {                                                               // C4: T = O(1)
            Slot::Empty | Slot::Tombstone => {                                                      // C5: T = O(1)
                self.table[hash_key] = Slot::Occupied(key.clone(), value.clone());                  // C6: T = O(|L| + |V|) for cloning key and value.
                                                                                                    // For a_star, V is Location or f64, so this is O(|L|).
                self.len += 1;                                                                      // C7: T = O(1)
                return Ok(());                                                                      // C8: T = O(1)
            }
            Slot::Occupied(existing_key, _) => {                                                    // C9: T = O(1)
                if existing_key == &key {                                                           // C10: T = O(|L|) for comparison.
                    self.table[hash_key] = Slot::Occupied(key.clone(), value.clone());              // C11: T = O(|L| + |V|) for cloning.
                    return Ok(());                                                                  // C12: T = O(1)
                }
                hash_key = (hash_key + 1) % self.m;                                                 // C13: T = O(1)
                if hash_key == start {                                                              // C14: T = O(1)
                    return Err(HashTableError::TableFull);                                          // C15: T = O(1)
                }
            }
        }
    }
}
```
Worst: O(m * |L|) - Occurs when the table is full, and we must probe every slot. Each probe involves a comparison costing O(|L|).
Average: O(|L| + |V|) - Assuming a low load factor, we expect to find a slot in O(1) probes. The cost is dominated by the initial hash and the final insertion cloning.
Best: O(|L| + |V|) - Occurs on the first probe.

### get(key)
```rs
pub fn get(&self, key: &K) -> Result<&V, HashTableError> {
    let mut hash_key = self.universal_hash(key);                                                    // C1: T = O(|L|)
    let start = hash_key;                                                                           // C2: T = O(1)

    loop {                                                                                          // C3: Runs until key or empty slot is found. Average O(1), Worst O(m).
        match &self.table[hash_key] {                                                               // C4: T = O(1)
            Slot::Occupied(existing_key, value) => {                                                // C5: T = O(1)
                if existing_key == key {                                                            // C6: T = O(|L|) for comparison.
                    return Ok(value);                                                               // C7: T = O(1)
                }
                hash_key = (hash_key + 1) % self.m;                                                 // C8: T = O(1)
                if hash_key == start {                                                              // C9: T = O(1)
                    return Err(HashTableError::TableFull);                                          // C10: T = O(1)
                }
            }
            Slot::Tombstone => {                                                                    // C11: T = O(1)
                hash_key = (hash_key + 1) % self.m;                                                 // C12: T = O(1)
                if hash_key == start {                                                              // C13: T = O(1)
                    return Err(HashTableError::KeyNotFound);                                        // C14: T = O(1)
                }
            }
            Slot::Empty => {                                                                        // C15: T = O(1)
                return Err(HashTableError::KeyNotFound);                                            // C16: T = O(1)
            }
        }
    }
}
```
Worst: O(m * |L|) - When the table is full and the key is not present.
Average: O(|L|) - We expect O(1) probes, dominated by the hash and comparison.
Best: O(|L|) - When the key is found at the first probe.

### contains(key)
```rs
pub fn contains(&self, key: &K) -> bool {
    let mut hash_key = self.universal_hash(&key);                                                   // C1: T = O(|L|)
    let start = hash_key;                                                                           // C2: T = O(1)

    loop {                                                                                          // C3: Runs until key or empty slot is found. Average O(1), Worst O(m).
        match &self.table[hash_key] {                                                               // C4: T = O(1)
            Slot::Occupied(existing_key, _) => {                                                    // C5: T = O(1)
                if existing_key == key {                                                            // C6: T = O(|L|) for comparison.
                    return true;                                                                    // C7: T = O(1)
                }
                hash_key = (hash_key + 1) % self.m;                                                 // C8: T = O(1)
                if hash_key == start {                                                              // C9: T = O(1)
                    return false;                                                                   // C10: T = O(1)
                }
            }
            Slot::Tombstone => {                                                                    // C11: T = O(1)
                hash_key = (hash_key + 1) % self.m;                                                 // C12: T = O(1)
                if hash_key == start {                                                              // C13: T = O(1)
                    return false;                                                                   // C14: T = O(1)
                }
            }
            Slot::Empty => {                                                                        // C15: T = O(1)
                return false;                                                                       // C16: T = O(1)
            }
        }
    }
}
```
Result: Same as `get`: Worst O(m * |L|), Average O(|L|), Best O(|L|).

#### Annotate your A* algorithm with cost and times for each line 
##### (For statements that call methods on your data structures, the costs will be those determined above.)

### a_star(graph, start, goal)
Let V = number of vertices, E = number of edges.
```rs
fn a_star(
    graph: &WeightedDirectedGraph<Location>,
    start: Location,
    goal: Location,
) -> Result<Vec<Location>, AStarError> {
    // Validate that start and goal exist in the graph
    if !graph.contains_vertex(&start) || !graph.contains_vertex(&goal) {                            // C1: 2 * O(|L|) avg. = O(|L|)
        return Err(AStarError::InvalidStartOrGoal);
    }

    // Create our queue frontier
    let mut frontier = PriorityQueue::<PriorityLocation>::new();                                    // C2: O(1)

    // Add the first location with priority 0.0
    frontier.push(PriorityLocation {                                                               // C3: O(log V) + O(|L|) for clone
        location: start.clone(),
        priority: OrderedFloat(0.0),
    });

    let mut came_from = HashTable::<Location, Location>::new();                                     // C4: O(m)
    let mut cost_so_far = HashTable::<Location, f64>::new();                                        // C5: O(m)

    // insert start cost = 0.0 (ignore insert error)
    cost_so_far.insert(start.clone(), 0.0).ok();                                                    // C6: O(|L|) avg.

    while !frontier.is_empty() {                                                                    // C7: Loop runs V times in the worst case
        let priority_loc = frontier.pop().ok_or(AStarError::NoPathFound)?;                         // C8: O(log V)
        let current = priority_loc.location;

        // Goal reached
        if current == goal {                                                                        // C9: O(|L|) for comparison
            return Ok(reconstruct_path(&came_from, start, goal));                                   // C10: O(P * |L|) where P is path length
        }

        // Explore neighbors
        let neighbors = graph.get_neighbors(current.clone());                                       // C11: O(k*|L|) where k is num neighbors

        for (next, edge_cost) in neighbors {                                                        // C12: This inner loop runs E times in total over the whole algorithm
            // current_cost: read from cost_so_far
            let current_cost = match cost_so_far.get(&current) {                                    // C13: O(|L|) avg.
                Ok(v) => *v,
                Err(_) => f64::INFINITY,
            };

            let new_cost = current_cost + edge_cost;                                                // C14: O(1)

            // old_cost: if not present treat as +inf
            let old_cost = match cost_so_far.get(&next) {                                           // C15: O(|L|) avg.
                Ok(v) => *v,
                Err(_) => f64::INFINITY,
            };

            if new_cost < old_cost {                                                                // C16: O(1)
                // update cost_so_far and came_from (ignoring errors as not fatal here)
                cost_so_far.insert(next.clone(), new_cost).ok();                                    // C17: O(|L|) avg.
                came_from.insert(next.clone(), current.clone()).ok();                               // C18: O(|L|) avg.

                // push
                frontier.push(PriorityLocation {                                                    // C19: O(log V) + O(1) for heuristic
                    location: next.clone(),
                    priority: OrderedFloat(new_cost + heuristic(&goal, &next)),
                });
            }
        }
    }

    Err(AStarError::NoPathFound)                                                                    // C20: O(1)
}
```
Result: O(E * log V) on average. The main work is `V` pops `(V * log V)` and `E` pushes `(E * log V)`. This simplifies to `O(E * log V)`. The HashTable operations are `O(|L|)` on average, so a more precise bound is `O(E * log V + E * |L|)`.

### heuristic(goal, current)
```rs
fn heuristic(goal: &Location, current: &Location) -> f64 {
    let dx = (goal.lat as i64 - current.lat as i64).abs() as f64;                                    // C1: O(1)
    let dy = (goal.long as i64 - current.long as i64).abs() as f64;                                    // C2: O(1)
    (dx * dx + dy * dy).sqrt()                                                                      // C3: O(1)
}
```
Result: O(1)

### reconstruct_path(came_from, start, goal)
```rs
fn reconstruct_path(
    came_from: &HashTable<Location, Location>,
    start: Location,
    goal: Location,
) -> Vec<Location> {
    let mut path = vec![goal.clone()];                                                              // C1: O(|L|)
    let mut current = goal;                                                                         // C2: O(|L|) for clone

    while current != start {                                                                        // C3: Loop runs P times, where P is path length
        let prev = came_from
            .get(&current)                                                                          // C4: O(|L|) avg.
            .expect("Path should exist in came_from");
        current = prev.clone();                                                                     // C5: O(|L|)
        path.push(prev.clone());                                                                    // C6: O(|L|) + O(1) amortized
    }

    path.reverse();                                                                                 // C7: O(P * |L|)
    path
}
```
Result: O(P * |L|), where P is the number of nodes in the path.

#### Estimate best case, worst case, and expected running times for your A* implementation.
##### (Base this on your annotations and then summarize using asymptotic notation.)

### Best Case 
`O(P * log V)` where P is the length of the shortest path. This occurs when the heuristic is very accurate, and the algorithm explores only the nodes along the shortest path. Each of the `P` nodes is pushed and popped from the priority queue, and their neighbors are considered.

### Worst Case
`O(E * log V)`. This occurs when the heuristic is uninformative (e.g., h(n)=0 for all n), making A* equivalent to Dijkstra's algorithm. In this case, the algorithm may explore every edge and vertex in the graph. For dense graphs where `E` is approximately `V^2`, this becomes `O(V^2 * log V)`.

### Average Case
`O(E * log V)`. The practical performance depends heavily on the heuristic's quality, but the upper bound remains `O(E * log V)`. For good heuristics, it performs much closer to the best case. The cost of hash table operations adds a factor, making it more precisely `O(E * log V + E * |L|)`.