struct Solution;

impl Solution {
    fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut r = false;
        let mut last_index: HashMap<i32, i32> = HashMap::new();
        for (pos, &num) in nums.iter().enumerate() {
            if let Some(&last_pos) = last_index.get(&num) {
                if (pos as i32 - last_pos) <= k {
                    return true;
                }
            }
            last_index.insert(num, pos as i32);
        }
        r
    }
}

fn main() {
    println!("{:?}", Solution::contains_nearby_duplicate(vec![1,2,3,1], 3));
}
