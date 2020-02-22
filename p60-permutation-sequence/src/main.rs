struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut curr = n;
        let mut index: Vec<i32> = vec![];
        let mut remain = k;
        let mut using: Vec<i32> = vec![];

        loop {
            if curr == 0 {
                break;
            }
            // curr = 3,  remain = 2
            // group_count = 2
            let group_count = Solution::xx(curr-1);
            if remain <= 1 || curr == 1 {
                index.push(1);
            } else {
                if remain%group_count == 0 {
                    let group_index = remain/group_count;
                    index.push(group_index);
                    remain = remain - (group_index - 1) * group_count;
                } else {
                    let group_index = (remain - remain%group_count)/group_count + 1;
                    index.push(group_index);
                    remain = remain - (group_index - 1) * group_count;
                }
            }
            curr-=1;
        }
        let mut st = vec![];
        for i in 0..n {
            st.push(i+1);
        }
        for i in index {
            let mut count = 0;
            for j in 0..n {
                if !using.contains(&st[j as usize]) {
                    count += 1;
                }
                if count == i {
                    using.push(st[j as usize]);
                    break;
                }
            }
        }
        let mut r = String::new();
        for i in using {
            r = r + i.to_string().as_str();
        }
        r
    }
    pub fn xx(mut n: i32) -> i32 {
        let mut s = 1;
        for i in 1..(n+1) {
            s *= i;
        }
        s
    }
}

fn main() {
    println!("{:?}", Solution::get_permutation(5, 8));
}
