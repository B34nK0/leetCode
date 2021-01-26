pub struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first = i32::max_value();
        let mut second = i32::max_value();

        for num in nums {
            if num > second {
                return true;
            } else if num > first {
                second = num;
            } else {
                first = num;
            }
        }

        false
    }
}

impl super::Solution for Solution {
    fn increasing_triplet(nums: Vec<i32>) -> bool {
        Self::increasing_triplet(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}