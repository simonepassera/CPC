use std::cmp::max;
use std::cmp::min;

/// A node in the segment tree that represents a segment of an array.
pub struct Node<T> {
    /// The range of the array represented by this node (start, end).
    range: (usize, usize),

    /// The value stored at this node.
    node_value: T,

    /// Lazy propagation value for deferred updates.
    lazy: Option<T>,

    /// Left child node of the current node.
    left: Option<Box<Node<T>>>,

    /// Right child node of the current node.
    right: Option<Box<Node<T>>>,
}

impl<T: Clone + std::ops::Add<Output = T> + std::cmp::Ord> Node<T> {
    /// Creates a new node with the specified range and initial value.
    ///
    /// # Parameters
    /// - `range`: The range of the array this node represents.
    /// - `max`: The value of the node.
    ///
    /// # Returns
    /// A new `Node` instance with the specified range and value.
    fn new(range: (usize, usize), max: T) -> Self {
        Self {
            range,
            node_value: max,
            lazy: None,
            left: None,
            right: None,
        }
    }

    /// Builds a segment tree from the given array.
    ///
    /// # Parameters
    /// - `arr`: The array to build the segment tree from.
    /// - `range`: The range of indices in the array that this node represents.
    ///
    /// # Returns
    /// An `Option<Box<Node<T>>>` representing the root of the segment tree.
    fn build(arr: &[T], range: (usize, usize)) -> Option<Box<Self>> {
        // Base case: if the range is invalid, return None.
        if range.0 > range.1 {
            return None;
        }

        // Calculate the midpoint of the current range.
        let mid = (range.0 + range.1) / 2;

        // Initialize a new node for the current range, using the first element.
        let mut node = Self::new(range, arr[range.0].clone());

        // If the range has more than one element, build left and right subtrees.
        if range.0 < range.1 {
            node.left = Self::build(arr, (range.0, mid));
            node.right = Self::build(arr, (mid + 1, range.1));

            // The node value is the maximum of the left and right subtrees.
            node.node_value = match (&node.left, &node.right) {
                (Some(left), Some(right)) => max(left.node_value.clone(), right.node_value.clone()),
                (Some(left), None) => left.node_value.clone(),
                (None, Some(right)) => right.node_value.clone(),
                (None, None) => arr[range.0].clone(),
            };
        }

        // Return the newly created node wrapped in an `Option<Box>`.
        Some(Box::new(node))
    }

    /// Queries the segment tree for the value in the specified range.
    ///
    /// # Parameters
    /// - `range`: The range to query in the segment tree.
    ///
    /// # Returns
    /// An `Option<T>` containing the value in the queried range (if any).
    fn query(&mut self, range: (usize, usize)) -> Option<T> {
        // If the current range does not overlap with the query range, return None.
        if self.range.0 > range.1 || self.range.1 < range.0 {
            return None;
        }

        // Apply any pending updates (lazy propagation).
        if self.lazy.is_some() {
            self.node_value = min(self.node_value.clone(), self.lazy.clone().unwrap());

            // If this node is not a leaf, propagate the lazy value to children.
            if self.range.0 != self.range.1 {
                if let Some(left) = &mut self.left {
                    if let Some(lazy_value) = &left.lazy {
                        left.lazy = Some(min(lazy_value.clone(), self.lazy.clone().unwrap()));
                    } else {
                        left.lazy = Some(self.lazy.clone().unwrap());
                    }
                }

                if let Some(right) = &mut self.right {
                    if let Some(lazy_value) = &right.lazy {
                        right.lazy = Some(min(lazy_value.clone(), self.lazy.clone().unwrap()));
                    } else {
                        right.lazy = Some(self.lazy.clone().unwrap());
                    }
                }
            }

            // Clear the lazy value after it is propagated.
            self.lazy = None;
        }

        // If the current range is fully inside the query range, return the node value.
        if self.range.0 >= range.0 && self.range.1 <= range.1 {
            return Some(self.node_value.clone());
        }

        // Otherwise, split the query into left and right children.
        let mid = (self.range.0 + self.range.1) / 2;

        // Query the left or right subtree depending on the range.
        if range.1 <= mid {
            if let Some(left) = &mut self.left {
                left.query(range)
            } else {
                None
            }
        } else if range.0 > mid {
            if let Some(ref mut right) = self.right {
                right.query(range)
            } else {
                None
            }
        } else {
            match (&mut self.left, &mut self.right) {
                (Some(left), Some(right)) => {
                    match (left.query((range.0, mid)), right.query((mid + 1, range.1))) {
                        (Some(left_query), Some(right_query)) => Some(max(left_query, right_query)),
                        (Some(left_query), None) => Some(left_query),
                        (None, Some(right_query)) => Some(right_query),
                        (None, None) => None,
                    }
                }
                (Some(left), None) => left.query((range.0, mid)),
                (None, Some(right)) => right.query((mid + 1, range.1)),
                (None, None) => None,
            }
        }
    }

    /// Updates the segment tree in the specified range with a new value.
    ///
    /// # Parameters
    /// - `range`: The range to update in the segment tree.
    /// - `value`: The value to update the segment tree with.
    fn update(&mut self, range: (usize, usize), value: T) {
        // Apply any pending updates (lazy propagation).
        if self.lazy.is_some() {
            self.node_value = min(self.node_value.clone(), self.lazy.clone().unwrap());

            // If this node is not a leaf, propagate the lazy value to children.
            if self.range.0 != self.range.1 {
                if let Some(left) = &mut self.left {
                    left.lazy = min(left.lazy.clone(), self.lazy.clone());
                }

                if let Some(right) = &mut self.right {
                    right.lazy = min(right.lazy.clone(), self.lazy.clone());
                }
            }

            // Clear the lazy value after it is propagated.
            self.lazy = None;
        }

        // If the current range does not overlap with the update range, return early.
        if self.range.0 > range.1 || self.range.1 < range.0 {
            return;
        }

        // If the current range is fully inside the update range, update the node value.
        if self.range.0 >= range.0 && self.range.1 <= range.1 {
            self.node_value = min(self.node_value.clone(), value.clone());

            // Propagate the update to the children if not a leaf.
            if self.range.0 != self.range.1 {
                if let Some(left) = &mut self.left {
                    if let Some(lazy_value) = &left.lazy {
                        left.lazy = Some(min(lazy_value.clone(), value.clone()));
                    } else {
                        left.lazy = Some(value.clone());
                    }
                }

                if let Some(right) = &mut self.right {
                    if let Some(lazy_value) = &right.lazy {
                        right.lazy = Some(min(lazy_value.clone(), value.clone()));
                    } else {
                        right.lazy = Some(value.clone());
                    }
                }
            }

            return;
        }

        // Otherwise, propagate the update to the left and right subtrees.
        if let Some(left) = &mut self.left {
            left.update(range, value.clone());
        }

        if let Some(right) = &mut self.right {
            right.update(range, value.clone());
        }

        // Recalculate the node value after the update.
        if let Some(left) = &self.left {
            if let Some(right) = &self.right {
                self.node_value = max(left.node_value.clone(), right.node_value.clone());
            } else {
                self.node_value = left.node_value.clone();
            }
        } else if let Some(right) = &self.right {
            self.node_value = right.node_value.clone();
        }
    }
}

/// A segment tree data structure.
pub struct SegmentTree<T> {
    /// The root of the segment tree.
    root: Option<Box<Node<T>>>,
}

impl<T: Clone + std::ops::Add<Output = T> + std::cmp::Ord> SegmentTree<T> {
    /// Creates a new segment tree from the given array.
    ///
    /// # Parameters
    /// - `arr`: The array to initialize the segment tree with.
    /// - `range`: The range of the array to build the segment tree for.
    ///
    /// # Returns
    /// A new `SegmentTree` instance.
    pub fn new(arr: &[T], range: (usize, usize)) -> Self {
        Self {
            root: Node::build(arr, range),
        }
    }

    /// Queries the segment tree for the vmaximum alue in the specified range.
    ///
    /// # Parameters
    /// - `range`: The range to query in the segment tree.
    ///
    /// # Returns
    /// An `Option<T>` containing the maximim value in the queried range (if any).
    pub fn query(&mut self, range: (usize, usize)) -> Option<T> {
        if let Some(node) = &mut self.root {
            return Node::query(node, range);
        }

        None
    }

    /// Updates the segment tree in the specified range with a new value.
    ///
    /// # Parameters
    /// - `range`: The range to update in the segment tree.
    /// - `value`: The value to update the segment tree with.
    pub fn update(&mut self, range: (usize, usize), value: T) {
        if let Some(node) = &mut self.root {
            Node::update(node, range, value);
        }
    }
}

/// A node in the segment tree that represents a segment of an array and keeps track of counts to support the 'is_there' operation.
pub struct NodeSegments {
    /// The range of the array represented by this node (start, end).
    range: (usize, usize),

    /// Store the answer for each possible k value of 'is_there' operation, represented as a vector.
    counts: Vec<usize>,

    /// The lazy propagation value for deferred updates.
    lazy: i32,

    /// The left child node of the current node.
    left: Option<Box<NodeSegments>>,

    /// The right child node of the current node.
    right: Option<Box<NodeSegments>>,
}

impl NodeSegments {
    /// Creates a new node with the specified range and the maximum value for k.
    ///
    /// # Parameters
    /// - `range`: The range of the array that this node represents.
    /// - `max_k`: The maximum value for k (used to initialize the counts).
    ///
    /// # Returns
    /// A new `NodeSegments` instance representing the segment.
    fn new(range: (usize, usize), max_k: usize) -> Self {
        Self {
            range,
            counts: vec![0; max_k + 1],
            lazy: 0,
            left: None,
            right: None,
        }
    }

    /// Builds the segment tree from the given range and the maximum value for k.
    ///
    /// # Parameters
    /// - `range`: The range to be represented by the node.
    /// - `max_k`: The maximum possible value for counts (used to size the counts vector).
    ///
    /// # Returns
    /// An `Option<Box<NodeSegments>>` containing the root of the segment tree.
    fn build(range: (usize, usize), max_k: usize) -> Option<Box<Self>> {
        // If the range is invalid (start > end), return None.
        if range.0 > range.1 {
            return None;
        }

        // Create a new node for the current range.
        let mut node = Box::new(Self::new(range, max_k));

        // If the range contains more than one element, recursively build the left and right children.
        if range.0 < range.1 {
            let mid = (range.0 + range.1) / 2;

            node.left = Self::build((range.0, mid), max_k);
            node.right = Self::build((mid + 1, range.1), max_k);

            // Initialize the counts at this node based on the counts of the left and right children.
            node.counts[0] =
                node.left.as_ref().unwrap().counts[0] + node.right.as_ref().unwrap().counts[0];
        } else {
            node.counts[0] = 1;
        }

        // Return the newly created node.
        Some(node)
    }

    /// Updates the segment tree for the given range with a specified value.
    ///
    /// # Parameters
    /// - `range`: The range to update in the segment tree.
    /// - `value`: The value to apply to the counts in the specified range.
    ///
    /// # Updates
    /// The node's counts are updated according to the given value, and the lazy propagation
    /// is used to delay updates to child nodes.
    fn update(&mut self, range: (usize, usize), value: i32) {
        // Apply any pending lazy updates before proceeding.
        if self.lazy != 0 {
            let max_k = self.counts.len() - 1;
            let mut new_counts = vec![0; max_k + 1];

            // Apply the lazy value to the current counts.
            for k in 0..=max_k {
                if (k as i32 + self.lazy >= 0) && (k as i32 + self.lazy <= max_k as i32) {
                    new_counts[(k as i32 + self.lazy) as usize] += self.counts[k];
                }
            }

            // Update the counts and propagate the lazy value to the children if necessary.
            self.counts = new_counts;

            // If this is not a leaf node, propagate the lazy value to the children.
            if self.range.0 != self.range.1 {
                if let Some(left) = &mut self.left {
                    left.lazy += self.lazy;
                }

                if let Some(right) = &mut self.right {
                    right.lazy += self.lazy;
                }
            }

            // Clear the lazy value after applying it.
            self.lazy = 0;
        }

        // If the current range does not overlap with the update range, return early.
        if self.range.0 > range.1 || self.range.1 < range.0 {
            return;
        }

        // If the current range is fully within the update range, update the counts.
        if self.range.0 >= range.0 && self.range.1 <= range.1 {
            let max_k = self.counts.len() - 1;
            let mut new_counts = vec![0; max_k + 1];

            // Apply the update value to the current counts.
            for k in 0..=max_k {
                if (k as i32 + value >= 0) && (k as i32 + value <= max_k as i32) {
                    new_counts[(k as i32 + value) as usize] += self.counts[k];
                }
            }

            // Update the counts at this node.
            self.counts = new_counts;

            // If the node is not a leaf, propagate the update to the children.
            if self.range.0 != self.range.1 {
                if let Some(left) = &mut self.left {
                    left.lazy += value;
                }

                if let Some(right) = &mut self.right {
                    right.lazy += value;
                }
            }

            return;
        }

        // Otherwise, propagate the update to the left and right children.
        if let Some(left) = &mut self.left {
            left.update(range, value);
        }

        if let Some(right) = &mut self.right {
            right.update(range, value);
        }

        // Recalculate the counts at this node based on the updated children.
        let max_k = self.counts.len() - 1;
        self.counts = vec![0; max_k + 1];

        if let Some(left) = &self.left {
            for k in 0..=max_k {
                self.counts[k] += left.counts[k];
            }
        }

        if let Some(right) = &self.right {
            for k in 0..=max_k {
                self.counts[k] += right.counts[k];
            }
        }
    }

    /// Queries the segment tree to check if exists a position p, such that exactly k segments contain position p.
    ///
    /// # Parameters
    /// - `range`: The range to query in the segment tree.
    /// - `k`: Number of segments.
    ///
    /// # Returns
    /// `true` if exists a position contains `k` segments in the specified range, otherwise `false`.
    fn query(&mut self, range: (usize, usize), k: usize) -> bool {
        // Apply any pending lazy updates before querying.
        if self.lazy != 0 {
            let max_k = self.counts.len() - 1;
            let mut new_counts = vec![0; max_k + 1];

            // Apply the lazy value to the current counts.
            for k in 0..=max_k {
                if (k as i32 + self.lazy >= 0) && (k as i32 + self.lazy <= max_k as i32) {
                    new_counts[(k as i32 + self.lazy) as usize] += self.counts[k];
                }
            }

            // Update the counts and propagate the lazy value to the children if necessary.
            self.counts = new_counts;

            // If this is not a leaf node, propagate the lazy value to the children.
            if self.range.0 != self.range.1 {
                if let Some(left) = &mut self.left {
                    left.lazy += self.lazy;
                }

                if let Some(right) = &mut self.right {
                    right.lazy += self.lazy;
                }
            }

            // Clear the lazy value after applying it.
            self.lazy = 0;
        }

        // If the current range does not overlap with the query range, return false.
        if self.range.0 > range.1 || self.range.1 < range.0 {
            return false;
        }

        // If the current range is fully inside the query range, return whether `k` exists.
        if self.range.0 >= range.0 && self.range.1 <= range.1 {
            return self.counts[k] > 0;
        }

        // Otherwise, recursively query the left and right children.
        let mut result = false;

        if let Some(left) = &mut self.left {
            result |= left.query(range, k);
        }

        if let Some(right) = &mut self.right {
            result |= right.query(range, k);
        }

        result
    }

    /// Performs the 'is_there' operation.
    ///
    /// # Parameters
    /// - `n`: The size of the array.
    /// - `segments`: A list of segments.
    /// - `queries`: A list of queries to check for specific values in specified ranges.
    ///
    /// # Returns
    /// A vector of booleans representing the results of each query.
    pub fn is_there(
        n: usize,
        segments: Vec<(usize, usize)>,
        queries: Vec<(usize, usize, usize)>,
    ) -> Vec<bool> {
        // Build the segment tree for the range [0, n-1].
        let mut tree = NodeSegments::build((0, n - 1), n).unwrap();

        // Apply the update ranges.
        for seg in segments {
            tree.update(seg, 1);
        }

        // Execute the queries and collect the results.
        let mut results = Vec::new();

        for (i, j, k) in queries {
            results.push(tree.query((i, j), k));
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_and_max_0() {
        let arr = [1, 4, 2, 3, 4];
        let mut tree = SegmentTree::new(&arr, (0, arr.len() - 1));

        assert_eq!(tree.query((4, 4)).unwrap(), 4);
        assert_eq!(tree.query((4, 4)).unwrap(), 4);
        assert_eq!(tree.query((2, 3)).unwrap(), 3);
        assert_eq!(tree.query((0, 2)).unwrap(), 4);
        assert_eq!(tree.query((0, 3)).unwrap(), 4);
    }

    #[test]
    fn min_and_max_1() {
        let arr = [9, 4, 1, 6, 5, 10, 6, 8, 7, 4];
        let mut tree = SegmentTree::new(&arr, (0, arr.len() - 1));

        tree.update((5, 6), 10);
        assert_eq!(tree.query((4, 7)).unwrap(), 10);
        assert_eq!(tree.query((7, 7)).unwrap(), 8);
        assert_eq!(tree.query((3, 9)).unwrap(), 10);
        assert_eq!(tree.query((5, 7)).unwrap(), 10);
        assert_eq!(tree.query((9, 9)).unwrap(), 4);
        tree.update((2, 9), 4);
        assert_eq!(tree.query((1, 3)).unwrap(), 4);
        assert_eq!(tree.query((0, 8)).unwrap(), 9);
        assert_eq!(tree.query((8, 9)).unwrap(), 4);
    }

    #[test]
    fn min_and_max_2() {
        let arr = [8, 1, 7, 3, 4, 5, 5, 5, 3, 5];
        let mut tree = SegmentTree::new(&arr, (0, arr.len() - 1));

        tree.update((3, 8), 5);
        tree.update((9, 9), 5);
        assert_eq!(tree.query((4, 8)).unwrap(), 5);
        tree.update((4, 4), 4);
        tree.update((9, 9), 5);
        tree.update((8, 8), 3);
        assert_eq!(tree.query((7, 9)).unwrap(), 5);
        tree.update((4, 7), 5);
        tree.update((3, 5), 3);
        tree.update((1, 6), 3);
    }

    #[test]
    fn min_and_max_3() {
        let arr = [2, 5, 10, 9, 1, 7, 8, 7, 4, 1];
        let mut tree = SegmentTree::new(&arr, (0, arr.len() - 1));

        assert_eq!(tree.query((1, 7)).unwrap(), 10);
        assert_eq!(tree.query((5, 8)).unwrap(), 8);

        tree.update((7, 9), 1);
        tree.update((1, 5), 5);

        assert_eq!(tree.query((3, 8)).unwrap(), 8);
        assert_eq!(tree.query((8, 8)).unwrap(), 1);

        tree.update((9, 9), 1);
        tree.update((0, 0), 2);

        assert_eq!(tree.query((5, 7)).unwrap(), 8);
        assert_eq!(tree.query((4, 5)).unwrap(), 5);
    }

    #[test]
    fn min_and_max_4() {
        let arr = [
            6, 5, 8, 9, 9, 9, 13, 18, 9, 11, 3, 16, 16, 2, 14, 17, 9, 9, 3, 11,
        ];
        let mut tree = SegmentTree::new(&arr, (0, arr.len() - 1));

        tree.update((18, 19), 11);
        assert_eq!(tree.query((4, 15)).unwrap(), 18);
        assert_eq!(tree.query((9, 12)).unwrap(), 16);
        tree.update((15, 19), 11);
        tree.update((0, 8), 9);
        assert_eq!(tree.query((12, 12)).unwrap(), 16);
        assert_eq!(tree.query((1, 8)).unwrap(), 9);
        assert_eq!(tree.query((4, 16)).unwrap(), 16);
        tree.update((15, 17), 9);
        tree.update((13, 16), 2);
        assert_eq!(tree.query((0, 5)).unwrap(), 9);
        assert_eq!(tree.query((0, 13)).unwrap(), 16);
        assert_eq!(tree.query((5, 18)).unwrap(), 16);
        assert_eq!(tree.query((5, 13)).unwrap(), 16);
        assert_eq!(tree.query((0, 16)).unwrap(), 16);
        assert_eq!(tree.query((14, 16)).unwrap(), 2);
        tree.update((10, 14), 14);
        assert_eq!(tree.query((5, 7)).unwrap(), 9);
        tree.update((14, 14), 14);
        tree.update((17, 18), 9);
    }

    #[test]
    fn is_there_0() {
        assert_eq!(
            NodeSegments::is_there(
                10,
                vec![
                    (8, 9),
                    (3, 8),
                    (4, 6),
                    (1, 1),
                    (5, 9),
                    (6, 7),
                    (8, 9),
                    (0, 7),
                    (1, 2),
                    (2, 7)
                ],
                vec![
                    (1, 7, 8),
                    (4, 6, 6),
                    (7, 7, 6),
                    (5, 9, 3),
                    (7, 8, 1),
                    (1, 2, 0),
                    (3, 7, 0),
                    (4, 8, 6),
                    (6, 9, 8)
                ]
            ),
            vec![false, true, false, true, false, false, false, true, false]
        );
    }

    #[test]
    fn is_there_1() {
        assert_eq!(
            NodeSegments::is_there(
                10,
                vec![
                    (4, 7),
                    (0, 8),
                    (9, 9),
                    (3, 6),
                    (6, 9),
                    (5, 6),
                    (5, 5),
                    (1, 3),
                    (7, 7),
                    (3, 8)
                ],
                vec![
                    (0, 4, 6),
                    (8, 9, 8),
                    (9, 9, 1),
                    (2, 9, 4),
                    (8, 9, 1),
                    (4, 6, 5),
                    (2, 8, 7)
                ]
            ),
            vec![false, false, false, true, false, false, false]
        );
    }

    #[test]
    fn is_there_2() {
        assert_eq!(
            NodeSegments::is_there(
                10,
                vec![
                    (8, 9),
                    (1, 5),
                    (9, 9),
                    (6, 6),
                    (8, 9),
                    (6, 6),
                    (8, 9),
                    (2, 4),
                    (3, 7),
                    (0, 0)
                ],
                vec![
                    (4, 5, 6),
                    (5, 5, 2),
                    (5, 5, 4),
                    (8, 9, 8),
                    (7, 8, 8),
                    (0, 3, 7),
                    (7, 9, 5),
                    (6, 7, 1),
                    (8, 8, 7),
                    (7, 9, 6)
                ]
            ),
            vec![false, true, false, false, false, false, false, true, false, false]
        );
    }

    #[test]
    fn is_there_3() {
        assert_eq!(
            NodeSegments::is_there(
                10,
                vec![
                    (4, 6),
                    (6, 7),
                    (6, 9),
                    (2, 6),
                    (9, 9),
                    (2, 6),
                    (7, 8),
                    (1, 7),
                    (3, 4),
                    (9, 9)
                ],
                vec![(2, 4, 2), (3, 6, 8), (2, 2, 3), (3, 3, 6), (5, 8, 9)]
            ),
            vec![false, false, true, false, false]
        );
    }

    #[test]
    fn is_there_4() {
        assert_eq!(
            NodeSegments::is_there(
                10,
                vec![
                    (3, 8),
                    (4, 8),
                    (2, 4),
                    (6, 6),
                    (2, 3),
                    (6, 7),
                    (0, 7),
                    (1, 4),
                    (3, 9),
                    (7, 8)
                ],
                vec![(1, 5, 5), (4, 4, 8), (6, 9, 6), (6, 8, 1), (5, 5, 4)]
            ),
            vec![false, false, true, false, true]
        );
    }

    #[test]
    fn is_there_5() {
        assert_eq!(
            NodeSegments::is_there(
                10,
                vec![
                    (8, 9),
                    (3, 9),
                    (4, 9),
                    (1, 3),
                    (0, 7),
                    (8, 9),
                    (7, 8),
                    (5, 8),
                    (5, 5),
                    (4, 7)
                ],
                vec![(7, 8, 8), (8, 9, 1), (1, 6, 3), (1, 7, 0)]
            ),
            vec![false, false, true, false]
        );
    }

    #[test]
    fn is_there_6() {
        assert_eq!(
            NodeSegments::is_there(
                10,
                vec![
                    (4, 6),
                    (9, 9),
                    (2, 8),
                    (5, 7),
                    (4, 8),
                    (3, 6),
                    (0, 6),
                    (7, 8),
                    (5, 9),
                    (9, 9)
                ],
                vec![
                    (7, 8, 2),
                    (5, 7, 0),
                    (0, 7, 1),
                    (3, 7, 2),
                    (7, 7, 4),
                    (8, 8, 2)
                ]
            ),
            vec![false, false, true, false, false, false]
        );
    }

    #[test]
    fn is_there_7() {
        assert_eq!(
            NodeSegments::is_there(
                10,
                vec![
                    (0, 9),
                    (1, 7),
                    (6, 8),
                    (9, 9),
                    (6, 9),
                    (5, 6),
                    (8, 8),
                    (2, 2),
                    (2, 8),
                    (4, 6)
                ],
                vec![
                    (0, 7, 5),
                    (8, 8, 2),
                    (9, 9, 3),
                    (3, 4, 2),
                    (4, 7, 1),
                    (3, 4, 1),
                    (3, 9, 1),
                    (7, 7, 5),
                    (3, 9, 9)
                ]
            ),
            vec![true, false, true, false, false, false, false, true, false]
        );
    }
}
