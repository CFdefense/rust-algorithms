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

pub enum GraphError {
    VertexAlreadyExists,
    GraphFull,
    FromNotFound,
    ToNotFound,
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
    fn new() -> Self {
        WeightedDirectedGraph {
            adj_list: HashTable::new(),
        }
    }

    /// add_vertex()
    ///
    /// Adds a new vertex to the graph.
    /// Initializes the Node and an empty list to go along with it.
    ///
    /// Returns the Result of the operation.
    ///
    fn add_vertex(&mut self, id: T) -> Result<(), GraphError> {
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
    fn add_edge(&mut self, to: T, from: T, cost: f64) -> Result<(), GraphError> {
        // Confirm we have the From vertex
        if !self.adj_list.contains(&from) {
            return Err(GraphError::FromNotFound);
        }

        // Confirm we have the To vertex
        if !self.adj_list.contains(&to) {
            return Err(GraphError::ToNotFound);
        }

        // Add edge
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
    /// Returns a list of the adjacent vertexes.
    ///
    fn get_neighbors(&self, vertex: T) -> Vec<(T, f64)> {
        self.adj_list
            .get(&vertex)
            .map(|v| v.clone())
            .unwrap_or_else(|_| Vec::new())
    }

    /// compute_cost()
    ///
    ///
    ///
    fn compute_cost(&self, to: T, from: T) -> Result<(), GraphError> {
        todo!()
    }
}
