struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.len() == 0 {
            return vec![];
        }
        let mut column_min = 0;
        let mut column_max = matrix[0].len() as i32 - 1;
        let mut row_min = 0;
        let mut row_max = matrix.len() as i32 - 1;
        let mut curr_row = 0;
        let mut curr_column = 0;
        let mut r: Vec<i32> = vec![];
        let mut direction = 1;
        loop {
            r.push(matrix[curr_row][curr_column]);
            if r.len() == (matrix.len() * matrix[0].len()) {
                break;
            }
            if direction == 1 {
                //向右走
                if (curr_column as i32) < column_max {
                    curr_column += 1;
                } else {
                    direction = 2;
                    curr_row += 1;
                    row_min += 1;
                }
            } else if direction == 2 {
                //下
                if (curr_row as i32) < row_max {
                    curr_row += 1;
                } else {
                    direction = 3;
                    curr_column -= 1;
                    column_max -= 1;
                }
            } else if direction == 3 {
                //左
                if (curr_column as i32) > column_min {
                    curr_column -= 1;
                } else {
                    direction = 4;
                    curr_row -= 1;
                    row_max -= 1;
                }
            } else if direction == 4 {
                //上
                if (curr_row as i32) > row_min {
                    curr_row -= 1;
                } else {
                    direction = 1;
                    curr_column += 1;
                    column_min += 1;
                }
            }
        }
        r
    }
}

fn main() {
    println!("{:?}", Solution::spiral_order(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]));
}
