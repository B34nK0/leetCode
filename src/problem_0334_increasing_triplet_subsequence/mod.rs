pub mod iterative;
pub mod iterative_2;
pub mod iterative_3;

pub trait Solution {
    fn increasing_triplet(nums: Vec<i32>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 2, 3, 4, 5] as &[_], true), (&[5, 4, 3, 2, 1], false)];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::increasing_triplet(nums.to_vec()), expected);
        }
    }
}
