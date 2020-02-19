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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut r = Box::new(ListNode::new(0));
        let mut rh = &mut r;
        let mut additional = 0;
        loop {
            let mut sum = additional;
            if l1 == None && l2 == None && sum == 0 {
                break;
            }
            if let Some(x) = l1 {
                sum += x.val;
                l1 = x.next;
            }
            if let Some(y) = l2 {
                sum += y.val;
                l2 = y.next;
            }
            if sum > 9 {
                additional = 1;
                sum -= 10;
            } else {
                additional = 0;
            }
            rh.next = Some(Box::new(ListNode::new(sum)));
            rh = rh.next.as_mut().unwrap();
        }
        (*r).next
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
    let l1 = Solution::init_list_node(vec![9]);
    let l2 = Solution::init_list_node(vec![1,9,9,9,9,9,9,9,9,9]);
    println!("{:?}", Solution::add_two_numbers(l1, l2));
}
