pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as _;
        let mut cache = vec![0; n];

        cache[0] = 1;

        let mut i2 = 0;
        let mut i3 = 0;
        let mut i5 = 0;

        for i in 1..n {
            let v2 = cache[i2] * 2;
            let v3 = cache[i3] * 3;
            let v5 = cache[i5] * 5;

            cache[i] = match v2.cmp(&v3) {
                Ordering::Less => match v2.cmp(&v5) {
                    Ordering::Less => {
                        i2 += 1;

                        v2
                    }
                    Ordering::Equal => {
                        i2 += 1;
                        i5 += 1;

                        v2
                    }
                    Ordering::Greater => {
                        i5 += 1;

                        v5
                    }
                },
                Ordering::Equal => match v2.cmp(&v5) {
                    Ordering::Less => {
                        i2 += 1;
                        i3 += 1;

                        v2
                    }
                    Ordering::Equal => {
                        i2 += 1;
                        i3 += 1;
                        i5 += 1;

                        v2
                    }
                    Ordering::Greater => {
                        i5 += 1;

                        v5
                    }
                },
                Ordering::Greater => match v3.cmp(&v5) {
                    Ordering::Less => {
                        i3 += 1;

                        v3
                    }
                    Ordering::Equal => {
                        i3 += 1;
                        i5 += 1;

                        v3
                    }
                    Ordering::Greater => {
                        i5 += 1;

                        v5
                    }
                },
            }
        }

        cache[n - 1]
    }
}

impl super::Solution for Solution {
    fn nth_ugly_number(n: i32) -> i32 {
        Self::nth_ugly_number(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}