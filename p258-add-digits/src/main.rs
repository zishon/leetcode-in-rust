struct Solution;

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        if num == 0 {
            0
        } else if num % 9 == 0 {
            9
        } else {
            num % 9
        }
    }
}

fn main() {
    println!("{:?}", Solution::add_digits(38));
}
