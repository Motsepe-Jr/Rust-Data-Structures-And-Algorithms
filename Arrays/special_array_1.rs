impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {

        // a single or empty array is is true

        // odd number mod by 2 the remainder is 1, and even number mod by 2 the remainder is 0

        if nums.len() < 2 {
            return true;
        }


        for i in 0..nums.len() - 1 {
            if nums[i] % 2 == nums[i + 1] % 2 {
                return false;
            }
        }

        true

        
    }
}