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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => {
                None
            },
            Some(node) => {
                let mut left_child = node.borrow_mut().left.take();
                let mut right_child = node.borrow_mut().right.take();
                if left_child.is_some() {
                    left_child = Solution::invert_tree(left_child);
                }
                if right_child.is_some() {
                    right_child = Solution::invert_tree(right_child);
                }
                node.borrow_mut().left = right_child;
                node.borrow_mut().right = left_child;
                Some(node)
            },
        }
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

    println!("{:?}", Solution::invert_tree(root));
}
