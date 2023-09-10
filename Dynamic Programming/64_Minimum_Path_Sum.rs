use std::collections::HashMap;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        i
        let max_value = 1e5 as i32;
        let row = grid.len();
        let col = grid[0].len();

        fn find_min_path(
            i: usize,
            j: usize,
            row: usize,
            col: usize,
            grid: &Vec<Vec<i32>>,
            memo: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            if let Some(&value) = memo.get(&(i, j)) {
                return value;
            }

            if i == row - 1 && j == col - 1 {
                memo.insert((i, j), grid[i][j]);
                return grid[i][j];
            }

            if i >= row || j >= col {
                memo.insert((i, j), 1e5 as i32);
                return 1e5 as i32;
            }

            let right = grid[i][j] + find_min_path(i, j + 1, row, col, grid, memo);
            let down = grid[i][j] + find_min_path(i + 1, j, row, col, grid, memo);

            let min_path = right.min(down);
            memo.insert((i, j), min_path);
            min_path
        }

        let mut memo: HashMap<(usize, usize), i32> = HashMap::new();
        find_min_path(0, 0, row, col, &grid, &mut memo)
    }
}