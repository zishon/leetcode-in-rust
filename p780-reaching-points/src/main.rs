struct Solution;

impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, mut tx: i32, mut ty: i32) -> bool {
        let mut result: bool;
        loop {
            if sx == tx && sy == ty {
                result = true;
                break;
            }

            if tx == ty || tx < sx || ty < sy{
                result = false;
                break;
            }

            if tx > ty {
                if ((tx - sx) - (tx - sx) % ty) == 0{
                    result = false;
                    break;
                }
                tx = tx - ((tx - sx) - (tx - sx) % ty);
            } else if tx < ty {
                if ((ty - sy) - (ty - sy) % tx) == 0 {
                    result = false;
                    break;
                }
                ty = ty - ((ty - sy) - (ty - sy) % tx);
            } else {
                result = false;
                break;
            }
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::reaching_points(1, 1, 3, 5));
}
