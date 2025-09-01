// Simple BFS test cases
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub vertex: u32,
    pub neighbors: Vec<TreeNode>,
}

// Test cases: adjacency list, start, goal, expected path
pub fn get_test_cases() -> Vec<(HashMap<u32, Vec<u32>>, u32, u32, Vec<u32>)> {
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
            1, 4, vec![1, 2, 3, 4]
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
            1, 5, vec![1, 2, 4, 5] // or [1, 3, 4, 5] - both are shortest
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
            1, 3, vec![] // No path
        ),
        
        // Test 4: Single node
        (
            {
                let mut adj = HashMap::new();
                adj.insert(1, vec![]);
                adj
            },
            1, 1, vec![1]
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
            2, 5, vec![2, 1, 5]
        ),
    ]
}

// Convert adjacency list to TreeNode structure
pub fn build_tree(adj: &HashMap<u32, Vec<u32>>, start: u32) -> TreeNode {
    let mut visited = std::collections::HashSet::new();
    build_recursive(adj, start, &mut visited)
}

fn build_recursive(
    adj: &HashMap<u32, Vec<u32>>, 
    node: u32, 
    visited: &mut std::collections::HashSet<u32>
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
pub fn find_node(root: &TreeNode, target: u32) -> Option<TreeNode> {
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
pub fn run_tests(traverse_fn: fn(TreeNode, TreeNode) -> Vec<TreeNode>) {
    println!("Running BFS tests...");
    
    for (i, (adj, start, goal, expected)) in get_test_cases().iter().enumerate() {
        println!("\nTest {}: Start={}, Goal={}", i + 1, start, goal);
        
        let start_tree = build_tree(adj, *start);
        let goal_tree = find_node(&start_tree, *goal);
        
        if goal_tree.is_none() && expected.is_empty() {
            println!("✅ PASSED (no path exists)");
            continue;
        }
        
        if let Some(goal_node) = goal_tree {
            let result = traverse_fn(start_tree, goal_node);
            let result_path: Vec<u32> = result.iter().map(|n| n.vertex).collect();
            
            println!("Expected: {:?}", expected);
            println!("Got:      {:?}", result_path);
            
            if result_path == *expected || (expected.len() > 0 && result_path.len() == expected.len() && result_path[0] == *start && result_path[result_path.len()-1] == *goal) {
                println!("✅ PASSED");
            } else {
                println!("❌ FAILED");
            }
        } else {
            println!("❌ FAILED (couldn't find goal node)");
        }
    }
}
