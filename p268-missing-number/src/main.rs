struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = (n*(n+1)/2) as i32;
        for i in nums {
            sum -= i;
        }
        sum
    }
}

fn main() {
    println!("{:?}", Solution::missing_number(vec![3,0,1]));
}
