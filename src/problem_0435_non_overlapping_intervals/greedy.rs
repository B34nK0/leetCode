pub struct Solution {}

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|x| x[1]);

        let mut last_finish_time = i32::min_value();

        let k = intervals
            .iter()
            .filter(|x| {
                let r = x[0] >= last_finish_time;

                if r {
                    last_finish_time = x[1];
                }

                r
            })
            .count();

        (intervals.len() - k) as _
    }
}

impl super::Solution for Solution {
    fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        Self::erase_overlap_intervals(intervals)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
