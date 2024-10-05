use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

#[derive(Debug)]
struct BinarySearchTree {
    root: Option<Rc<RefCell<TreeNode>>>
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree {
            root: None
        }
    }

    fn insert(&mut self, val: i32) {
        let new_node = Rc::new(RefCell::new(TreeNode::new(val)));

        if let Some(root) = &self.root {
            Self::insert_recursive(root, new_node);
        } else {
            self.root = Some(new_node);
        }
    }

    fn insert_recursive(node: &Rc<RefCell<TreeNode>>, new_node: Rc<RefCell<TreeNode>>) {
        let mut node = node.borrow_mut();
        if new_node.borrow().val < node.val {
            if let Some(left) = &node.left {
                Self::insert_recursive(left, new_node);
            } else {
                node.left = Some(new_node);
            }
        } else {
            if let Some(right) = &node.right {
                Self::insert_recursive(right, new_node);
            } else {
                node.right = Some(new_node);
            }
        }
    }

    fn in_order_traversal(&self) -> Vec<i32> {
        let mut result = Vec::new();
        if let Some(root) = &self.root {
            Self::in_order_recursive(root, &mut result);
        }
        result
    }

    fn in_order_recursive(node: &Rc<RefCell<TreeNode>>, result: &mut Vec<i32>) {
        let node = node.borrow();
        if let Some(left) = &node.left {
            Self::in_order_recursive(left, result);
        }
        result.push(node.val);
        if let Some(right) = &node.right {
            Self::in_order_recursive(right, result);
        }
    }
}




pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>{
    let data: Vec<Vec<i32>> = Vec::new();
    return data
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create tree nodes
    fn node(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn test_example_tree() {
        let root = node(3,
                        node(9, None, None),
                        node(20,
                             node(15, None, None),
                             node(7, None, None)));

        assert_eq!(level_order(root), vec![vec![3], vec![9, 20], vec![15, 7]]);
    }

    #[test]
    fn test_empty_tree() {
        assert_eq!(level_order(None), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_single_node_tree() {
        let root = node(1, None, None);
        assert_eq!(level_order(root), vec![vec![1]]);
    }

    #[test]
    fn test_bst_insert_and_traverse() {
        let mut bst = BinarySearchTree::new();

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(1);
        bst.insert(9);

        let traversal = bst.in_order_traversal();
        assert_eq!(traversal, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_bst_empty() {
        let bst = BinarySearchTree::new();
        assert_eq!(bst.in_order_traversal(), vec![]);
    }

    #[test]
    fn test_bst_single_node() {
        let mut bst = BinarySearchTree::new();
        bst.insert(1);
        assert_eq!(bst.in_order_traversal(), vec![1]);
    }
}
