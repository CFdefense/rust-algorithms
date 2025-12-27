use std::{
    fmt::{Display, Error, Formatter},
    vec,
};

pub struct TreeNode {
    /*
        A single node in the self-balancing BST.

        Stores the `key`, cached sizes of left/right subtrees (`left_count`/`right_count`),
        and child pointers. Counts are recomputed after structural updates.
    */
    pub key: i32,
    pub left_count: i32,
    pub right_count: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl Display for TreeNode {
    /*
       In rust we must implement the Display trait to be able to print a defined struct.

       Will print the tree node in a readable format.

       This allows us to print the tree node in a readable format with simply calling println!("{}", node);
    */
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "TreeNode {{ key: {}, left_count: {}, right_count: {}, left: {}, right: {}}}",
            self.key,
            self.left_count,
            self.right_count,
            self.left
                .as_ref()
                .map(|n| n.to_string())
                .unwrap_or_else(|| "None".to_string()),
            self.right
                .as_ref()
                .map(|n| n.to_string())
                .unwrap_or_else(|| "None".to_string()),
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
            left: None,
            right: None,
        }
    }

    fn subtree_size(node: &Option<Box<TreeNode>>) -> i32 {
        /*
            Compute the total size of `node`'s subtree.

            This is used for display and verification; counts on nodes are updated
            separately via `recompute_counts` on the owning tree.
        */
        match node {
            Some(n) => 1 + TreeNode::subtree_size(&n.left) + TreeNode::subtree_size(&n.right),
            _ => 0,
        }
    }

    fn bst_property_holds(&self) -> bool {
        /*
            Verify the BST property holds for this node's subtree.
        */
        let left_ok = match &self.left {
            Some(l) => l.key < self.key && l.bst_property_holds(),
            _ => true,
        };
        let right_ok = match &self.right {
            Some(r) => r.key > self.key && r.bst_property_holds(),
            _ => true,
        };
        left_ok && right_ok
    }
}

pub struct BalancedTree {
    /*
        A BST that maintains balance by size-based root adjustments on insert/delete,
        and recursive local rebalancing to ensure |left_size - right_size| ≤ 1 at every node.

        Q: Given a BST of size n constructed and maintained by these self-balancing
           versions of INSERT and DELETE, what is the maximum height (in terms of n)?

        A: Θ(log n). More precisely, height ≤ ⌈log2(n + 1)⌉ due to the fact that
           each node's left and right subtree sizes differ by at most 1.
    */
    root: Option<Box<TreeNode>>,
}

impl Display for BalancedTree {
    /*
       In rust we must implement the Display trait to be able to printa defined struct.

       Will print the tree node in a readable format.

       This allows us to print the tree in a readable format with simply calling println!("{}", tree);
    */
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        fn fmt_node(
            node: &Box<TreeNode>,
            f: &mut Formatter<'_>,
            prefix: &str,
            is_left: bool,
        ) -> Result<(), Error> {
            let connector = if prefix.is_empty() {
                ""
            } else if is_left {
                "├── "
            } else {
                "└── "
            };

            let lr_label = if is_left { "L: " } else { "R: " };
            let l_sz = TreeNode::subtree_size(&node.left);
            let r_sz = TreeNode::subtree_size(&node.right);

            writeln!(
                f,
                "{}{}{}{} [L:{} R:{}]",
                prefix, connector, lr_label, node.key, l_sz, r_sz
            )?;

            let next_prefix_left = if prefix.is_empty() {
                "    "
            } else {
                if is_left { "│   " } else { "    " }
            };

            let new_prefix_left = format!("{}{}", prefix, next_prefix_left);

            if let Some(ref right) = node.right {
                fmt_node(right, f, &new_prefix_left, false)?;
            }

            if let Some(ref left) = node.left {
                fmt_node(left, f, &new_prefix_left, true)?;
            }
            Ok(())
        }

        if let Some(root) = &self.root {
            writeln!(
                f,
                "Root: {} (bst_property_holds: {})",
                root.key,
                root.bst_property_holds()
            )?;
            let l_sz = TreeNode::subtree_size(&root.left);
            let r_sz = TreeNode::subtree_size(&root.right);
            writeln!(f, "{} [L:{} R:{}]", root.key, l_sz, r_sz)?;
            if let Some(ref right) = root.right {
                fmt_node(right, f, "", false)?;
            }
            if let Some(ref left) = root.left {
                fmt_node(left, f, "", true)?;
            }
        } else {
            writeln!(f, "Root: None")?;
        }
        Ok(())
    }
}

impl BalancedTree {
    fn new() -> Self {
        /*
            Create a new, empty `BalancedTree`.
        */
        Self { root: None }
    }

    fn recompute_counts(&mut self) {
        /*
            Recompute `left_count` and `right_count` for all nodes in the tree.
        */
        fn dfs(node: &mut Option<Box<TreeNode>>) -> i32 {
            if let Some(n) = node.as_mut() {
                let l = dfs(&mut n.left);
                let r = dfs(&mut n.right);
                n.left_count = l;
                n.right_count = r;
                1 + l + r
            } else {
                0
            }
        }
        dfs(&mut self.root);
    }

    fn subtree_size_opt(n: &Option<Box<TreeNode>>) -> i32 {
        /*
            Convenience wrapper around `TreeNode::subtree_size`.
        */
        TreeNode::subtree_size(n)
    }

    fn peek_max_key(n: &Option<Box<TreeNode>>) -> Option<i32> {
        /*
            Get the maximum key in a subtree (in-order predecessor of the subtree root).
        */
        let mut cur = n.as_ref();
        let mut last = None;
        while let Some(node) = cur {
            last = Some(node.key);
            cur = node.right.as_ref();
        }
        last
    }

    fn peek_min_key(n: &Option<Box<TreeNode>>) -> Option<i32> {
        /*
            Get the minimum key in a subtree (in-order successor of the subtree root).
        */
        let mut cur = n.as_ref();
        let mut last = None;
        while let Some(node) = cur {
            last = Some(node.key);
            cur = node.left.as_ref();
        }
        last
    }

    fn rebalance_node(node: &mut Box<TreeNode>) -> bool {
        /*
            Locally rebalance a single node until |left_size - right_size| ≤ 1.

            Achieves balance by replacing the node's key with a predecessor/successor
            and re-inserting the former key into the lighter subtree. Returns whether
            any changes were made.
        */
        let mut changed = false;

        // Rebalance the nodes
        loop {
            let l = TreeNode::subtree_size(&node.left);
            let r = TreeNode::subtree_size(&node.right);
            if (l - r).abs() <= 1 {
                break;
            }
            changed = true;
            if l > r {
                if let Some(pred) = Self::peek_max_key(&node.left) {
                    let old_key = node.key;
                    node.key = pred;
                    Self::delete_in(&mut node.left, pred);
                    Self::bst_insert(&mut node.right, old_key);
                } else {
                    break;
                }
            } else {
                if let Some(succ) = Self::peek_min_key(&node.right) {
                    let old_key = node.key;
                    node.key = succ;
                    Self::delete_in(&mut node.right, succ);
                    Self::bst_insert(&mut node.left, old_key);
                } else {
                    break;
                }
            }
        }
        changed
    }

    fn rebalance_all(node: &mut Option<Box<TreeNode>>) -> bool {
        /*
            Post-order traversal that rebalances all nodes. Returns true if any
            node was changed during the pass.
        */
        if let Some(n) = node.as_mut() {
            let mut changed = false;

            // Recurse into children first
            if Self::rebalance_all(&mut n.left) {
                changed = true;
            }

            if Self::rebalance_all(&mut n.right) {
                changed = true;
            }

            // Then fix current node
            if Self::rebalance_node(n) {
                changed = true;
            }

            return changed;
        }
        false
    }

    fn bst_insert(node: &mut Option<Box<TreeNode>>, key: i32) {
        /*
            Standard BST insert into a given subtree, without balancing.
        */
        if node.is_none() {
            *node = Some(Box::new(TreeNode::new(key)));
            return;
        }
        let n = node.as_mut().unwrap();
        if key < n.key {
            Self::bst_insert(&mut n.left, key);
        } else if key > n.key {
            Self::bst_insert(&mut n.right, key);
        }
    }

    fn delete_in(node: &mut Option<Box<TreeNode>>, key: i32) -> bool {
        /*
            Standard BST delete within a given subtree. Returns true if a node was
            removed. Used internally by `insert`, `delete`, and rebalancing steps.
        */
        if node.is_none() {
            return false;
        }
        let should_delete_here;
        {
            let n = node.as_ref().unwrap();
            if key < n.key {
                return Self::delete_in(&mut node.as_mut().unwrap().left, key);
            } else if key > n.key {
                return Self::delete_in(&mut node.as_mut().unwrap().right, key);
            } else {
                should_delete_here = true;
            }
        }
        if should_delete_here {
            // Handle deletion at this node
            let mut cur = node.take().unwrap();
            let left_opt = cur.left.take();
            let right_opt = cur.right.take();
            if left_opt.is_none() && right_opt.is_none() {
                *node = None;
            } else if left_opt.is_some() && right_opt.is_none() {
                *node = left_opt;
            } else if left_opt.is_none() && right_opt.is_some() {
                *node = right_opt;
            } else {
                // both children exist
                let left = left_opt.unwrap();
                let right = right_opt.unwrap();

                // Replace with successor (min of right)
                let mut right_opt2 = Some(right);
                let succ_key = {
                    let mut p = right_opt2.as_ref();
                    let mut last = None;
                    while let Some(nn) = p {
                        last = Some(nn.key);
                        p = nn.left.as_ref();
                    }
                    last.unwrap()
                };
                // remove successor key from right subtree
                Self::delete_in(&mut right_opt2, succ_key);
                let mut new_node = Box::new(TreeNode::new(succ_key));
                new_node.left = Some(left);
                new_node.right = right_opt2;
                *node = Some(new_node);
            }
            return true;
        }
        false
    }

    fn insert(&mut self, x: TreeNode) {
        /*
            Insert a key with root-aware balancing. If one side of the root is larger,
            apply the predecessor/successor rules to keep the two sides within 1, then
            run a fixpoint rebalancing pass across the entire tree.

            Q: What are the running times, in asymptotic notation, for INSERT?

            A: Best Θ(log n) when the tree is already balanced and only a root-local
               adjustment (or none) is needed. Typical Θ(log n) to descend plus
               O(1) rebalances.

               Worst-case Θ(n log n) for a full-tree rebalance pass
               (O(log n) work at many nodes).

            Q: Are the best and worst cases different asymptotically compared to DELETE.

            A: Same as DELETE.
        */
        println!("Insert: {}", x.key);
        // Empty tree
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(x.key)));
            self.recompute_counts();
            return;
        }

        // Ensure counts are up to date
        self.recompute_counts();

        if let Some(root) = self.root.as_mut() {
            if x.key == root.key {
                return;
            }

            let l = Self::subtree_size_opt(&root.left);
            let r = Self::subtree_size_opt(&root.right);

            if x.key < root.key {
                if l > r {
                    if let Some(pred) = Self::peek_max_key(&root.left) {
                        if x.key > pred {
                            // replace parent by x and insert former parent into the right sub-tree
                            let old_key = root.key;
                            root.key = x.key;
                            Self::bst_insert(&mut root.right, old_key);
                        } else {
                            // replace the parent by its predecessor and insert x normally into the left sub-tree
                            let old_key = root.key;
                            let pred_val = pred;
                            root.key = pred_val;
                            Self::delete_in(&mut root.left, pred_val);
                            Self::bst_insert(&mut root.left, x.key);

                            // preserve the former parent key in the right sub-tree
                            Self::bst_insert(&mut root.right, old_key);
                        }
                    } else {
                        // No predecessor, insert normally
                        Self::bst_insert(&mut root.left, x.key);
                    }
                } else {
                    Self::bst_insert(&mut root.left, x.key);
                }
            } else {
                if r > l {
                    if let Some(succ) = Self::peek_min_key(&root.right) {
                        if x.key < succ {
                            // replace parent by x and insert former parent into the left sub-tree
                            let old_key = root.key;
                            root.key = x.key;
                            Self::bst_insert(&mut root.left, old_key);
                        } else {
                            // replace the parent by its successor and insert x normally into the right sub-tree
                            let old_key = root.key;
                            let succ_val = succ;
                            root.key = succ_val;
                            Self::delete_in(&mut root.right, succ_val);
                            Self::bst_insert(&mut root.right, x.key);

                            // preserve the former parent key in the left sub-tree
                            Self::bst_insert(&mut root.left, old_key);
                        }
                    } else {
                        // No successor, insert normally
                        Self::bst_insert(&mut root.right, x.key);
                    }
                } else {
                    Self::bst_insert(&mut root.right, x.key);
                }
            }
        }

        self.recompute_counts();
        // Balance the tree
        loop {
            let changed = Self::rebalance_all(&mut self.root);
            if !changed {
                break;
            }
        }
        self.recompute_counts();
    }

    fn delete(&mut self, x: i32) {
        /*
            Delete a key and then rebalance the tree.

            Q: What are the running times, in asymptotic notation, for DELETE?

            A: Best Θ(log n) when the tree remains balanced and only local work is
               needed; typical Θ(log n) to descend plus small, local rebalancing;
               worst-case Θ(n log n) for a full-tree rebalance pass.

            Q: Are the best and worst cases different asymptotically compared to Insert?

            A: Same as INSERT.
        */
        if Self::delete_in(&mut self.root, x) {
            self.recompute_counts();
            // Rebalance the tree
            loop {
                let changed = Self::rebalance_all(&mut self.root);
                if !changed {
                    break;
                }
            }
            self.recompute_counts();
        }
    }
}

fn main() {
    /*
        Devise a test case that builds, from a set of 24 elements,
        a self-balancing BST of this kind through a series of insertions and deletions,
        resulting in a tree of size 15.
    */

    // Insert and delete 24 elements to build a tree of size 15
    let mut tree = BalancedTree::new();
    let ops_rb: Vec<Box<dyn Fn(&mut BalancedTree)>> = vec![
        Box::new(|t| t.insert(TreeNode::new(1))),
        Box::new(|t| t.insert(TreeNode::new(2))),
        Box::new(|t| t.insert(TreeNode::new(3))),
        Box::new(|t| t.delete(1)),
        Box::new(|t| t.delete(2)),
        Box::new(|t| t.delete(3)),
        Box::new(|t| t.insert(TreeNode::new(4))),
        Box::new(|t| t.insert(TreeNode::new(5))),
        Box::new(|t| t.insert(TreeNode::new(6))),
        Box::new(|t| t.insert(TreeNode::new(7))),
        Box::new(|t| t.insert(TreeNode::new(8))),
        Box::new(|t| t.insert(TreeNode::new(9))),
        Box::new(|t| t.insert(TreeNode::new(10))),
        Box::new(|t| t.insert(TreeNode::new(11))),
        Box::new(|t| t.insert(TreeNode::new(12))),
        Box::new(|t| t.insert(TreeNode::new(13))),
        Box::new(|t| t.insert(TreeNode::new(14))),
        Box::new(|t| t.insert(TreeNode::new(15))),
        Box::new(|t| t.insert(TreeNode::new(16))),
        Box::new(|t| t.insert(TreeNode::new(17))),
        Box::new(|t| t.insert(TreeNode::new(18))),
        Box::new(|t| t.insert(TreeNode::new(19))),
        Box::new(|t| t.insert(TreeNode::new(20))),
        Box::new(|t| t.insert(TreeNode::new(21))),
        Box::new(|t| t.insert(TreeNode::new(22))),
        Box::new(|t| t.insert(TreeNode::new(23))),
        Box::new(|t| t.insert(TreeNode::new(24))),
        Box::new(|t| t.delete(24)),
        Box::new(|t| t.delete(23)),
        Box::new(|t| t.delete(22)),
        Box::new(|t| t.delete(21)),
        Box::new(|t| t.delete(20)),
        Box::new(|t| t.delete(19)),
    ];
    for op in &ops_rb {
        op(&mut tree);
        println!("[Balanced Tree] Tree: {} \n", tree);
    }
}
