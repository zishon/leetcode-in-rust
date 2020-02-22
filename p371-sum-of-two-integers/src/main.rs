struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    println!("{:?}", Solution::get_sum(2,3));
}
