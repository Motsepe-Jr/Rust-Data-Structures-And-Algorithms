use std::collections::HashMap;

// BRUTE FORCE SOLUTION 0(N) + SPACE 0(N)
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {

        let mut hash_map: HashMap<i32, i32>  = HashMap::new();

        for &num in &nums {
            let count = hash_map.entry(num).or_insert(0);
            *count += 1;
        }

        for (&key, &value) in &hash_map {
            if value == 1 {
                return key;
            }
        }

        -1

    }
}

// OPTINUM SOLUTION 0(N) AND 0(1) SPACE
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {

        let mut xor = 0;

        for &num in &nums{
            xor ^= num 
        }

        xor
    }
}