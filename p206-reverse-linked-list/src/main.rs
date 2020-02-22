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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head;
        let mut prev = None;

        while let Some(mut inner) = curr {
            curr = inner.next.take();
            inner.next = prev;
            prev = Some(inner);
        }
        prev
    }
}

fn main() {
    let node1 = Some(Box::new(ListNode::new(2)));
    let node2 = Some(Box::new(ListNode {
        val: 1,
        next: node1,
    }));
    println!("{:?}", Solution::reverse_list(node2));
}
