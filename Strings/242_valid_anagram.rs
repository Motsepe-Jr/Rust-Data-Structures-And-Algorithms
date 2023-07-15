use std::collections::HashMap;

struct Solution {}


 // HASHMAP SOLUTION
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count_s: HashMap<char, i32> = HashMap::new();
        let mut count_t: HashMap<char, i32> = HashMap::new();

        for i in 0..s.len() {
            *count_s.entry(s.chars().nth(i).unwrap()).or_insert(0) += 1;
            *count_t.entry(t.chars().nth(i).unwrap()).or_insert(0) += 1;
        }

        for (key, value) in count_s.iter() {
            if *count_t.get(key).unwrap_or(&0) != *value {
                return false;
            }
        }

        true
    }
}

fn main() {
    let s = String::from("anagram");
    let t = String::from("nagaram");
    println!("{}", Solution::is_anagram(s, t)); // Output: true
}

// O(1) MEMORY -> SORT 
struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let sorted_s: Vec<char> = s.chars().collect();
        let sorted_t: Vec<char> = t.chars().collect();

        sorted_s.sort();
        sorted_t.sort();

        sorted_s == sorted_t
    }
}
