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
The analysis below assumes that the generic type `T` provides only the minimal trait implementations required by the data structures used in `a_star()` (e.g., `Eq`, `Hash`, or cloning). No additional ordering, iteration, or semantic constraints are assumed.


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
    self.heap.push(value)                                                                           // C1: T = 
}
```
Result: TBD

##### Heap
```rs
pub fn push(&mut self, value: T) {
    self.data.push(value);
    self.bubble_up(self.data.len() - 1);
}
```
Result: TBD

```rs
fn bubble_up(&mut self, mut idx: usize) {
    while idx > 0 {
        let parent = (idx - 1) / 2;
        if self.data[idx] < self.data[parent] {
            self.data.swap(idx, parent);
            idx = parent;
        } else {
            break;
        }
    }
}
```
Result: TBD

### pop()
```rs
pub fn pop(&mut self) -> Option<T> {
    self.heap.pop()
}
```
Result: TBD

##### Heap
```rs
pub fn pop(&mut self) -> Option<T> {
    if self.data.is_empty() {
        return None;
    }
    let last = self.data.pop().unwrap();
    if self.data.is_empty() {
        return Some(last);
    }
    let mut root = std::mem::replace(&mut self.data[0], last);
    self.bubble_down(0);
    Some(root)
}
```
Result: TBD

```rs
fn bubble_down(&mut self, mut idx: usize) {
    let len = self.data.len();
    loop {
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;

        if left >= len {
            break;
        }

        let smallest =
            if right < len && self.data[right] < self.data[left] { right } else { left };

        if self.data[smallest] < self.data[idx] {
            self.data.swap(idx, smallest);
            idx = smallest;
        } else {
            break;
        }
    }
}
```
Result: TBD

### is_empty()
```rs
pub fn is_empty(&self) -> bool {
    self.heap.is_empty()
}
```
Result: TBD

##### Heap
```rs
pub fn is_empty(&self) -> bool {
    self.data.is_empty()
}
```
Result: TBD


## HashTable<K, V>

### new()
```rs
pub fn new() -> Self {
    let p = 2305843009213693951;
    let mut rng = rand::thread_rng();
    let m = 101;

    Self {
        table: vec![Slot::Empty; m],
        a: rng.gen_range(1..p),
        b: rng.gen_range(0..p),
        p,
        m,
        len: 0,
    }
}
```
Result: TBD

### insert(key, value)
```rs
pub fn insert(&mut self, key: K, value: V) -> Result<(), HashTableError> {
    let mut hash_key = self.universal_hash(&key);
    let start = hash_key;

    loop {
        match &self.table[hash_key] {
            Slot::Empty | Slot::Tombstone => {
                self.table[hash_key] = Slot::Occupied(key.clone(), value.clone());
                self.len += 1;
                return Ok(());
            }
            Slot::Occupied(existing_key, _) => {
                if existing_key == &key {
                    self.table[hash_key] = Slot::Occupied(key.clone(), value.clone());
                    return Ok(());
                }
                hash_key = (hash_key + 1) % self.m;
                if hash_key == start {
                    return Err(HashTableError::TableFull);
                }
            }
        }
    }
}
```
Result: TBD

### get(key)
```rs
pub fn get(&self, key: &K) -> Result<&V, HashTableError> {
    let mut hash_key = self.universal_hash(key);
    let start = hash_key;

    loop {
        match &self.table[hash_key] {
            Slot::Occupied(existing_key, value) => {
                if existing_key == key {
                    return Ok(value);
                }
                hash_key = (hash_key + 1) % self.m;
                if hash_key == start {
                    return Err(HashTableError::TableFull);
                }
            }
            Slot::Tombstone => {
                hash_key = (hash_key + 1) % self.m;
                if hash_key == start {
                    return Err(HashTableError::KeyNotFound);
                }
            }
            Slot::Empty => {
                return Err(HashTableError::KeyNotFound);
            }
        }
    }
}
```
Result: TBD

### contains(key)
```rs
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
```
Result: TBD

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
    let dx = (goal.lat as i64 - current.lat as i64).abs() as f64;
    let dy = (goal.long as i64 - current.long as i64).abs() as f64;
    (dx * dx + dy * dy).sqrt()
}
```
Result: TBD

### reconstruct_path(came_from, start, goal)
```rs
fn reconstruct_path(
    came_from: &HashTable<Location, Location>,
    start: Location,
    goal: Location,
) -> Vec<Location> {
    let mut path = vec![goal.clone()];
    let mut current = goal;

    while current != start {
        let prev = came_from
            .get(&current)
            .expect("Path should exist in came_from");
        current = prev.clone();
        path.push(prev.clone());
    }

    path.reverse();
    path
}
```
Result: TBD

#### Estimate best case, worst case, and expected running times for your A* implementation.
##### (Base this on your annotations and then summarize using asymptotic notation.)

### Best Case 

### Worst Case

### Average Case