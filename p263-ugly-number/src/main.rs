struct Solution;

impl Solution {
    pub fn is_ugly(mut num: i32) -> bool {
        if num < 1 {
            false
        } else {
            loop {
                if num % 2 == 0 {
                    num /= 2;
                }
                if num % 3 == 0 {
                    num /= 3;
                }
                if num % 5 == 0 {
                    num /= 5;
                }

                if num % 2 != 0 && num % 3 != 0 && num % 5 != 0 {
                    break;
                }
            }
            if num == 1 {
                true
            } else {
                false
            }
        }
    }
}

fn main() {
    println!("{:?}", Solution::is_ugly(30));
}
