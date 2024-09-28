pub mod solution {
    use crate::{structs::stack::MyStack, NodeItemTraits};

    pub struct StackedQueue<T: NodeItemTraits> {
        input: MyStack<T>,
        output: MyStack<T>,
    }

    impl<T: NodeItemTraits> StackedQueue<T> {
        pub fn new() -> Self {
            Self {
                input: MyStack::new(),
                output: MyStack::new(),
            }
        }

        pub fn change_stacks(&mut self) {
            match (self.input.is_empty(), self.output.is_empty()) {
                (true, false) => loop {
                    let node = self.output.pop();
                    match node {
                        Some(value) => {
                            self.input.push(value);
                        }
                        None => {
                            break;
                        }
                    }
                },
                (false, true) => loop {
                    let node = self.input.pop();
                    match node {
                        Some(value) => {
                            self.output.push(value);
                        }
                        None => {
                            break;
                        }
                    }
                },
                _ => {}
            }
        }

        pub fn push(&mut self, value: T) {
            if self.input.is_empty() {
                self.change_stacks();
            }

            self.input.push(value);
        }

        pub fn pop(&mut self) -> Option<T> {
            if self.output.is_empty() {
                self.change_stacks();
            }

            self.output.pop()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solution::StackedQueue;

    #[test]
    fn test_push_pop() {
        let mut queue = StackedQueue::<i32>::new();

        for l in 0..9 {
            queue.push(l);
        }

        let mut aux = 0;
        while let Some(out) = queue.pop() {
            assert_eq!(aux, out);
            aux += 1;
        }
    }
}
