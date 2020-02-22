struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        if n <= 1 {
            return s;
        }
        let mut matrix: Vec<Vec<bool>> = vec![vec![false;n];n];
        let mut max = 1;
        let mut r = String::from(s.get(0..1).unwrap().to_string());
        for step in 0..n {
            //间距从0开始到n-2
            for i in 0..n {
                if (i + step) >= n {
                    break;
                }
                if step == 0 {
                    matrix[i][i+step] = true;
                } else if step == 1 {
                    if s.get(i..i+1) == s.get(i+step..i+step+1) {
                        matrix[i][i+step] = true;
                    } else {
                        matrix[i][i+step] = false;
                    }
                } else {
                    if s.get(i..i+1) == s.get(i+step..i+step+1) && matrix[i+1][i+step-1] == true {
                        matrix[i][i+step] = true;
                    } else {
                        matrix[i][i+step] = false;
                    }
                }
                if matrix[i][i+step] == true {
                    if (step+1) > max {
                        max = step+1;
                        r = s.get(i..i+step+1).unwrap().to_string();
                    }
                }
            }
        }
        r
    }
}

fn main() {
    println!("{:?}", Solution::longest_palindrome(String::from("hello")));
    println!("{:?}", Solution::longest_palindrome(String::from("aaaababa")));
}
