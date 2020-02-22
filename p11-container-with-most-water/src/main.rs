struct Solution;
use std::cmp::min;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut left = 0;
        let mut right = n - 1;
        let mut area = 0;
        while left < right {
            let temp_area = min(height[right], height[left]) * (right - left) as i32;
            if temp_area > area {
                area = temp_area;
            }
            if height[left] <= height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        area
    }
}

fn main() {
    println!("{:?}", Solution::max_area(vec![1,8,6,2,5,4,8,3,7]));
}
