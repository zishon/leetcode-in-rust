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

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut read = head.clone();
        let mut write = head;
        let mut step = 0;
        let mut r = Box::new(ListNode::new(0));
        let mut rlh = &mut r;

        while read.is_some() {
            read = read.unwrap().next;
            if step < n {
                step += 1;
            } else {
                rlh.next = Some(Box::new(ListNode::new(write.clone().unwrap().val)));
                rlh = rlh.next.as_mut().unwrap();
                write = write.unwrap().next;
            }
        }
        rlh.next = write.unwrap().next;
        r.next
    }

    //init ListNode for test
    pub fn init_list_node(mut l: Vec<i32>) -> Option<Box<ListNode>> {
        let mut r = Box::new(ListNode::new(0));
        let mut rh = &mut r;
        for i in l {
            rh.next = Some(Box::new(ListNode::new(i)));
            rh = rh.next.as_mut().unwrap();
        }
        (*r).next
    }
}

fn main() {
    println!("{:?}", Solution::remove_nth_from_end(Solution::init_list_node(vec![1,2,3,4,5,6,7,8]), 3));
}
