use std::{
    fmt::{Display, Error, Formatter},
    mem, vec,
};

pub struct TreeNode {
    pub key: i32,
    pub left_count: i32,
    pub right_count: i32,
    pub right_subtree: Vec<Box<TreeNode>>,
    pub left_subtree: Vec<Box<TreeNode>>,
}

impl Display for TreeNode {
    /*
       Will print the tree node in a readable format.

       This allows us to print the tree node in a readable format with simply calling println!("{}", node);
    */
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "TreeNode {{ key: {}, left_count: {}, right_count: {}, right_subtree: {}, left_subtree: {} }}",
            self.key,
            self.left_count,
            self.right_count,
            self.right_subtree
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("\n, "),
            self.left_subtree
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }
}

impl TreeNode {
    fn new(key: i32) -> Self {
        /*
           Will create a new tree node with the given key.
        */
        Self {
            key,
            left_count: 0,
            right_count: 0,
            right_subtree: Vec::new(),
            left_subtree: Vec::new(),
        }
    }

    fn collect_all_keys(&self) -> (Vec<i32>, Vec<i32>) {
        /*
           Will collect all keys from the left and right subtrees.

           Returns a tuple of two vectors, one for the left subtree and one for the right subtree.

           This is used to print the tree in a readable format.
        */
        let mut left_keys = Vec::new();
        let mut right_keys = Vec::new();

        // Collect all keys from left subtree (including nested ones)
        for node in &self.left_subtree {
            left_keys.push(node.key);
            let (nested_left, nested_right) = node.collect_all_keys();
            // All nested keys are part of the left subtree
            left_keys.extend(nested_left);
            left_keys.extend(nested_right);
        }

        // Collect all keys from right subtree (including nested ones)
        for node in &self.right_subtree {
            right_keys.push(node.key);
            let (nested_left, nested_right) = node.collect_all_keys();
            // All nested keys are part of the right subtree
            right_keys.extend(nested_left);
            right_keys.extend(nested_right);
        }

        (left_keys, right_keys)
    }

    fn get_predecessor_index(&self) -> Option<usize> {
        /*
           Will get the max node's index in the left subtree.

           Ie: The node which comes right before the root node in an in-order traversal.
        */
        self.left_subtree
            .iter()
            .enumerate()
            .max_by_key(|(_, n)| n.key)
            .map(|(idx, _)| idx)
    }

    fn get_successor_index(&self) -> Option<usize> {
        /*
           Will get the min node's index in the right subtree.

           Ie: The node which comes right after the root node in an in-order traversal.
        */
        self.right_subtree
            .iter()
            .enumerate()
            .min_by_key(|(_, n)| n.key)
            .map(|(idx, _)| idx)
    }

    fn remove_from_subtree(subtree: &mut Vec<Box<TreeNode>>, key: i32) -> bool {
        /*
           Will remove the node with the given key from the subtree.

           Returns true if the node was found and removed, false otherwise.

           Tree is only 2 levels deep, so we can just iterate through the subtree and remove the node.
        */
        if let Some(idx) = subtree
            .iter()
            .enumerate()
            .find(|(_, n)| n.key == key)
            .map(|(idx, _)| idx)
        {
            subtree.remove(idx);
            return true;
        }

        false
    }
}

pub struct SelfBalancingTree {
    root: Option<Box<TreeNode>>,
}

impl Display for SelfBalancingTree {
    /*
       In rust we must implement the Display trait to be able to print the tree.

       This allows us to print the tree in a readable format with simply calling println!("{}", tree);
    */
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if let Some(root) = &self.root {
            let (left_keys, right_keys) = root.collect_all_keys();
            writeln!(f, "Root: {}", root.key)?;
            if !left_keys.is_empty() {
                writeln!(f, "Left subtree keys: {:?}", left_keys)?;
            } else {
                writeln!(f, "Left subtree keys: []")?;
            }
            if !right_keys.is_empty() {
                write!(f, "Right subtree keys: {:?}", right_keys)?;
            } else {
                write!(f, "Right subtree keys: []")?;
            }
        } else {
            write!(
                f,
                "Root: None\nLeft subtree keys: []\nRight subtree keys: []"
            )?;
        }
        Ok(())
    }
}

impl SelfBalancingTree {
    /*
       Q: Given a BST of size n constructed and maintained by these self-balancing version of INSERT and DELETE,
       what is the maximum height of the tree (in terms of n)?

       A:

    */
    fn new() -> Self {
        /*
           Will create a new empty tree.
        */
        Self { root: None }
    }

    fn insert(&mut self, x: TreeNode) {
        /*

           Will insert the given node into the tree.

           This is the main function for inserting a node into the tree.

           It will compare the key of the node to the root node and insert it into the appropriate subtree.

           If the tree is unbalanced, it will be balanced by swapping the root node with the predecessor or successor.

           This is done to maintain the BST property.

           Q: What are the running times, in asymptotic notation, for INSERT?

           A:

           Q: Are the best and worst cases different asymptotically compared to DELETE.

           A:
        */

        println!("Insert: {}", x.key);

        // First compare x to the root
        if let Some(root) = self.root.as_mut() {
            if x.key < root.key {
                // In the case that x belongs in the left sub-tree

                // If left sub-tree is larger than right sub-tree
                if root.left_count > root.right_count {
                    // compare x.key to the predecessor of the root node. (max(left sub-tree))

                    if let Some(predecessor_idx) = root.get_predecessor_index() {
                        let predecessor = root.left_subtree.remove(predecessor_idx);
                        root.left_count -= 1; // Decrement before root replacement
                        let predecessor_key = predecessor.key;
                        if x.key > predecessor_key {
                            // If x.key is larger
                            // Replace the root by x and insert the former root into the right sub-tree
                            let old_root: Box<TreeNode> = mem::replace(root, Box::new(x));
                            root.right_subtree.push(old_root);
                            root.right_count += 1;
                        } else {
                            // If x.key is smaller
                            // Replace the root by its predecessor and insert x normally into the left sub-tree.
                            let old_root: Box<TreeNode> = mem::replace(root, predecessor);
                            root.left_subtree.push(Box::new(x));
                            root.right_subtree.push(old_root);
                            root.left_count += 1; // x added
                            root.right_count += 1; // old root added
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

                    if let Some(successor_idx) = root.get_successor_index() {
                        let successor = root.right_subtree.remove(successor_idx);
                        root.right_count -= 1; // Decrement before root replacement
                        let successor_key = successor.key;
                        if x.key < successor_key {
                            // If x.key is smaller
                            // Replace the root by x and insert the former root into the left sub-tree.
                            let old_root: Box<TreeNode> = mem::replace(root, Box::new(x));
                            root.left_subtree.push(old_root);
                            root.left_count += 1;
                        } else {
                            // If x.key is larger
                            // Replace the root by its successor and insert x normally into the right sub-tree.
                            // Move successor to the left sub-tree
                            let old_root: Box<TreeNode> = mem::replace(root, successor);
                            root.right_subtree.push(Box::new(x));
                            root.left_subtree.push(old_root);
                            root.right_count += 1;
                            root.left_count += 1;
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

           Will delete the given node from the tree.

           This is the main function for deleting a node from the tree.

           It will compare the key of the node to the root node and delete it from the appropriate subtree.

           If the tree is unbalanced, it will be balanced by swapping the root node with the predecessor or successor.

           This is done to maintain the BST property.

           Q: What are the running times, in asymptotic notation, for DELETE?

           A:

           Q: Are the best and worst cases different asymptotically compared to Insert?

           A:
        */

        println!("Delete: {}", x);

        // Delete x in the normal way for BST

        // Check if we're deleting the root with no children first
        if let Some(root_ref) = self.root.as_ref() {
            if root_ref.key == x
                && root_ref.left_subtree.is_empty()
                && root_ref.right_subtree.is_empty()
            {
                // Root deletion with no children - set to None
                self.root = None;
                return;
            }
        }

        if let Some(root) = self.root.as_mut() {
            if x == root.key {
                // x == root.key - deleting the root itself (with children)

                // Replace the root node by its predecessor
                if let Some(predecessor_idx) = root.get_predecessor_index() {
                    let predecessor = root.left_subtree.remove(predecessor_idx);
                    let _ = mem::replace(root, predecessor);
                    root.left_count -= 1;
                } else if let Some(successor_idx) = root.get_successor_index() {
                    // No predecessor found, so we need to replace the root with the successor
                    let successor = root.right_subtree.remove(successor_idx);
                    let _ = mem::replace(root, successor);
                    root.left_count += 1;
                    root.right_count -= 1;
                }
            } else {
                // x != root.key - search both subtrees since BST property may not hold due to balancing
                // Try left subtree first
                if TreeNode::remove_from_subtree(&mut root.left_subtree, x) {
                    root.left_count -= 1;
                } else if TreeNode::remove_from_subtree(&mut root.right_subtree, x) {
                    // Not in left, try right subtree
                    root.right_count -= 1;
                } else {
                    panic!("x {} not found in tree", x);
                }
            }

            // Then fixup the left and right sizes for all nodes

            // Compare the new left and right-sizes of the root node after deletion.
            if root.left_count > root.right_count + 1 && !root.left_subtree.is_empty() {
                // If the left-size is larger than the right-size by more than 1
                // Replace the root node by its predecessor and re-insert the former root node into the right sub-tree.
                if let Some(predecessor_idx) = root.get_predecessor_index() {
                    let predecessor = root.left_subtree.remove(predecessor_idx);
                    let old_root: Box<TreeNode> = mem::replace(root, predecessor);
                    root.right_subtree.push(old_root);
                    root.right_count += 1;
                    root.left_count -= 1;
                }
            } else if root.right_count > root.left_count + 1 && !root.right_subtree.is_empty() {
                // If the right-size is larger than the left-size by more than 1
                // Replace the root node by its successor and re-insert the former root node into the left sub-tree.
                if let Some(successor_idx) = root.get_successor_index() {
                    let successor = root.right_subtree.remove(successor_idx);
                    let old_root: Box<TreeNode> = mem::replace(root, successor);
                    root.left_subtree.push(old_root);
                    root.left_count += 1;
                    root.right_count -= 1;
                }
            }
        }
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
        Box::new(|tree| tree.insert(TreeNode::new(1))), // Count = 1
        Box::new(|tree| tree.insert(TreeNode::new(2))), // Count = 2
        Box::new(|tree| tree.insert(TreeNode::new(3))), // Count = 3
        Box::new(|tree| tree.delete(1)),                // Count = 2
        Box::new(|tree| tree.delete(2)),                // Count = 1
        Box::new(|tree| tree.delete(3)),                // Count = 0
        Box::new(|tree| tree.insert(TreeNode::new(4))), // Count = 1
        Box::new(|tree| tree.insert(TreeNode::new(5))), // Count = 2
        Box::new(|tree| tree.insert(TreeNode::new(6))), // Count = 3
        Box::new(|tree| tree.insert(TreeNode::new(7))), // Count = 4
        Box::new(|tree| tree.insert(TreeNode::new(8))), // Count = 5
        Box::new(|tree| tree.insert(TreeNode::new(9))), // Count = 6
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
        Box::new(|tree| tree.delete(24)),               // Count = 20
        Box::new(|tree| tree.delete(23)),               // Count = 19
        Box::new(|tree| tree.delete(22)),               // Count = 18
        Box::new(|tree| tree.delete(21)),               // Count = 17
        Box::new(|tree| tree.delete(20)),               // Count = 16
        Box::new(|tree| tree.delete(19)),               // Count = 15
    ];

    // Create our balancing tree
    let mut balancing_tree = SelfBalancingTree::new();

    // Preform The Operations on the Tree
    for op in &operations {
        op(&mut balancing_tree);

        println!("Tree: {} \n", balancing_tree);
    }
}
