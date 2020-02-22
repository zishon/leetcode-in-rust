struct Solution;

impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        let mut result = false;
        loop {
            if n <= 0 {
                break;
            } else if n == 1 {
                result = true;
                break;
            } else if (n % 2) != 0 {
                result = false;
                break;
            } else {
                n = n / 2;
            }
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::is_power_of_two(4));
    println!("{:?}", Solution::is_power_of_two(5));
}
