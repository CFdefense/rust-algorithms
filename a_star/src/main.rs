mod data_structures;

use core::panic;
use std::{fmt, cmp::Ordering};

use crate::data_structures::{
    priority_queue::PriorityQueue,
    hash_table::HashTable,
    graph::{GraphError, WeightedDirectedGraph},
};


#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Location {
    name: String,
    lat: u32,
    long: u32,
}

#[derive(Clone, PartialEq, Eq, Default)]
struct PriorityLocation {
    location: Location,
    priority: OrderedFloat,
}

impl PartialOrd for PriorityLocation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriorityLocation {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap (lower priority first)
        other.priority.cmp(&self.priority)
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
struct OrderedFloat(f64);

impl Eq for OrderedFloat {}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (lat: {}, long: {})", self.name, self.lat, self.long)
    }
}

#[derive(Debug)]
enum AStarError {
    NoPathFound,
    InvalidStartOrGoal,
}

///
/// A* Search
///
///    1. Define a test graph
///         This graph should represent travel distances among cities or points-of-interest (real or imaginary).
///         Each vertex represents a place (e.g., city, point-of-interest, etc.) and should have a name and coordinates (i.e., GPS lat. & lon.).
///         Your weighting function for the graph should return the travel distance (e.g., via road) between two places.
///         Note that a weight should not be computed from the coordinates - that would be straight-line distance (i.e., "as the crow flies").
///
///    2. Implement A* Search
///         Use datastructures created in data_structures/
///         Your heuristic function should compute the straight-line distance between two vertices using their coordinates.
///         (Don't worry about accounting for planet surface curvature, just assume a flat plane.)
///
///     3. Main Function Will
///         Creates a graph, selects a start and a goal, and then runs your A* function to determine a shortest path.
///
fn main() {
    // Define our locations - Marist College buildings with approximate coordinates
    // Coordinates represent positions on campus (simplified 2D plane)
    let locations = vec![
        Location { name: "Donnelly Hall".to_string(), lat: 0, long: 0 },
        Location { name: "Student Center".to_string(), lat: 5, long: 3 },
        Location { name: "Library".to_string(), lat: 8, long: 5 },
        Location { name: "Steel Plant".to_string(), lat: 12, long: 8 },
        Location { name: "Hancock Center".to_string(), lat: 15, long: 12 },
    ];

    // Define walking paths with distances in meters
    let roads = vec![
        (0, 1, 120.0),  // Donnelly <-> Student Center
        (1, 0, 120.0),
        (1, 2, 80.0),   // Student Center <-> Library
        (2, 1, 80.0),
        (2, 3, 150.0),  // Library <-> Steel Plant
        (3, 2, 150.0),
        (3, 4, 200.0),  // Steel Plant <-> Hancock
        (4, 3, 200.0),
        (0, 2, 180.0),  // Donnelly <-> Library (direct path)
        (2, 0, 180.0),
        (1, 4, 400.0),  // Student Center <-> Hancock
        (4, 1, 400.0),
    ];

    // Create the graph
    let graph = match create_graph(&locations, roads) {
        Ok(g) => g,
        Err(e) => panic!("Graph Creation Failed {:?}", e)
    };

    // Select destinations
    let (from, to) = pick_locations(&locations);

    println!("\nSearching for shortest path from {} to {}...\n", from, to);

    // A* Search
    match a_star(&graph, from.clone(), to.clone()) {
        Ok(path) => {
            println!("Path found!");
            println!("\nRoute:");
            for (i, location) in path.iter().enumerate() {
                if i == 0 {
                    println!("  START: {}", location);
                } else if i == path.len() - 1 {
                    println!("  GOAL:  {}", location);
                } else {
                    println!("  STEP {}: {}", i, location);
                }
            }
            println!("\nTotal stops: {}", path.len());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

/// create_graph()
///
/// Creates a WeightedDirectedGraph of our defined locations.
/// Also creates edges between locations based on the roads parameter.
///
/// Returns an instance of the created graph.
///
fn create_graph(
    locations: &Vec<Location>,
    roads: Vec<(usize, usize, f64)>
) -> Result<WeightedDirectedGraph<Location>, GraphError> {
    let mut graph = WeightedDirectedGraph::<Location>::new();
    
    // Add our vertices
    for loc in locations {
        graph.add_vertex(loc.clone())?;
    }

    // Add edges
    for (from_idx, to_idx, distance) in roads {
        if from_idx >= locations.len() || to_idx >= locations.len() {
            return Err(GraphError::VertexNotFound);
        }
        
        let from_loc = locations[from_idx].clone();
        let to_loc = locations[to_idx].clone();
        
        graph.add_edge(from_loc, to_loc, distance)?;
    }

    Ok(graph)
}

/// pick_locations
///
/// Allow the user to select the locations to travel between.
///
/// Returns a tuple of Locations (from, to)
///
fn pick_locations(locations: &Vec<Location>) -> (Location, Location) {
    // print the options
    for (i, loc) in locations.iter().enumerate() {
        println!("{}: {}", (i+1), loc)
    }

    loop {
        println!("Please select 2 locations using numbers, e.g. (1,2):");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // Remove parentheses and whitespace
        let cleaned: Vec<char> = input
            .chars()
            .filter(|c| !c.is_whitespace() && *c != '(' && *c != ')')
            .collect();

        // Must contain exactly something like: '1', ',', '2'
        if !cleaned.iter().any(|c| *c == ',') {
            println!("Missing comma. Try again.");
            continue;
        }

        // Split on the comma
        let parts: Vec<String> = cleaned
            .split(|c| *c == ',')
            .map(|chunk| chunk.iter().collect::<String>())
            .collect();

        if parts.len() != 2 {
            println!("You must enter exactly two numbers. Try again.");
            continue;
        }

        // Convert both sides to numbers
        let Ok(a) = parts[0].parse::<usize>() else {
            println!("First value is not a valid number. Try again.");
            continue;
        };
        let Ok(b) = parts[1].parse::<usize>() else {
            println!("Second value is not a valid number. Try again.");
            continue;
        };

        // Bounds check: must be 1..=locations.len()
        if a == 0 || a > locations.len() || b == 0 || b > locations.len() {
            println!("Each number must be between 1 and {}.", locations.len());
            continue;
        }

        // Return the selected locations
        return (locations[a - 1].clone(), locations[b - 1].clone());
    }
}

/// a_star
///
/// Computes the shortest path between two Locations using A* Search Algorithm.
///
/// Returns the Result of the algorithm as a Path or an Error.
///
fn a_star(
    graph: &WeightedDirectedGraph<Location>,
    start: Location,
    goal: Location,
) -> Result<Vec<Location>, AStarError> {
    // Validate that start and goal exist in the graph
    if graph.get_neighbors(start.clone()).is_empty() && start != goal {
        return Err(AStarError::InvalidStartOrGoal);
    }

    // Create our queue frontier
    let mut frontier = PriorityQueue::<PriorityLocation>::new();

    // Add the first location
    frontier.push(PriorityLocation {
        location: start.clone(),
        priority: OrderedFloat(0.0),
    });

    let mut came_from = HashTable::<Location, Location>::new();
    let mut cost_so_far = HashTable::<Location, f64>::new();
    
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
            let current_cost = cost_so_far.get(&current).unwrap_or(&f64::INFINITY);
            let new_cost = current_cost + edge_cost;

            // If we haven't visited this node or found a cheaper path
            if !cost_so_far.contains(&next) || new_cost < *cost_so_far.get(&next).unwrap() {
                cost_so_far.insert(next.clone(), new_cost).ok();
                let priority = new_cost + heuristic(&goal, &next);
                frontier.push(PriorityLocation {
                    location: next.clone(),
                    priority: OrderedFloat(priority),
                });
                came_from.insert(next.clone(), current.clone()).ok();
            }
        }
    }

    Err(AStarError::NoPathFound)
}


/// heuristic
///
/// Computes the straight-line distance between two locations.
/// Uses the Pythagorean theorem on a flat plane.
///
/// Returns the estimated distance as f64.
///
fn heuristic(goal: &Location, current: &Location) -> f64 {
    let dx = (goal.lat as i64 - current.lat as i64).abs() as f64;
    let dy = (goal.long as i64 - current.long as i64).abs() as f64;
    (dx * dx + dy * dy).sqrt()
}

/// reconstruct_path
///
/// Reconstructs the path from start to goal using the came_from map.
///
/// Returns a vector of Locations representing the path.
///
fn reconstruct_path(
    came_from: &HashTable<Location, Location>,
    start: Location,
    goal: Location,
) -> Vec<Location> {
    let mut path = vec![goal.clone()];
    let mut current = goal;

    while current != start {
        if let Ok(prev) = came_from.get(&current) {
            path.push(prev.clone());
            current = prev.clone();
        } else {
            break;
        }
    }

    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_graph_with_edges() {
        // Create test locations - we'll use planets
        let locations = vec![
            Location { name: "Mercury".to_string(), lat: 0, long: 0 },
            Location { name: "Venus".to_string(), lat: 10, long: 5 },
            Location { name: "Earth".to_string(), lat: 15, long: 10 },
        ];
        
        // Define space routes by index: (from_idx, to_idx, distance)
        let roads = vec![
            (0, 1, 50.0),  // Mercury -> Venus
            (0, 2, 100.0), // Mercury -> Earth
            (1, 0, 50.0),  // Venus -> Mercury
            (1, 2, 40.0),  // Venus -> Earth
            (2, 1, 40.0),  // Earth -> Venus
        ];
        
        // Create graph with edges
        let graph = create_graph(&locations, roads).unwrap();
        
        // Verify edges exist
        let mercury_neighbors = graph.get_neighbors(locations[0].clone());
        assert_eq!(mercury_neighbors.len(), 2, "Mercury should have 2 outgoing routes");
        assert!(mercury_neighbors.iter().any(|(loc, weight)| 
            loc.name == "Venus" && *weight == 50.0), 
            "Mercury should have route to Venus");
        assert!(mercury_neighbors.iter().any(|(loc, weight)| 
            loc.name == "Earth" && *weight == 100.0), 
            "Mercury should have route to Earth");
        
        // Verify Venus neighbors
        let venus_neighbors = graph.get_neighbors(locations[1].clone());
        assert_eq!(venus_neighbors.len(), 2, "Venus should have 2 outgoing routes");
        assert!(venus_neighbors.iter().any(|(loc, weight)| 
            loc.name == "Mercury" && *weight == 50.0), 
            "Venus should have route to Mercury");
        assert!(venus_neighbors.iter().any(|(loc, weight)| 
            loc.name == "Earth" && *weight == 40.0), 
            "Venus should have route to Earth");
    }

    #[test]
    fn test_create_graph_bidirectional() {
        let locations = vec![
            Location { name: "Mars".to_string(), lat: 0, long: 0 },
            Location { name: "Jupiter".to_string(), lat: 20, long: 15 },
        ];
        
        let roads = vec![
            (0, 1, 200.0), // Mars -> Jupiter
            (1, 0, 200.0), // Jupiter -> Mars
        ];
        
        let graph = create_graph(&locations, roads).unwrap();
        
        // Verify bidirectional routes
        let mars_neighbors = graph.get_neighbors(locations[0].clone());
        assert_eq!(mars_neighbors.len(), 1, "Mars should have 1 outgoing route");
        assert!(mars_neighbors.iter().any(|(loc, _)| *loc == locations[1]), 
                "Mars should have route to Jupiter");
        
        let jupiter_neighbors = graph.get_neighbors(locations[1].clone());
        assert_eq!(jupiter_neighbors.len(), 1, "Jupiter should have 1 outgoing route");
        assert!(jupiter_neighbors.iter().any(|(loc, _)| *loc == locations[0]), 
                "Jupiter should have route to Mars");
    }

    #[test]
    fn test_create_graph_hub_and_spoke() {
        // Test a hub (Sun) connected to multiple planets
        let locations = vec![
            Location { name: "Sun".to_string(), lat: 0, long: 0 },
            Location { name: "Mercury".to_string(), lat: 5, long: 0 },
            Location { name: "Venus".to_string(), lat: 10, long: 0 },
            Location { name: "Earth".to_string(), lat: 15, long: 0 },
        ];
        
        // Sun connects to all planets
        let roads = vec![
            (0, 1, 58.0),  // Sun -> Mercury
            (0, 2, 108.0), // Sun -> Venus
            (0, 3, 150.0), // Sun -> Earth
        ];
        
        let graph = create_graph(&locations, roads).unwrap();
        
        let sun_neighbors = graph.get_neighbors(locations[0].clone());
        assert_eq!(sun_neighbors.len(), 3, "Sun should have 3 outgoing routes");
        
        // Verify each planet route
        assert!(sun_neighbors.iter().any(|(loc, weight)| 
            loc.name == "Mercury" && *weight == 58.0));
        assert!(sun_neighbors.iter().any(|(loc, weight)| 
            loc.name == "Venus" && *weight == 108.0));
        assert!(sun_neighbors.iter().any(|(loc, weight)| 
            loc.name == "Earth" && *weight == 150.0));
    }
}