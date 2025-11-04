use std::{vec, mem};

pub struct TreeNode {
    pub key: i32,
    pub left_count: i32,
    pub right_count: i32,
    pub right_subtree: Vec<Box<TreeNode>>,
    pub left_subtree: Vec<Box<TreeNode>>,
}

impl TreeNode {
    fn new(key: i32) -> Self {
        Self { key, left_count: 0, right_count: 0, right_subtree: Vec::new(), left_subtree: Vec::new() }
    }

    fn get_predecessor(&self) -> Option<&TreeNode> {
        self.left_subtree
            .iter()
            .map(|b| b.as_ref())
            .max_by_key(|n| n.key)
    } 

    fn get_successor(&self) -> Option<&TreeNode> {
        self.right_subtree
            .iter()
            .map(|b| b.as_ref())
            .min_by_key(|n| n.key)
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

    fn insert(&mut self, x: TreeNode) {
        /*
            Q: What are the running times, in asymptotic notation, for INSERT? 
            
            A:
            
            Q: Are the best and worst cases different asymptotically compared to DELETE.

            A:
         */

        // First compare x to the root
        if let Some(root) = self.root.as_mut() {
            if x.key < root.key {
                // In the case that x belongs in the left sub-tree

                    // If left sub-tree is larger than right sub-tree
                    if root.left_count > root.right_count {
                        // compare x.key to the predecessor of the root node. (max(left sub-tree))

                        if let Some(predecessor) = root.get_predecessor() {
                            if x.key > predecessor.key {
                                // If x.key is larger
                                // Replace the root by x and insert the former root into the right sub-tree


                            } else {
                                // If x.key is smaller
                                // Replace the root by its predecessor and insert x normally into the left sub-tree.

                            }
                        }
                    } else {
                        // Insert normally into left sub-tree
                        root.left_subtree.push(Box::new(x));
                        root.left_count += 1;

                    }

            } else {
                // In the case that x belongs in the right sub-tree

                    // If the right sub-tree is larger than the left sub-tree
                    if root.right_count > root.left_count {
                        // compare x.key to the successor of the root node. (min(right sub-tree))

                        if let Some(successor) = root.get_successor() {
                            if x.key < successor.key {
                                // If x.key is smaller
                                // Replace the root by x and insert the former root into the left sub-tree. 
                                let temp = TreeNode::new(root.key);
                                *root = Box::new(x);
                                root.left_subtree.push(Box::new(temp));
                                root.right_count += 1;
                                let old_root = 
                            } else {
                                // If x.key is larget
                                // Replace the root by its successor and insert x normally into the right sub-tree.
                                let old_root: Box<TreeNode> = mem::replace(root, Box::new(x));
                                root.right_subtree.push(old_root);
                                root.right_count += 1;
                            }
                        }
                    } else {
                        // Insert normally into right sub-tree
                        root.right_subtree.push(Box::new(x));
                        root.right_count += 1;
                    }
            }
        } else {
            // No root exists, create new root
            self.root = Some(Box::new(x));
        }
    }

    fn delete(&mut self, x: i32) {
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
        Box::new(|tree| tree.insert(TreeNode::new(1))),  // Count = 1
        Box::new(|tree| tree.insert(TreeNode::new(2))),  // Count = 2
        Box::new(|tree| tree.insert(TreeNode::new(3))),  // Count = 3
        Box::new(|tree| tree.delete(1)),                 // Count = 2
        Box::new(|tree| tree.delete(2)),                 // Count = 1
        Box::new(|tree| tree.delete(3)),                 // Count = 0
        Box::new(|tree| tree.insert(TreeNode::new(4))),  // Count = 1
        Box::new(|tree| tree.insert(TreeNode::new(5))),  // Count = 2
        Box::new(|tree| tree.insert(TreeNode::new(6))),  // Count = 3
        Box::new(|tree| tree.insert(TreeNode::new(7))),  // Count = 4
        Box::new(|tree| tree.insert(TreeNode::new(8))),  // Count = 5
        Box::new(|tree| tree.insert(TreeNode::new(9))),  // Count = 6 
        Box::new(|tree| tree.insert(TreeNode::new(10))), // Count = 7 
        Box::new(|tree| tree.insert(TreeNode::new(11))), // Count = 8 
        Box::new(|tree| tree.insert(TreeNode::new(12))), // Count = 9  
        Box::new(|tree| tree.insert(TreeNode::new(13))), // Count = 10  
        Box::new(|tree| tree.insert(TreeNode::new(14))), // Count = 11 
        Box::new(|tree| tree.insert(TreeNode::new(15))), // Count = 12  
        Box::new(|tree| tree.insert(TreeNode::new(16))), // Count = 13 
        Box::new(|tree| tree.insert(TreeNode::new(17))), // Count = 14 
        Box::new(|tree| tree.insert(TreeNode::new(18))), // Count = 15 
        Box::new(|tree| tree.insert(TreeNode::new(19))), // Count = 16 
        Box::new(|tree| tree.insert(TreeNode::new(20))), // Count = 17 
        Box::new(|tree| tree.insert(TreeNode::new(21))), // Count = 18 
        Box::new(|tree| tree.insert(TreeNode::new(22))), // Count = 19 
        Box::new(|tree| tree.insert(TreeNode::new(23))), // Count = 20 
        Box::new(|tree| tree.insert(TreeNode::new(24))), // Count = 21 
        Box::new(|tree| tree.delete(24)),                // Count = 20 
        Box::new(|tree| tree.delete(23)),                // Count = 19 
        Box::new(|tree| tree.delete(22)),                // Count = 18 
        Box::new(|tree| tree.delete(1)),                 // Count = 17 
        Box::new(|tree| tree.delete(20)),                // Count = 16 
        Box::new(|tree| tree.delete(19)),                // Count = 15 
    ];


    // Create our balancing tree
    let mut balancing_tree = SelfBalancingTree::new();


    // Preform The Operations on the Tree
    for op in &operations {
        op(&mut balancing_tree);
    }

}

