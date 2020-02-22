struct Solution;

impl Solution {
    fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut r: Vec<i32> = vec![0;nums.len()];
        let mut t: Vec<i32> = vec![];
        let mut left = 0;
        let mut right = 0;
        let mut position = 0;
        for i in (0..nums.len()).rev() {
            if t.len() == 0 {
                t.push(nums[i]);
                r[i] = 0;
            } else {
                left = 0;
                right = t.len();
                while left < right {
                    position = (left + right) / 2;
                    if t[position] >= nums[i] {
                        right = position;
                    } else {
                        if left != position {
                            left = position;
                        } else {
                            left += 1;
                        }
                    }
                }
                r[i] = left as i32;
                t.insert(left, nums[i]);
                //t.push(nums[i]);
                //t.sort();
            }
        }
        r
    }
}

fn main() {
    println!("{:?}", Solution::count_smaller(vec![5,2,6,1]));
}
