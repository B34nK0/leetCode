pub mod double_ended_stack;

pub trait MyQueue {
    fn new() -> Self;
    fn push(&mut self, x: i32);
    fn pop(&mut self) -> i32;
    fn peek(&self) -> i32;
    fn empty(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::MyQueue;

    enum QueueOperation {
        Push(i32),
        Pop(i32),
        Peek(i32),
        Empty(bool),
    }

    pub fn run_tests<Q: MyQueue>() {
        use QueueOperation::{Empty, Peek, Pop, Push};

        let test_cases = vec![vec![Push(1), Push(2), Peek(1), Pop(1), Empty(false)]];

        for test_case in test_cases {
            let mut queue = Q::new();

            for operation in test_case {
                match operation {
                    Push(value) => queue.push(value),
                    Pop(value) => assert_eq!(queue.pop(), value),
                    Peek(value) => assert_eq!(queue.peek(), value),
                    Empty(value) => assert_eq!(queue.empty(), value),
                }
            }
        }
    }
}
