use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {

        let mut visit = HashSet::new();


        while !visit.contains(&n) {
            visit.insert(n);

            n = Self::sum_of_squares(n);

            if n == 1 {
                return true;
            }
        }

        false
    }

    fn sum_of_squares(mut n: i32) -> i32 {
        let mut sum_values = 0;

        while n > 0 {
            let mut digit = n % 10;
            n /= 10;
            sum_values += digit * digit

        } 

        sum_values  
    }
}