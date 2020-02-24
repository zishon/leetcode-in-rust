struct Solution;

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let mut min_m = m;
        let mut min_n = n;

        for i in ops {
            if min_m > i[0] {
                min_m = i[0];
            }
            if min_n > i[1] {
                min_n = i[1];
            }
        }

        min_m * min_n
    }
}

fn main() {
    println!("{:?}", Solution::max_count(3, 3, vec![vec![2,2], vec![3,3]]));
}
