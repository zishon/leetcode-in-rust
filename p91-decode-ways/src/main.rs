struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let ss = s.as_str();
        let len = s.len();
        if len == 0 {
            return 0;
        } else if len == 1 {
            return if ss[0..1].parse::<i32>().unwrap() == 0 {
                0
            } else {
                1
            }
        } else {
            let mut count = 1;
            let mut start: usize = 0;
            for mut i in 0..len {
                if i != (len-1) {
                    let first_letter = ss[i..i+1].parse::<i32>().unwrap();
                    let second_letter = ss[i+1..i+2].parse::<i32>().unwrap();
                    if first_letter == 0 && second_letter == 0 {
                        return 0;
                    } else if first_letter == 0 || (first_letter * 10 + second_letter) > 26 {
                        let number_str = &ss[start..i+1];
                        if number_str.parse::<i32>().unwrap() == 0 {
                            return 0;
                        } else {
                            start = i+1;
                            i += 1;
                            count *= Solution::_num_decodings(number_str);;
                        }
                    }
                } else {
                    count *= Solution::_num_decodings(&ss[start..]);
                }
            }
            count
        }
    }

    pub fn _num_decodings(s: &str) -> i32 {
        if s.len() == 1 {
            if s.parse::<i32>().unwrap() == 0 {
                0
            } else {
                1
            }
        } else {
            let first = &s[0..1];
            let second = &s[1..2];
            if s.len() > 2 {
                if second.parse::<i32>().unwrap() == 0 {
                    Solution::_num_decodings(&s[2..])
                } else {
                    Solution::_num_decodings(&s[1..]) + Solution::_num_decodings(&s[2..])
                }
            } else {
                if second.parse::<i32>().unwrap() == 0 {
                    1
                } else {
                    2
                }
            }
        }
    }
}
fn main() {
    println!("{:?}", Solution::num_decodings(String::from("123412415123412453281234")));
}
