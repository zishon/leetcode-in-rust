struct Solution;
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut s= String::from("1");
        for i in 1..n {
            //  111221
            let mut count: i32 = 0;
            let mut number = '0';
            let mut temp = String::new();
            for c in s.chars() {
                if c != number && count > 0 {
                    temp.extend(count.to_string().chars());
                    temp.push(number);
                    count = 0;
                }
                if count == 0 {
                    count += 1;
                    number = c;
                } else {
                    count += 1;
                }
            }
            if count > 0 {
                temp.extend(count.to_string().chars());
                temp.push(number);
            }
            s = temp;
        }
        s
    }
}

fn main() {
    println!("{:?}", Solution::count_and_say(5));
}
