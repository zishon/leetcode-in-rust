struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }
        let mut temp:Vec<u8> = vec![1;n as usize];
        temp[0] = 0;
        temp[1] = 0;

        let square_root = (n as f64).sqrt() as i32 + 1;
        for i in 2..square_root {
            if temp[i as usize] == 1 {
                for j in (2*i..n).step_by(i as usize) {
                    temp[j as usize] = 0;
                }
            }
        }
        let mut count = 0;
        for i in temp.iter() {
            if i == &1u8 {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    println!("{:?}", Solution::count_primes(15));
}
