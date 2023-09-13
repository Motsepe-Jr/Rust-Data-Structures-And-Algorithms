impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        const MAX_NUM: usize = 102;
        let mut buckets = vec![0; MAX_NUM];

        for &num in &nums {
            buckets[num as usize] += 1;
        }

        for i in 1..MAX_NUM {
            buckets[i] += buckets[i - 1];
        }

        let mut result = vec![0; nums.len()];

        for i in 0..nums.len() {
            if nums[i] == 0 {
                result[i] = 0;
            } else {
                result[i] = buckets[(nums[i] - 1) as usize];
            }
        }

        result
    }
}