use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
        let row = mat.len();
        let col = mat[0].len();

        for r in 0..row {
            for c in 0..col {
                if mat[r][c] == 0 {
                    queue.push_back((r, c, 0));
                    visited.insert((r, c));
                }
            }
        }

        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some((r, c, d)) = queue.pop_front() {
            for (row_offset, col_offset) in &directions {
                let nrow = (r as isize + row_offset) as usize;
                let ncol = (c as isize + col_offset) as usize;

                if nrow < row && ncol < col && !visited.contains(&(nrow, ncol)) && mat[nrow][ncol] == 1 {
                    queue.push_back((nrow, ncol, d + 1));
                    visited.insert((nrow, ncol));
                    mat[nrow][ncol] = d + 1;
                }
            }
        }

        mat
    }
}