struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut ss = s.chars();
        let mut tt = t.chars();
        let mut ss_c: u64 = 1;
        let mut tt_c: u64 = 1;
        let mut temp_s: char;
        let mut temp_t: char;
        let mut s_num: i8;
        let mut t_num: i8;
        let mut prime_s: i32;
        let mut prime_t: i32;
        for i in 0..s.len() {
            temp_s = ss.next().unwrap();
            temp_t = tt.next().unwrap();
            if temp_s == temp_t {
                continue;
            }
            s_num = (temp_t as i8 - 97);
            t_num = (temp_s as i8 - 97);
            prime_s = Solution::get_n_prime(s_num as i32);
            prime_t = Solution::get_n_prime(t_num as i32);

            if (tt_c % (prime_s as u64)) == 0 && (ss_c % (prime_t as u64)) == 0 {
                ss_c /= (prime_t as u64);
                tt_c /= (prime_s as u64);
            } else {
                ss_c *= prime_s as u64;
                tt_c *= prime_t as u64;
            }
        }
        if ss_c == tt_c {
            true
        } else {
            false
        }
    }

    pub fn get_n_prime(n: i32) -> i32 {
        let primes = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
            31, 37, 41, 43, 47, 53, 59, 61, 67,
            71, 73, 79, 81, 83, 89, 97
        ];
        primes[n as usize]
    }
}

fn main() {
    println!("{:?}", Solution::is_anagram(String::from("anagram"), String::from("nagaram")));
}
