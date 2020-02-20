use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() <= 1 {
            return s.len() as i32;
        }
        let mut pre_index = vec![];
        let mut next_index = vec![];
        let mut last_index: HashMap<char, i32> = HashMap::new();
        let mut count = 0;

        let mut chars = s.chars();
        while let Some(x) = chars.next() {
            next_index.push(-1);
            if let Some(&index) = last_index.get(&x) {
                next_index[index as usize] = count as i32;
                pre_index.push(index);
            } else {
                pre_index.push(-1);
            }
            last_index.insert(x, count);
            count += 1;
        }
        let mut max = 0;
        let mut i = 0;
        let mut j = 0;
        while i < s.len() || j < s.len() {
            if j < s.len() && pre_index[j] >= i as i32 && pre_index[j] < j as i32 {
                i = (pre_index[j] + 1) as usize;
            } else if j < s.len() {
                if (j - i + 1) > max {
                    max = j - i + 1;
                }
                j += 1;
            } else {
                break;
            }
        }
        max as i32
    }
}

fn main() {
    println!("{:?}", Solution::length_of_longest_substring(String::from("abdekfaliec")));
}
