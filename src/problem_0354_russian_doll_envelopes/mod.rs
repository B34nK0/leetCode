pub mod longest_increasing_subsequence;

pub trait Solution {
    fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[[5, 4], [6, 4], [6, 7], [2, 3]] as &[[_; 2]], 3)];

        for (envelopes, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::max_envelopes(envelopes.iter().map(|p| p.to_vec()).collect()),
                expected
            );
        }
    }
}