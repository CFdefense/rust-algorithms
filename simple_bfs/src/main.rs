use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TreeNode {
    pub vertex: u32,
    pub neighbors: Vec<TreeNode>,
}

// Test cases: adjacency list, start, goal, expected path
fn get_test_cases() -> Vec<(HashMap<u32, Vec<u32>>, u32, u32, Vec<u32>)> {
    vec![
        // Test 1: Simple linear path
        (
            {
                let mut adj = HashMap::new();
                adj.insert(1, vec![2]);
                adj.insert(2, vec![1, 3]);
                adj.insert(3, vec![2, 4]);
                adj.insert(4, vec![3]);
                adj
            },
            1,
            4,
            vec![1, 2, 3, 4],
        ),
        // Test 2: Multiple paths (BFS should find shortest)
        (
            {
                let mut adj = HashMap::new();
                adj.insert(1, vec![2, 3]);
                adj.insert(2, vec![1, 4]);
                adj.insert(3, vec![1, 4]);
                adj.insert(4, vec![2, 3, 5]);
                adj.insert(5, vec![4]);
                adj
            },
            1,
            5,
            vec![1, 2, 4, 5], // or [1, 3, 4, 5] - both are shortest
        ),
        // Test 3: No path exists
        (
            {
                let mut adj = HashMap::new();
                adj.insert(1, vec![2]);
                adj.insert(2, vec![1]);
                adj.insert(3, vec![4]);
                adj.insert(4, vec![3]);
                adj
            },
            1,
            3,
            vec![], // No path
        ),
        // Test 4: Single node
        (
            {
                let mut adj = HashMap::new();
                adj.insert(1, vec![]);
                adj
            },
            1,
            1,
            vec![1],
        ),
        // Test 5: Star graph
        (
            {
                let mut adj = HashMap::new();
                adj.insert(1, vec![2, 3, 4, 5]);
                adj.insert(2, vec![1]);
                adj.insert(3, vec![1]);
                adj.insert(4, vec![1]);
                adj.insert(5, vec![1]);
                adj
            },
            2,
            5,
            vec![2, 1, 5],
        ),
        // Test 6: Prof Johnson's sparse graph
        (
            {
                let mut adj = HashMap::new();
                adj.insert(0, vec![1, 3]);
                adj.insert(1, vec![0, 2, 4]);
                adj.insert(2, vec![1, 5]);
                adj.insert(3, vec![0, 4, 6]);
                adj.insert(4, vec![1, 3, 5, 6]);
                adj.insert(5, vec![2, 4, 7]);
                adj.insert(6, vec![3, 4, 7]);
                adj.insert(7, vec![5, 6]);
                adj
            },
            0,
            5,
            vec![0, 1, 2, 5], // or [0, 3, 4, 5] - both are shortest paths
        ),
    ]
}

// Convert adjacency list to TreeNode structure
fn build_tree(adj: &HashMap<u32, Vec<u32>>, start: u32) -> TreeNode {
    let mut visited = HashSet::new();
    build_recursive(adj, start, &mut visited)
}

fn build_recursive(
    adj: &HashMap<u32, Vec<u32>>,
    node: u32,
    visited: &mut HashSet<u32>,
) -> TreeNode {
    visited.insert(node);

    let neighbors = adj.get(&node).cloned().unwrap_or_default();
    let mut tree_neighbors = Vec::new();

    for neighbor in neighbors {
        if !visited.contains(&neighbor) {
            tree_neighbors.push(build_recursive(adj, neighbor, visited));
        }
    }

    TreeNode {
        vertex: node,
        neighbors: tree_neighbors,
    }
}

// Find a specific node in the tree
fn find_node(root: &TreeNode, target: u32) -> Option<TreeNode> {
    if root.vertex == target {
        return Some(root.clone());
    }

    for neighbor in &root.neighbors {
        if let Some(found) = find_node(neighbor, target) {
            return Some(found);
        }
    }

    None
}

// Simple test runner
fn run_tests() {
    println!("Running BFS tests...");

    for (i, (adj, start, goal, expected)) in get_test_cases().iter().enumerate() {
        println!("\nTest {}: Start={}, Goal={}", i + 1, start, goal);

        let start_tree = build_tree(adj, *start);
        let goal_tree = find_node(&start_tree, *goal);

        if goal_tree.is_none() && expected.is_empty() {
            println!("PASSED (no path exists)");
            continue;
        }

        if let Some(goal_node) = goal_tree {
            let result = traverse(start_tree, goal_node);
            let result_path: Vec<u32> = result.iter().map(|n| n.vertex).collect();

            println!("Expected: {:?}", expected);
            println!("Got:      {:?}", result_path);

            if result_path == *expected
                || (expected.len() > 0
                    && result_path.len() == expected.len()
                    && result_path[0] == *start
                    && result_path[result_path.len() - 1] == *goal)
            {
                println!("PASSED");
            } else {
                println!("FAILED");
            }
        } else {
            println!("FAILED (couldn't find goal node)");
        }
    }
}

fn main() {
    run_tests();
}

pub fn traverse(start: TreeNode, goal: TreeNode) -> Vec<TreeNode> {
    let mut frontier = VecDeque::new();
    let mut came_from: HashMap<TreeNode, Option<TreeNode>> = HashMap::new();

    frontier.push_back(start.clone());
    came_from.insert(start.clone(), None);

    while let Some(current) = frontier.pop_front() {
        for next in &current.neighbors {
            if !came_from.contains_key(&next) {
                frontier.push_back(next.clone());
                came_from.insert(next.clone(), Some(current.clone()));
            }
        }
    }

    // reconstruct the best path
    let mut current = goal;
    let mut path: Vec<TreeNode> = Vec::new();
    path.push(current.clone());

    while current != start {
        // get previous node from came_from
        if let Some(Some(prev)) = came_from.get(&current) {
            current = prev.clone();
            path.push(current.clone());
        } else {
            // couldnt find path
            return vec![];
        }
    }
    path.reverse();
    path
}
