struct Solution;

impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut max = 0;
        if nums.len() < 2 {
            return 0;
        }
        for i in 0..(nums.len() - 1) {
            if (nums[i+1] - nums[i]) > max {
                max = nums[i+1] - nums[i];
            }
        }
        max
    }
}

fn main() {
    println!("{:?}", Solution::maximum_gap(vec![3,6,9,1]));
}
