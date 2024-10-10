pub struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }

    /// Checks if the tree satisfies the Binary Search Tree (BST) property.
    /// The tree is a BST if, for every node, the values in its left subtree
    /// are smaller than or equal to the node's key, and the values in its right subtree
    /// are greater.
    pub fn is_bst(&self) -> bool {
        self.rec_is_bst(Some(0), u32::MIN, u32::MAX)
    }

    /// A private recursive function that checks if the subtree rooted at `node_id`
    /// satisfies the BST property. Each node's value must lie between `min` and `max`.
    fn rec_is_bst(&self, node_id: Option<usize>, min: u32, max: u32) -> bool {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            // If the node's key violates the BST property, return false
            if !(node.key > min && node.key <= max) {
                return false;
            }

            // Recursively check the left and right subtrees, updating bounds
            return self.rec_is_bst(node.id_left, min, node.key)
                && self.rec_is_bst(node.id_right, node.key, max);
        }

        true // Return true if the node_id is None (leaf node's child)
    }

    /// Finds the maximum path sum in the tree.
    /// The maximum path sum is the highest sum of node values along any path
    /// from one leaf to another (may or may not pass through the root).
    pub fn max_path_sum(&self) -> u32 {
        let (_, sum) = self.rec_max_path_sum(Some(0));
        sum
    }

    /// A private recursive function to find the maximum path sum in the subtree
    /// rooted at `node_id`. It returns a tuple where:
    /// - The first value is the maximum sum that can be obtained by including
    ///   the current node and one of its subtrees.
    /// - The second value is the maximum path sum found so far in the subtree.
    fn rec_max_path_sum(&self, node_id: Option<usize>) -> (u32, u32) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let (max_left, best_left) = self.rec_max_path_sum(node.id_left);
            let (max_right, best_right) = self.rec_max_path_sum(node.id_right);

            let max_node = node.key + max_left.max(max_right);

            // The best path sum either includes the current node or comes from a subtree
            let current_path = node.key + max_left + max_right;
            let best_node = best_left.max(best_right).max(current_path);

            return (max_node, best_node);
        }

        (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.sum(), 64);
    }

    #[test]
    fn test_is_bst_single_node() {
        let tree = Tree::with_root(42);
        assert_eq!(tree.is_bst(), true); // A single-node tree is a valid BST
    }

    #[test]
    fn test_is_bst_left_skewed_valid() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(1, 3, true);
        tree.add_node(2, 2, true);
        assert_eq!(tree.is_bst(), true); // Valid left-skewed BST
    }

    #[test]
    fn test_is_bst_left_skewed_invalid() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(1, 12, true); // Invalid: 12 > 10
        assert_eq!(tree.is_bst(), false);
    }

    #[test]
    fn test_is_bst_right_skewed_valid() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 15, false);
        tree.add_node(1, 20, false);
        tree.add_node(2, 25, false);
        assert_eq!(tree.is_bst(), true); // Valid right-skewed BST
    }

    #[test]
    fn test_is_bst_right_skewed_invalid() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 15, false);
        tree.add_node(1, 5, false); // Invalid: 5 < 10
        assert_eq!(tree.is_bst(), false);
    }

    #[test]
    fn test_is_bst_with_duplicates_valid() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(1, 10, false); // Duplicate on the left is valid
        assert_eq!(tree.is_bst(), true);
    }

    #[test]
    fn test_is_bst_with_duplicates_invalid() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 10, false); // Duplicate on the right should be invalid
        assert_eq!(tree.is_bst(), false);
    }

    #[test]
    fn test_is_bst_deep_violation() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(1, 3, true);
        tree.add_node(2, 4, false);
        tree.add_node(3, 1, true);
        tree.add_node(3, 15, false); // Invalid: 15 > 10 should not be in the left subtree
        assert_eq!(tree.is_bst(), false);
    }

    #[test]
    fn test_max_path_sum_single_node() {
        let tree = Tree::with_root(42);
        assert_eq!(tree.max_path_sum(), 42); // Single-node tree
    }

    #[test]
    fn test_max_path_sum_multiple_paths() {
        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(0, 15, false);
        tree.add_node(1, 3, true);
        tree.add_node(1, 7, false);
        tree.add_node(2, 12, true);
        tree.add_node(2, 20, false);
        assert_eq!(tree.max_path_sum(), 57); // Path: 7 -> 5 -> 10 -> 15 -> 20
    }

    #[test]
    fn test_max_path_sum_all_left_children() {
        let mut tree = Tree::with_root(1);
        tree.add_node(0, 2, true);
        tree.add_node(1, 3, true);
        tree.add_node(2, 4, true);
        assert_eq!(tree.max_path_sum(), 10); // 4 + 3 + 2 + 1 = 10
    }

    #[test]
    fn test_max_path_sum_all_right_children() {
        let mut tree = Tree::with_root(1);
        tree.add_node(0, 2, false);
        tree.add_node(1, 3, false);
        tree.add_node(2, 4, false);
        assert_eq!(tree.max_path_sum(), 10); // 1 + 2 + 3 + 4 = 10
    }
}
