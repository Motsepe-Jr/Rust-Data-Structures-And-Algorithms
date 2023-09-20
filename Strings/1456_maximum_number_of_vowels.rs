use std::collections::HashSet;

fn max_vowels(s: &str, k:i32) -> i32 {

    let vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].iter().cloned().collect();
    let s_chars: Vec<char> = s.chars().collect();
    let mut max_vowels = 0;
    let mut current_vowels = 0;

    for i in 0..k as usize {
        if vowels.contains(&s_chars[i]) {
            current_vowels += 1
        }
    }

    max_vowels = current_vowels;

    for i in k as usize..s.len() {
        if vowels.contains(&s_chars[i - k as usize]) {
            current_vowels -= 1;
        }
        if vowels.contains(&s_chars[i as usize]) {
            current_vowels += 1;
        }

        max_vowels = max_vowels.max(current_vowels);
    }

    max_vowels

}