struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let first_char = letters[0];
        for i in letters {
            if i > target {
                return i;
            }
        }
        return first_char;
    }
}

fn main() {
    println!("{:?}", Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a'));
}
