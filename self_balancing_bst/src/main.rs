pub struct TreeNode {
    pub key: i32,
    pub left_count: u32,
    pub right_count: u32,
}

impl TreeNode {
    fn new(key: i32) -> Self {
        Self { key, left_count: 0, right_count: 0 }
    }
}

pub struct SelfBalancingTree {
    root: Option<Box<TreeNode>>,
}

impl SelfBalancingTree {
    /*
        Q: Given a BST of size n constructed and maintained by these self-balancing version of INSERT and DELETE, 
        what is the maximum height of the tree (in terms of n)?

        A: 
    
     */
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, value: u32) {
        /*
            Q: What are the running times, in asymptotic notation, for INSERT? 
            
            A:
            
            Q: Are the best and worst cases different asymptotically compared to DELETE.

            A:
         */

        // First compare x to the root 


        // In the case that x belongs in the left sub-tree

            // If left sub-tree is larger than right sub-tree

                // compare x.key to the predecessor of the root node.

                // If x.key is larger
                
                    // Replace the root by x and insert the former root into the right sub-tree

                // If x.key is smaller
                
                    // Replace the root by its predecessor and insert x normally into the left sub-tree.

        // In the case that x belongs in the right sub-tree

            // If the right sub-tree is larger than the left sub-tree

                // compare x.key to the successor of the root node. 

                // If x.key is smaller

                    // Replace the root by x and insert the former root into the left sub-tree. 

                // If x.key is larget

                    // Replace the root by its successor and insert x normally into the right sub-tree.
    }

    fn delete(&mut self, value: u32) {
        /*
            Q: What are the running times, in asymptotic notation, for DELETE? 
            
            A:
            
            Q: Are the best and worst cases different asymptotically compared to Insert?

            A:
         */

        // Delete x in the normal way for BST

        // Then fixup the left and right sizes for all nodes

        // Compare the new left and right-sizes of the root node after deletion.

        // If the left-size is larger than the right-size by more than 1
            
            // Replace the root node by its predecessor and re-insert the former root node into the right sub-tree.

        // If the right-size is larger than the left-size by more than 1
        
            // Replace the root node by its successor and re-insert the former root node into the left sub-tree.
    }
}

fn main() {
    /*
        Devise a test case that builds, from a set of 24 elements, 
        a self-balancing BST of this kind through a series of insertions and deletions, 
        resulting in a tree of size 15.
    */
    
    // Create a vector of operations, each a closure taking &mut SelfBalancingTree
    let operations: Vec<Box<dyn Fn(&mut SelfBalancingTree)>> = vec![
        Box::new(|tree| tree.insert(1)),  // Count = 1
        Box::new(|tree| tree.insert(2)),  // Count = 2
        Box::new(|tree| tree.insert(3)),  // Count = 3
        Box::new(|tree| tree.delete(1)),  // Count = 2
        Box::new(|tree| tree.delete(2)),  // Count = 1
        Box::new(|tree| tree.delete(3)),  // Count = 0
        Box::new(|tree| tree.insert(4)),  // Count = 1
        Box::new(|tree| tree.insert(5)),  // Count = 2
        Box::new(|tree| tree.insert(6)),  // Count = 3
        Box::new(|tree| tree.insert(7)),  // Count = 4
        Box::new(|tree| tree.insert(8)),  // Count = 5
        Box::new(|tree| tree.insert(9)),  // Count = 6 
        Box::new(|tree| tree.insert(10)), // Count = 7 
        Box::new(|tree| tree.insert(11)), // Count = 8 
        Box::new(|tree| tree.insert(12)), // Count = 9  
        Box::new(|tree| tree.insert(13)), // Count = 10  
        Box::new(|tree| tree.insert(14)), // Count = 11 
        Box::new(|tree| tree.insert(15)), // Count = 12  
        Box::new(|tree| tree.insert(16)), // Count = 13 
        Box::new(|tree| tree.insert(17)), // Count = 14 
        Box::new(|tree| tree.insert(18)), // Count = 15 
        Box::new(|tree| tree.insert(19)), // Count = 16 
        Box::new(|tree| tree.insert(20)), // Count = 17 
        Box::new(|tree| tree.insert(21)), // Count = 18 
        Box::new(|tree| tree.insert(22)), // Count = 19 
        Box::new(|tree| tree.insert(23)), // Count = 20 
        Box::new(|tree| tree.insert(24)), // Count = 21 
        Box::new(|tree| tree.delete(24)), // Count = 20 
        Box::new(|tree| tree.delete(23)), // Count = 19 
        Box::new(|tree| tree.delete(22)), // Count = 18 
        Box::new(|tree| tree.delete(21)), // Count = 17 
        Box::new(|tree| tree.delete(20)), // Count = 16 
        Box::new(|tree| tree.delete(19)), // Count = 15 
    ];


    // Create our balancing tree
    let mut balancing_tree = SelfBalancingTree::new();


    // Preform The Operations on the Tree
    for op in &operations {
        op(&mut balancing_tree);
    }

}

