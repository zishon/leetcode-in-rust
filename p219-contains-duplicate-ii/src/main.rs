struct Solution;

impl Solution {
    fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() > 0 {
            let nums_len = nums.len();
            let mut result = false;
            let _k = k as usize;
            for i in 0..nums_len - 1 {
                for j in i + 1..nums_len {
                    if (j-i) > _k {
                        break;
                    }
                    if nums[i] == nums[j] && (j-i) <= _k {
                        result = true;
                        break;
                    }
                }
                if result == true {
                    break;
                }
            }
            result
        } else {
            false
        }
    }
}

fn main() {
    println!("{:?}", Solution::contains_nearby_duplicate(vec![1,2,3,1], 3));
}
