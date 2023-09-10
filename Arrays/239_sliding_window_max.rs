use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {

        let mut q: VecDeque<usize> = VecDeque::new();
        let mut res: Vec<i32> = Vec::new();
        let (mut l, mut r) = (0, 0);

        let nums_len = nums.len();

        while r < nums_len {
            while !q.is_empty() && nums[*q.back().unwrap()] < nums[r] {
                q.pop_back();
            }
            q.push_back(r);

            if l > q[0] {
                q.pop_front();
            }

            if (r + 1) >= k as usize {
                res.push(nums[q[0]]);
                l += 1;
            }

            r += 1;
        }
        res
    }
}