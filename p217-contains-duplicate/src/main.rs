struct Solution;

impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        if nums.len() == 0 {
            false
        } else {
            nums.sort();
            let nums_len = nums.len();
            let mut result = false;
            for i in 0 .. nums_len-1 {
                if nums[i] == nums[i+1] {
                    result = true;
                    break;
                }
            }
            result
        }
    }
}

fn main() {
    println!("{:?}", Solution::contains_duplicate(vec![1,2,3,4,5,5]));
}
