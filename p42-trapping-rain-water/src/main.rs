struct Solution;
impl Solution {
    pub fn trap(mut height: Vec<i32>) -> i32 {
        let n = height.len();
        if n <= 2 {
            return 0;
        }
        let mut left = 0;
        let mut right = n - 1;
        let mut sum = 0;
        loop {
            while (left+1) < n && height[left] <= height[left+1] {
                left += 1;
            }
            // left = 2, right = 2
            while right > left && (right-1) >= 0 && height[right] <= height[right-1] {
                right -= 1;
            }
            if left >= right {
                break;
            }
            if height[left] <= height[right] {
                //补齐left右边那个
                if (left+1) < right {
                    sum = sum + height[left] - height[left+1];
                    height[left+1] = height[left];
                } else {
                    break;
                }
            } else {
                //补齐right左边那个
                if (right-1) > left {
                    sum = sum + height[right] - height[right-1];
                    height[right-1] = height[right];
                } else {
                    break;
                }
            }
        }
        sum
    }
}

fn main() {
    println!("{:?}", Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]));
}
