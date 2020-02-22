struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut prev = 0;
        let mut current = 1;
        let mut sum = 0;
        if n == 0 {
            return prev;
        } else if n == 1 {
            return current;
        } else {
            for i in 2..(n+1) {
                sum = prev + current;
                prev = current;
                current = sum;
            }
        }
        sum
    }
}

fn main() {
    println!("{:?}", Solution::fib(5));
}
