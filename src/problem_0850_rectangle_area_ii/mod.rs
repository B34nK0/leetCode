pub mod sweep_line_with_segment_tree;

pub trait Solution {
    fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 0, 2, 2], [1, 0, 2, 3], [1, 0, 3, 1]] as &dyn Matrix<_>, 6),
            (&[[0, 0, 1_000_000_000, 1_000_000_000]], 49),
            (&[[25, 20, 70, 27], [68, 80, 79, 100], [37, 41, 66, 76]], 1550),
            (&[[49, 40, 62, 100], [11, 83, 31, 99], [19, 39, 30, 99]], 1584),
            (&[[39, 99, 60, 100], [69, 14, 91, 56], [13, 42, 20, 70]], 1141),
            (
                &[
                    [93516, 44895, 94753, 69358],
                    [13141, 52454, 59740, 71232],
                    [22877, 11159, 85255, 61703],
                    [11917, 8218, 84490, 36637],
                    [75914, 29447, 83941, 64384],
                    [22490, 71433, 64258, 74059],
                    [18433, 51177, 87595, 98688],
                    [70854, 80720, 91838, 92304],
                    [46522, 49839, 48550, 94096],
                    [95435, 37993, 99139, 49382],
                    [10618, 696, 33239, 45957],
                    [18854, 2818, 57522, 78807],
                    [61229, 36593, 76550, 41271],
                    [99381, 90692, 99820, 95125],
                ],
                971_243_962,
            ),
            (&[[71, 73, 72, 84], [56, 60, 64, 64], [82, 8, 85, 95]], 304),
        ];

        for (rectangles, expected) in test_cases {
            assert_eq!(S::rectangle_area(rectangles.to_vec()), expected);
        }
    }
}
