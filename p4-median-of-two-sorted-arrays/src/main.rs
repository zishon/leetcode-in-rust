struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(num1: Vec<i32>, num2: Vec<i32>) -> f64 {
        let mut i = 0;
        let mut j = 0;
        let len = num1.len() + num2.len();
        let mut left = 0;
        let mut right = 0;
        if len % 2 == 0 {
            right = len / 2;
            left = right - 1;
        } else {
            right = (len - 1) / 2;
            left = right;
        }
        let mut count = 0;
        let mut sum = 0;
        let mut t = 0;
        while i < num1.len() || j < num2.len() {
            if i < num1.len() && j < num2.len() {
                if num1[i] <= num2[j] {
                    t = num1[i];
                    i += 1;
                } else {
                    t = num2[j];
                    j += 1;
                }
            } else if i >= num1.len() {
                t = num2[j];
                j += 1;
            } else if j >= num2.len() {
                t = num1[i];
                i += 1;
            }
            count += 1;
            if (count - 1) == left {
                sum += t;
            }
            if (count - 1) == right {
                sum += t;
                break;
            }
        }
        sum as f64 / 2.0
    }
}

fn main() {
    println!("{:?}", Solution::find_median_sorted_arrays(vec![1,3], vec![2]));
}
