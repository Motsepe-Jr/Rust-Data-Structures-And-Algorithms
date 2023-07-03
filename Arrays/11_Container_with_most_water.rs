// BRUTE FORCE SOLUTION (N ^ 2)

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {

        let mut res = 0;

        for l in 0..height.len() {
            for r in (l + 1)..height.len() {
                let area = (r - l) as i32 * height[l].min(height[r]);
                res = res.max(area);
            }
        }

        res
        
    }
}

// OPTIMUM SOLUTION WITH LINEAR TIME (N)


impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        

        let mut res = 0;
        let mut r = height.len() - 1;
        let mut l = 0;

        while l < r {
            let area = (r - l) as i32 * height[l].min(height[r]);
            res = res.max(area);

              // maximize the area, so move small height
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        res
    }
}