
// the problem statement is to count number of ones and zeros after you split the string
// from the left substring count zeros, from the right substring count ones

// input and output
// given a string, which is made up of zeros and ones, return the max count of num of zero from left substring + num of ones from right substring

// edge cases
// Can I count ones from the left substring or count zeros from the right substring?


// brute force ~ N^2

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s = s.as_bytes(); // Convert to bytes for easier char comparison
        let mut max_score = 0;
        
        
        for i in 0..s.len()-1 {
            let left_zeros = s[..=i]
                .iter()
                .filter(|&&c| c == b'0')
                .count();
                
            let right_ones = s[i+1..]
                .iter()
                .filter(|&&c| c == b'1')
                .count();
            
            max_score = max_score.max((left_zeros + right_ones) as i32);
        }
        
        max_score
    }
}

// better solution 
// count all the ones, and subtract them if they occur from the left substring
// add the substrated total ones, with accumulated zeros from the left subrting
// 2N ~ N

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s = s.as_bytes();
        let mut max_score = 0;
        

        let total_ones = s.iter().filter(|&&c| c == b'1').count();
        let mut left_zeros = 0;
        let mut left_ones = 0;
        
        // Try each split point
        for i in 0..s.len()-1 {
            if s[i] == b'0' {
                left_zeros += 1;
            } else {
                left_ones += 1;
            }
            
            let right_ones = total_ones - left_ones;
            max_score = max_score.max((left_zeros + right_ones) as i32);
        }
        
        max_score
    }
}

// optimal solution
// we want more zero in the left and more ones in the right to max the score, 
// count the difference between the zeros and ones
// the goal is to keep track of diff between the zeros and ones, the diff tell the max number of zeros on the left 
// N

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s = s.as_bytes();
        let mut zeros = 0; 
        let mut ones = 0;  
        let mut max_diff = i32::MIN;
        
       
        for &ch in s.iter().take(s.len() - 1) {
            if ch == b'0' {
                zeros += 1;
            } else {
                ones += 1;
            }
            max_diff = max_diff.max(zeros - ones);
        }
        
        
        if s[s.len() - 1] == b'1' {
            ones += 1;
        }
        
        max_diff + ones
    }
}