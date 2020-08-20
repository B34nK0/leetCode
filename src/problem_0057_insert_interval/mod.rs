pub mod binary_search;
pub mod binary_search_2;
pub mod binary_search_3;

pub trait Solution {
    fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[1, 3], [6, 9]] as &[_], [2, 5]), &[[1, 5], [6, 9]] as &[_]),
            (
                (&[[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]], [4, 8]),
                &[[1, 2], [3, 10], [12, 16]],
            ),
        ];

        for ((intervals, new_interval), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::insert(
                    intervals.iter().map(|interval| interval.to_vec()).collect(),
                    new_interval.to_vec()
                ),
                expected
            );
        }
    }
}