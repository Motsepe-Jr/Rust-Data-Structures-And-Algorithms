struct Interval {
    start: i32,
    end: i32,
}

struct Solution;

impl Solution {
    fn min_meeting_rooms(intervals: Vec<Interval>) -> i32 {
        let mut start: Vec<i32> = intervals.iter().map(|i| i.start).collect();
        let mut end: Vec<i32> = intervals.iter().map(|i| i.end).collect();

        start.sort();
        end.sort();

        let mut res = 0;
        let mut count = 0;
        let mut s = 0;
        let mut e = 0;

        while s < intervals.len() {
            if start[s] < end[e] {
                s += 1;
                count += 1;
            } else {
                e += 1;
                count -= 1;
            }
            res = res.max(count);
        }

        res
    }
}