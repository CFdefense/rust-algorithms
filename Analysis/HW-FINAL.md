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
    self.adj_list.contains(vertex)                                                                  // C1: T = F(contains()) = O(m · |Location|)
}
```
Worst: O(m · |Location|)
Average: O(|Location|)
Best: O(|Location|)

##### See Hash Table contains() @ HashTable<K, V> Below

### get_neighbors(vertex)
```rs
pub fn get_neighbors(&self, vertex: T) -> Vec<(T, f64)> {
    self.adj_list                                                                                   
        .get(&vertex)                                                                               // C1: T = O(1)     Reason: Hash Lookup
        .map(|v| v.clone())                                                                         // C2: T = O(k)     Reason: Neighbor count
        .unwrap_or_else(|_| Vec::new())                                                             // C3: T = O(1)     Reason: Unwrap operation and vector init both O(1)
}
```
Worst: O(k · |Location|)
Average: O(k · |Location|)
Best: O(1)

## PriorityQueue<T>

### new()
```rs
pub fn new() -> Self {
    PriorityQueue {
        heap: MinHeap { data: Vec::new() },                                                         // C1: T = O(1)     Reason: Simple Initialization.
    }
}
```
Worst: O(1)
Average: O(1)
Best: O(1)

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
    if !graph.contains_vertex(&start) || !graph.contains_vertex(&goal) {                            // C1: T = 2F(contains()) = 2(O(|Location|))
        return Err(AStarError::InvalidStartOrGoal);                                                 // C2: T = O(0..1)      Reason: Happens at most once
    }

    // Create our queue frontier
    let mut frontier = PriorityQueue::<PriorityLocation>::new();                                    // C3: T = O(F(PriorityQueue::new)) = O(1)

    // Add the first location with priority 0.0
    frontier.push(PriorityLocation {                                                                // C4: T = O(1) + O(log 1)      
                                                                                                    // Reason: Initial push, heap has 0 elements
        location: start.clone(),                                                                    // C5: T = O(|Location|)        Reason: Clone of Location
        priority: OrderedFloat(0.0),                                                                // C6: T = O(1)         Reason: Simple Assignment
    });

    let mut came_from = HashTable::<Location, Location>::new();                                     // C7: T = F(HashTable::new()) = O(m)
    let mut cost_so_far = HashTable::<Location, f64>::new();                                        // C8: T = F(HashTable::new()) = O(m)

    // insert start cost = 0.0 (ignore insert error)
    cost_so_far.insert(start.clone(), 0.0).ok();                                                    // C9: T = O(|Location|) 
                                                                                                    // Reason: insert best case for empty table
    while !frontier.is_empty() {                                                                    // C10: T = O(1..n) 
                                                                                                    // Reason: At least once, at most # nodes
        let priority_loc = frontier.pop().ok_or(AStarError::NoPathFound)?;                          // C11: T = F(pop() = O(log n)
        let current = priority_loc.location;                                                        // C12: T = O(1)    Simple Assignment

        // Goal reached
        if current == goal {                                                                        // C13: T = O(|Location|)    Reason: Compare Location Eq
            return Ok(reconstruct_path(&came_from, start, goal));                                   // C14: T = F(reconstruct_path()) = O(p·|Location|) 
        }

        // Explore neighbors
        let neighbors = graph.get_neighbors(current.clone());                                       // C15: T = F(get_neighbors()) = O(k·|Location|)

        for (next, edge_cost) in neighbors {                                                        // C16: T = O(k)    Reason: Iterating over neighbors
            // current_cost: read from cost_so_far
            let current_cost = match cost_so_far.get(&current) {                                    // C17: T = F(get()) = O(|Location|
                Ok(v) => *v,                                                                        // C18: T = O(1)    Reason: Simple Dereference
                Err(_) => f64::INFINITY,                                                            // C19: T = O(1)    Reason: Simple Assignment
            };

            let new_cost = current_cost + edge_cost;                                                // C20: T = O(1)    Reason: Simple Operation

            // old_cost: if not present treat as +inf
            let old_cost = match cost_so_far.get(&next) {                                           // C21: T = F(get()) = O(|Location|
                Ok(v) => *v,                                                                        // C22: T = O(1)    Reason: Simple Dereference
                Err(_) => f64::INFINITY,                                                            // C23: T = O(1)    Reason: Simple Assignment
            };

            if new_cost < old_cost {                                                                // C24: T = O(1) Reason: Simple Comparison
                // update cost_so_far and came_from (ignoring errors as not fatal here)
                cost_so_far.insert(next.clone(), new_cost).ok();                                    // C25: T = (F(insert()) = O(|Location|) + O(|Location|)
                                                                                                    // Reason: insert() + cost of cloning next
                came_from.insert(next.clone(), current.clone()).ok();                               // C26: T = (F(insert()) = O(|Location|) + O(2|Location|)
                                                                                                    // Reason: insert() + cost of cloning next and current

                // push
                frontier.push(PriorityLocation {                                                    // C27: T = F(push()) = O(log n)
                    location: next.clone(),                                                         // C28: T = O(|Location|) Reason: Clone Location
                    priority: OrderedFloat(new_cost + heuristic(&goal, &next)),                     // C29: T = F(heuristic()) = O(1)
                });
            }
        }
    }

    Err(AStarError::NoPathFound)                                                                    // C30: T = O(0..1) 
                                                                                                    // Reason: Happens at most once if no path
}
```
Result: See A* Search Analysis Section Below

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

## A* Search Analysis

#### Estimate best case, worst case, and expected running times for your A* implementation.
##### (Base this on your annotations and then summarize using asymptotic notation.)

#### Variables
n → Number of nodes in the graph.

k → Average number of neighbors for a node.

p → Length of the path from start to goal.

m → Size of the hash table used in came_from and cost_so_far.

|Location| → Cost of hashing or comparing a Location struct.

### Worst Case

#### Scenerio
- Explores all nodes and all neighbors (full graph).
- Each hash table get/insert takes O(|Location|).
- Each heap operation is O(log n).

#### Add up the Costs
Worst = C1 + C2 + C3 + C4 + C5 + C6 + C7 + C8 + C9 + C10(n) * (C11 + C12 + C13 + C14 + C15(k) * (C17 + C18 + C19 + C20 + C21 + C22 + C23 + C24 + C25 + C26 + C27 + C28 + C29)) + C30

#### Substitute the Times
Worst = O(|Location|) + O(0..1) + O(1) + O(1) + O(|Location|) + O(1) + O(m) + O(m) + O(|Location|) + n * (O(|Location|) + O(1) + O(1) + O(1) + k * (O(|Location|) + O(|Location|) + O(1) + O(|Location|) + O(|Location|) + O(log n) + O(|Location|) + O(1) + O(|Location|) + O(0..1) + O(1) + O(1) + O(|Location|))) + O(0..1)

#### Simplify and Sum
Worst = O(n * k * |Location| + n * log n + m + p * |Location|)

#### Drop constants
Worst = O(n * k * |Location| + n * log n + m + p * |Location|)

### Average Case

#### Scenerio
- Explores roughly half the nodes (~n/2).
- Explores roughly half the neighbors per node (~k/2).
- Heap operations roughly log(frontier size) ≈ log(n).
- Hash table operations as usual.

#### Add up the Costs
Average = C1 + C2 + C3 + C4 + C5 + C6 + C7 + C8 + C9 + C10(~n/2) * (C11 + C12 + C13 + C14 + C15(~k/2) * (C17 + C18 + C19 + C20 + C21 + C22 + C23 + C24 + C25 + C26 + C27 + C28 + C29)) + C30

#### Substitute the Times
Average = O(|Location|) + O(0..1) + O(1) + O(1) + O(|Location|) + O(1) + O(m) + O(m) + O(|Location|) + (n/2) * (O(|Location|) + O(1) + O(1) + O(1) + (k/2) * (O(|Location|) + O(|Location|) + O(1) + O(|Location|) + O(|Location|) + O(log n) + O(|Location|) + O(1) + O(|Location|) + O(0..1) + O(1) + O(1) + O(|Location|))) + O(0..1)

#### Simplify and Sum
Average = O(|Location| + m + n * k * |Location| + n * k * log n)

#### Drop Constants
Average = O(m + n * k * |Location| + n * k * log n)

### Best Case 

#### Scenerio
- Goal is reached immediately (first pop).
- Only first node explored.
- Only one neighbor iteration happens.
- Frontier has size 1.

#### Add up the Costs
Best = C1 + C2 + C3 + C4 + C5 + C6 + C7 + C8 + C9 + 
       C10(1) + C11(1) + C12 + C13 + C14 + C15(1 neighbor) + 
       C17 + C18 + C19 + C20 + C21 + C22 + C23 + C24 + 
       C25 + C26 + C27 + C28 + C29 + C30(0..1)

#### Substitute the Times
Best = O(|Location|) + O(0..1) + O(1) + O(1) + O(|Location|) + O(1) + O(m) + O(m) + O(|Location|) +
       O(1) + O(1) + O(1) + O(|Location|) + O(p·|Location|) + O(k·|Location|) + O(|Location|) + O(1) + O(1) + O(1) + 
       O(|Location|) + O(|Location|) + O(log 1) + O(|Location|) + O(1) + O(|Location|) + O(0..1)

#### Simplify and Sum
Best = O(m + |Location| + p·|Location| + k·|Location| + log 1)

#### Drop constants
Best = O(m + p·|Location| + k·|Location|)