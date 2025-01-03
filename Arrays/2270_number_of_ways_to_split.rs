
// optimal solution TC 0(N) and SC 0(N)


impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let total_sum: i64 = nums.iter().map(|&x| x as i64).sum();
        let mut prefix_sum: i64 = 0;
        let mut count: i32 = 0;


        for i in 0..n-1 {
            prefix_sum += nums[i] as i64;
            if prefix_sum >= total_sum - prefix_sum {
                count += 1;
            }
        }

        count
    }

}
