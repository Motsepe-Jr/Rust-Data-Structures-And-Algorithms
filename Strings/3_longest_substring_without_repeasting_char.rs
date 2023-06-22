use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {

        let mut hashmap: HashMap<char, usize> = HashMap::new();
        let mut left = 0;
        let mut longest_substring = 0;

        for (right, c) in s.chars().enumerate() {
            if let Some(&prev_index) = hashmap.get(&c) {
                left = left.max(prev_index + 1);
            }

            hashmap.insert(c, right);

            longest_substring = longest_substring.max(right - left + 1);
        }

        longest_substring as i32
        
    }
}

// while loop

// use std::collections::HashMap;

// pub fn length_of_longest_substring(s: String) -> i32 {
//     let mut hashmap: HashMap<char, usize> = HashMap::new();
//     let mut left = 0;
//     let mut right = 0;
//     let mut longest_substring = 0;
//     let chars: Vec<char> = s.chars().collect();

//     while right < chars.len() {
//         if let Some(&index) = hashmap.get(&chars[right]) {
//             left = left.max(index + 1);
//         }

//         hashmap.insert(chars[right], right);

//         longest_substring = longest_substring.max(right - left + 1);

//         right += 1;
//     }

//     longest_substring as i32
// }