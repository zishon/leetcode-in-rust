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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        Solution::_binary_tree_paths(root, String::new())
    }

    pub fn _binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>, mut s: String) -> Vec<String> {
        println!("{:?} {:?} \n", root, s);
        match root {
            None => {
                vec![]
            },
            Some(node) => {
                if s.len() != 0 {
                    s.push_str("->");
                }
                s.push_str(node.borrow().val.to_string().as_str());
                let mut r = vec![];
                if node.borrow().left != None {
                    let left_result = Solution::_binary_tree_paths(node.borrow().left.clone(), s.clone());
                    for i in left_result {
                        r.push(i);
                    }
                }
                if node.borrow().right != None {
                    let right_result = Solution::_binary_tree_paths(node.borrow().right.clone(), s.clone());
                    for i in right_result {
                        r.push(i);
                    }
                }
                if node.borrow().left == None && node.borrow().right == None {
                    r.push(s);
                }
                r
            }
        }
    }
}
fn main() {
    let mut root = TreeNode::new(5);
    let mut left = TreeNode::new(6);
    let mut right = TreeNode::new(7);
    root.left = Some(Rc::new(RefCell::new(left)));
    root.right = Some(Rc::new(RefCell::new(right)));
    println!("{:?}", Solution::binary_tree_paths(Some(Rc::new(RefCell::new(root)))));
}
