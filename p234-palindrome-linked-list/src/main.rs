struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut curr = head;
        let mut value: Vec<i32> = vec![];
        while let Some(mut inner) = curr {
            curr = inner.next.take();
            value.push(inner.val);
        }

        let mut reverse_val = value.clone();
        reverse_val.reverse();
        value == reverse_val
    }
}

fn main() {
    let node1 = Some(Box::new(ListNode::new(2)));
    let node2 = Some(Box::new(ListNode {
        val: 1,
        next: node1,
    }));
    println!("{:?}", Solution::is_palindrome(node2));
}
