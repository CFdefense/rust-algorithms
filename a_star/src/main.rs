mod data_structures;

use crate::data_structures::graph::WeightedDirectedGraph;

struct Location {
    name: String,
    lat: u32,
    long: u32,
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
    compute_weights(&mut graph);

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
fn create_graph(locations: &Vec<Location>) -> WeightedDirectedGraph<Location> {
    todo!()
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
    todo!()
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
