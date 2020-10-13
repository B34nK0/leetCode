use super::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod recursive;

pub trait Solution {
    fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            &[Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)] as &[_],
            &[Some(4), Some(7), Some(2), Some(9), Some(6), Some(3), Some(1)] as &[_],
        )];

        for (root, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::invert_tree(test_utilities::make_tree(root.iter().copied())),
                test_utilities::make_tree(expected.iter().copied())
            );
        }
    }
}