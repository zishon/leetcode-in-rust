struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        } else if n == 2 {
            return 2;
        }
        let mut sn_1 = 1;
        let mut sn_2 = 2;
        let mut sn = 3;

        for i in 3..(n+1) {
            sn = sn_2 + sn_1;
            sn_1 = sn_2;
            sn_2 = sn;
        }
        sn
    }
}

fn main() {
    println!("{:?}", Solution::climb_stairs(5));
}
