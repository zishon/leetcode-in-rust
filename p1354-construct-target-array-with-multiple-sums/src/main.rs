use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::{Reverse, max};

struct Solution;

impl Solution {
    pub fn is_possible(mut target: Vec<i32>) -> bool {
        let mut s = BinaryHeap::new();
        let mut pos = HashMap::new();
        let mut sum = 0;
        let mut other = 0;
        for (k, &v) in target.iter().enumerate() {
            sum += v;
            if v != 1 {
                s.push(v);
                pos.insert(v, k);
            }
        }
        while let Some(mut num) = s.pop() {
            if num <= 1 {
                break;
            }
            if let Some(&position) = pos.get(&num) {
                other = sum - num;
                if other <= 0 {
                    break;
                }
                let times = max(num/other, 2);
                num = sum - times * other;
                sum = num + other;
                if num < 1 {
                    break;
                } else if num == 1 {
                    target[position] = 1;
                } else {
                    pos.insert(num, position);
                    s.push(num);
                }
            }
        }
        let mut r = true;
        for i in target {
            if i != 1 {
                r = false;
                break;
            }
        }
        r
    }
}

fn main() {
    println!("{:?}", Solution::is_possible(vec![1,1000000]));
}
