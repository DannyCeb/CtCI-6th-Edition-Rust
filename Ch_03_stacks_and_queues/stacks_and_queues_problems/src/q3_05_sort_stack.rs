pub mod solution {
    use std::ops::Deref;

    use crate::structs::stack::MyStack;

    pub struct SortStack {
        stack: MyStack<i32>,
        len: usize,
    }

    impl SortStack {
        pub fn new() -> Self {
            Self {
                stack: MyStack::new(),
                len: 0,
            }
        }
        pub fn push(&mut self, value: i32) {
            self.stack.push(value);
            self.len += 1;
        }

        pub fn pop(&mut self) -> Option<i32> {
            match self.stack.pop() {
                Some(res) => {
                    self.len -= 1;
                    Some(res)
                }
                None => None,
            }
        }

        fn sort_stack(&mut self, index: usize) {
            if index == self.len {
                return;
            } else {
                let mut aux_stack = MyStack::<i32>::new();
                let mut grather = self.stack.pop().unwrap();

                let mut l = index + 1;

                while l < self.len {
                    let aux = self.stack.pop().unwrap();

                    grather = match grather.cmp(&aux) {
                        std::cmp::Ordering::Less => {
                            aux_stack.push(grather);
                            aux
                        }
                        _ => {
                            aux_stack.push(aux);
                            grather
                        }
                    };

                    l += 1;
                }
                self.stack.push(grather);

                while let Some(value) = aux_stack.pop() {
                    self.stack.push(value);
                }

                self.sort_stack(index + 1);
            }
        }

        pub fn sort(&mut self) {
            if !self.is_empty() {
                self.sort_stack(0);
            }
        }
    }

    impl Deref for SortStack {
        type Target = MyStack<i32>;

        fn deref(&self) -> &Self::Target {
            &self.stack
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solution::SortStack;

    #[test]
    fn test_sort() {
        let mut my_stack = SortStack::new();

        for l in [1, 10, 2, 6, 5, 4, 9, 7, 8, 3] {
            my_stack.push(l);
        }

        my_stack.sort();

        for l in 1..11 {
            assert_eq!(Some(l), my_stack.pop())
        }
    }
}
