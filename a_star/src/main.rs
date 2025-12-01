mod data_structures;

use std::fmt;

use crate::data_structures::graph::{WeightedDirectedGraph, GraphError};

#[derive(Clone, Hash, PartialEq)]
struct Location {
    name: String,
    lat: u32,
    long: u32,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (lat: {}, long: {})", self.name, self.lat, self.long)
    }
}

enum AStarError {}

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
    // Define our locations
    let locations = vec![];

    // Create the graph
    let mut graph = create_graph(&locations);

    // Compute the weights
    match compute_weights(&mut graph) {
        Ok(_) => contin
    }

    // Select destinations
    let (from, to) = pick_locations(&locations);

    // A* Search
    let result = a_star(to, from);
}

/// define_locations()
///
/// Creates a WeightedDirectedGraph of our defined locations.
///
/// Returns an instance of the created graph.
///
fn create_graph(locations: &Vec<Location>) -> Result<WeightedDirectedGraph<Location>, GraphError> {
    let mut graph = WeightedDirectedGraph::<Location>::new();
    
    for loc in locations {
        // Add all our vertices
        match graph.add_vertex(loc.clone()) {
            Ok(_) => continue,
            Err(e) => return Err(e)
        }
    }

    Ok(graph)
}

/// compute_weights()
///
/// Computes weights between locations.
/// Creates the edges between vertices to be used by A*.
///
/// Returns None
///
fn compute_weights(graph: &mut WeightedDirectedGraph<Location>) {
    todo!()
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
fn a_star(to: Location, from: Location) -> Result<Vec<Location>, AStarError> {
    todo!()
}
