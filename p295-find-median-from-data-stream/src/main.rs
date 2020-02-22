use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
    left_count: i32,
    right_count: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    /** initialize your data structure here. */
    fn new() -> Self {
        MedianFinder {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
            left_count: 0,
            right_count: 0,
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.left_count == 0 {
            self.left.push(num);
            self.left_count += 1;
        } else {
            let left_max = self.left.peek().clone().unwrap();
            if self.left_count > self.right_count {
                if *left_max > num && self.left_count > self.right_count {
                    self.left.push(num);
                    let left_max = self.left.pop().unwrap();
                    self.right.push(Reverse(left_max));
                } else if *left_max <= num && self.left_count > self.right_count {
                    self.right.push(Reverse(num));
                }
                self.right_count += 1;
            } else {
                if *left_max > num && self.left_count <= self.right_count {
                    self.left.push(num);
                } else if *left_max <= num && self.left_count <= self.right_count {
                    self.right.push(Reverse(num));
                    let Reverse(right_min) = self.right.pop().unwrap();
                    self.left.push(right_min);
                }
                self.left_count += 1;
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.left_count > self.right_count {
            let left_max = self.left.peek().clone().unwrap();
            *left_max as f64
        } else {
            let left_max = self.left.peek().clone().unwrap();
            let Reverse(right_min) = self.right.peek().clone().unwrap();
            (*left_max + *right_min) as f64 / 2.0
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

fn main() {
    let mut s = MedianFinder::new();
    s.add_num(1);
    s.add_num(2);
    println!("{:?}", s.find_median());
    s.add_num(3);
    println!("{:?}", s.find_median());
}
