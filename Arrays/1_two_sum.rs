use std::collections::HashMap;

// HashMap Approach

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        // hashmap solution Time: 0(N) and Space 0(N)

        let mut map: HashMap<i32, i32> = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&complement_index) = map.get(&complement) {
                return vec![complement_index, index as i32];
            }

            map.insert(*num, index as i32);
        }
        vec![]
    }
}


// Two Pointer Approach --> N log N

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let (mut left, mut right) = (0, nums.len() - 1);

        while left < right {
            let sum = sorted_nums[left] + sorted_nums[right];

            if sum == target {
                let left_num = sorted_nums[left];
                let right_num = sorted_nums[right];

                let left_index = nums.iter().position(|&x| x == left_num).unwrap() as i32;
                let right_index = nums.iter().rposition(|&x| x == right_num).unwrap() as i32;

                return vec![left_index, right_index];
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }

        vec![]
    }
}

