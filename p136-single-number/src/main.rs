struct Solution;

impl Solution {
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        let mut number = 0;
        for i in nums {
            number ^= i;
        }
        number
    }
}

fn main() {
    println!("{:?}", Solution::single_number(vec![4,1,2,1,2]));
}
