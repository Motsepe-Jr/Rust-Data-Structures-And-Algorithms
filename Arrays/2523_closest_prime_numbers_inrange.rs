impl Solution {
    pub fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 || n == 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5; // 6k + 1 -> 5, 7, 11, 13, 17, 19
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut primes = Vec::new();
        
        
        for num in left..=right {
            if Self::is_prime(num) {
                primes.push(num);
            }
        }

   
        if primes.len() < 2 {
            return vec![-1, -1];
        }

        let mut min_gap = i32::MAX;
        let mut result = vec![-1, -1];

        for i in 0..primes.len() - 1 {
            let gap = primes[i + 1] - primes[i];
            if gap < min_gap {
                min_gap = gap;
                result = vec![primes[i], primes[i + 1]];
            }
        }

        result
    }
}
