mod solution {
    use std::{i32, ops::Deref};

    use crate::structs::stack::MyStack;

    pub struct MinStack {
        stack: MyStack<i32>,
        pub min: i32,
    }

    impl MinStack {
        pub fn new() -> Self {
            Self {
                stack: MyStack::new(),
                min: i32::MAX,
            }
        }

        pub fn push(&mut self, value: i32) {
            self.stack.push(value);

            self.min = std::cmp::min(self.min, value);
        }
    }

    impl Deref for MinStack {
        type Target = MyStack<i32>;

        fn deref(&self) -> &Self::Target {
            &self.stack
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solution::MinStack;

    #[test]
    fn test_min() {
        let mut my_min_stack = MinStack::new();

        for l in [1, 5, -7, 3, 2, 0, -2, 1, 7, 10, -5] {
            my_min_stack.push(l);
        }

        assert_eq!(-7, my_min_stack.min);
    }
}
