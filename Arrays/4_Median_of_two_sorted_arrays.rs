
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
       
        let m = nums1.len();
        let n = nums2.len();

        // make nums1 smaller to narrow down the search space
        let (mut nums1, mut nums2, mut m, mut n) = if m > n {
            (nums2, nums1, n, m)
        } else {
            (nums1, nums2, m, n)
        };

        let mut low = 0;
        let mut high = m;
        let median_pos = (m + n + 1) / 2;

        while low <= high {
            let cut1 = (low + high) / 2;
            let cut2 = median_pos - cut1;

            let l1 = if cut1 == 0 { i32::MIN } else { nums1[cut1 - 1] };
            let l2 = if cut2 == 0 { i32::MIN } else { nums2[cut2 - 1] };
            let r1 = if cut1 == m { i32::MAX } else { nums1[cut1] };
            let r2 = if cut2 == n { i32::MAX } else { nums2[cut2] };

            if l1 <= r2 && l2 <= r1 {
                if (m + n) % 2 != 0 {
                    return f64::from(l1.max(l2));
                } else {
                    return (f64::from(l1.max(l2)) + f64::from(r1.min(r2))) / 2.0;
                }
            } else if l1 > r2 {
                high = cut1 - 1;
            } else {
                low = cut1 + 1;
            }
        }

        0.0
    }
}
