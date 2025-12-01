/*
    WeightedDirectedGraph
        use an adjacency-list edge representation

    Adjacency List
        each node has a

    Note
        Each class must have a static function that performs a unit test of the class by instantiating and and calling the methods of the class.
*/

use crate::data_structures::hash_table::HashTable;
use std::hash::Hash;

#[derive(Debug)]
pub enum GraphError {
    VertexAlreadyExists,
    GraphFull,
    FromNotFound,
    ToNotFound,
    EdgeNotFound,
}

pub struct WeightedDirectedGraph<T> {
    adj_list: HashTable<T, Vec<(T, f64)>>,
}

impl<T: Hash + Clone + PartialEq> WeightedDirectedGraph<T> {
    /// new()
    ///
    /// Creates a new instance of a WeightedDirectedGraph.
    ///
    /// Returns an instance of a WeightedDirectedGraph.
    ///
    pub fn new() -> Self {
        WeightedDirectedGraph {
            adj_list: HashTable::<T, Vec<(T, f64)>>::new(),
        }
    }

    /// add_vertex()
    ///
    /// Adds a new vertex to the graph.
    /// Initializes the Node and an empty list to go along with it.
    ///
    /// Returns the Result of the operation.
    ///
    pub fn add_vertex(&mut self, id: T) -> Result<(), GraphError> {
        if !self.adj_list.contains(&id) {
            self.adj_list
                .insert(id, Vec::new())
                .map_err(|_| GraphError::GraphFull)?;
            Ok(())
        } else {
            Err(GraphError::VertexAlreadyExists)
        }
    }

    /// add_edge()
    ///
    /// Adds a new edge to the adjacency list.
    /// From and To should exist in the HashMap.
    ///
    /// Returns the Result of the Operation.
    ///
    pub fn add_edge(&mut self, to: T, from: T, cost: f64) -> Result<(), GraphError> {
        // Confirm we have the vertices
        if !self.adj_list.contains(&from) {
            return Err(GraphError::FromNotFound);
        }
        if !self.adj_list.contains(&to) {
            return Err(GraphError::ToNotFound);
        }

        // Add the edge
        let neighbors = self
            .adj_list
            .get_mut(&from)
            .map_err(|_| GraphError::FromNotFound)?;
        neighbors.push((to, cost));

        Ok(())
    }

    /// get_neighbors
    ///
    /// Gets the adjacent neighbors of a vertex.
    ///
    /// Returns a list of the adjacent vertices.
    ///
    fn get_neighbors(&self, vertex: T) -> Vec<(T, f64)> {
        self.adj_list
            .get(&vertex)
            .map(|v| v.clone())
            .unwrap_or_else(|_| Vec::new())
    }

    /// get_weight()
    ///
    /// Finds and returns the weight between two nodes.
    /// From and To should exist in the HashMap.
    ///
    /// Returns the weight between two nodes
    ///
    fn get_weight(&self, to: T, from: T) -> Result<Vec<(T, f64)>, GraphError> {
        // Confirm we have the vertices
        if !self.adj_list.contains(&from) {
            return Err(GraphError::FromNotFound);
        }
        if !self.adj_list.contains(&to) {
            return Err(GraphError::ToNotFound);
        }

        // Get the adjacency list for `from`
        let neighbors = self
            .adj_list
            .get(&from)
            .map_err(|_| GraphError::FromNotFound)?;

        // Filter edges to `to` and collect
        let edges: Vec<(T, f64)> = neighbors
            .iter()
            .filter(|(neighbor, _)| *neighbor == to)
            .map(|(neighbor, weight)| (neighbor.clone(), *weight))
            .collect();

        if edges.is_empty() {
            Err(GraphError::EdgeNotFound)
        } else {
            Ok(edges)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_graph() {
        let mut graph = WeightedDirectedGraph::<&str>::new();

        // Add some vertices
        assert!(graph.add_vertex("one").is_ok());
        assert!(graph.add_vertex("two").is_ok());
        assert!(graph.add_vertex("three").is_ok());
        assert!(graph.add_vertex("five").is_ok());
        assert!(graph.add_vertex("six").is_ok());

        // Add some edges
        assert!(graph.add_edge("one", "two", 3.0).is_ok());
        assert!(graph.add_edge("two", "three", 4.0).is_ok());
        assert!(graph.add_edge("five", "three", 2.0).is_ok());
        assert!(graph.add_edge("five", "one", 1.0).is_ok());

        // Add some bad edges
        assert!(graph.add_edge("four", "two", 3.0).is_err());
        assert!(graph.add_edge("two", "four", 3.0).is_err());

        // Get neighbors
        assert_eq!(graph.get_neighbors("one"), vec![("five", 1.0)]);
        assert_eq!(
            graph.get_neighbors("three"),
            vec![("two", 4.0), ("five", 2.0)]
        );
        assert_eq!(graph.get_neighbors("two"), vec![("one", 3.0)]);
        assert_eq!(graph.get_neighbors("six"), vec![]);

        // Test get_weight
        assert_eq!(graph.get_weight("one", "two").unwrap(), vec![("one", 3.0)]);
        assert!(graph.get_weight("one", "five").is_err()); // No edge exists
    }
}
