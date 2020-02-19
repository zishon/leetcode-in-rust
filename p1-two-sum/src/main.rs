struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash = HashMap::new();
        for i in 0..nums.len() {
            hash.insert(nums[i], i);
        }
        for i in 0..nums.len() {
            let temp = target - nums[i];
            if let Some(&x) = hash.get(&temp) {
                if x == i {
                    continue;
                }
                return vec![i as i32, x as i32];
            }
        }
        vec![]
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![3,2,4], 6));
}
