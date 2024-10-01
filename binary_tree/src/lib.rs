pub struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> TreeNode {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

pub fn count_nodes(node: &Option<Box<TreeNode>>, current_path_sum: i32) -> (i32, i32) {
    if let Some(n) = node {
        let left_result = count_nodes(&n.left, current_path_sum + n.value);
        let right_result = count_nodes(&n.right, current_path_sum + n.value);

        let mut count = left_result.0 + right_result.0;
        if left_result.1 + right_result.1 == current_path_sum {
            count += 1;
        }

        let sum = left_result.1 + right_result.1 + n.value;

        return (count, sum);
    }

    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut root = TreeNode::new(12);
        
        let mut left_child = TreeNode::new(8);
        left_child.left = Some(Box::new(TreeNode::new(3)));
        left_child.right = Some(Box::new(TreeNode::new(9)));

        root.left = Some(Box::new(left_child));
        root.right = Some(Box::new(TreeNode::new(20)));
        
        assert_eq!(count_nodes(&Some(Box::new(root)), 0), (1, 52));
    }
}
