struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            false
        } else if x < 10 {
            true
        } else {
            let mut numbers = vec![];
            let mut a = x;
            while a > 0 {
                numbers.push(a % 10);
                a = (a - a % 10) / 10;
            }
            for i in 0..numbers.len()/2 {
                if numbers[i] != numbers[numbers.len()-1-i] {
                    return false;
                }
            }
            true
        }
    }
}

fn main() {
    println!("{:?}", Solution::is_palindrome(121));
}
