use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut forward_hash: HashMap<char, char> = HashMap::new();
        let mut backward_hash: HashMap<char, char> = HashMap::new();
        let mut s_chars = s.chars();
        let mut t_chars = t.chars();

        while let Some(sc) = s_chars.next() {
            let tc = t_chars.next().unwrap();
            if let Some(&map) = forward_hash.get(&sc) {
                if map != tc {
                    return false;
                }
            } else if let Some(&map) = backward_hash.get(&tc) {
                if map != sc {
                    return false;
                }
            } else {
                forward_hash.insert(sc, tc);
                backward_hash.insert(tc, sc);
            }
        }
        true
    }
}

fn main() {
    println!("{:?}", Solution::is_isomorphic(String::from("acbc"), String::from("defe")));
}
