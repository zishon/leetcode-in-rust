//Definition for a binary tree node.
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
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut r = vec![];
        match root {
            None => {},
            Some(node) => {
                let left_child = (*node).borrow().left.clone();
                let right_child = (*node).borrow().right.clone();
                r.extend(Solution::inorder_traversal(left_child));
                r.extend(&[(*node).borrow().val]);
                r.extend(Solution::inorder_traversal(right_child));
            },
        }
        r
    }
}

fn main() {
    let left = TreeNode::new(2);
    let right = TreeNode::new(3);
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(left))),
        right: Some(Rc::new(RefCell::new(right))),
    })));

    println!("{:?}", Solution::inorder_traversal(root));
}
