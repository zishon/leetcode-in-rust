struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let n = digits.len();
        digits[n-1] += 1;

        for i in (1..n).rev() {
            if digits[i] >= 10 {
                digits[i] -= 10;
                digits[i-1] += 1;
            } else {
                break;
            }
        }
        if digits[0] >= 10 {
            digits[0] -= 10;
            digits.insert(0, 1);
        }
        digits
    }
}

fn main() {
    println!("{:?}", Solution::plus_one(vec![1,2,5,6,7,9,9, 9]));
}
