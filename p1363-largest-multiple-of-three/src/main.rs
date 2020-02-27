struct Solution;

impl Solution {
    pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
        let mut count = vec![0;10];
        let sum = digits.iter().sum::<i32>();
        let mut rest = sum % 3;

        for i in digits {
            count[i as usize] += 1;
        }

        //删除一次的做法
        if rest > 0 {
            for k in 0..9 {
                if count[k as usize] >= 1 && k % 3 == rest {
                    count[k as usize] -= 1;
                    rest = 0;
                    break;
                }
            }
        }
        //删除两次的做法
        if rest > 0 {
            let mode = (rest * 2) % 3;
            if rest == 1 {
                rest += 3;
            }
            for k in 0..9 {
                if count[k as usize] >= 1 && k % 3 == mode {
                    if count[k as usize] >= 2 {
                        count[k as usize] -= 2;
                        rest = 0;
                    } else {
                        count[k as usize] -= 1;
                        rest -= mode;
                    }
                    if rest == 0 {
                        break;
                    }
                }
            }
        }

        if (count.iter().sum::<i32>() - count[0]) == 0 && count[0] > 0 {
            return String::from("0");
        } else if count.iter().sum::<i32>() == 0 {
            return String::from("");
        }

        //从高位拼
        let mut r = String::new();
        for i in (0..10).rev() {
            for _ in 0..count[i] {
                r.push(('0' as u8 + i as u8) as char);
            }
        }
        r
    }
}

fn main() {
    println!("{:?}", Solution::largest_multiple_of_three(vec![8, 7, 6, 1, 0]));
}
