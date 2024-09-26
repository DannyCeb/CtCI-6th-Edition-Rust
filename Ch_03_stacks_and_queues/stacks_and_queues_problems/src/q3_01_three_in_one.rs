mod solution {

    struct StackPointers {
        end_of_stack: usize,
        top_of_stack: usize,
    }

    impl StackPointers {
        pub fn new(end_of_stack: usize, top_of_stack: usize) -> Self {
            Self {
                end_of_stack,
                top_of_stack,
            }
        }
    }

    pub struct StackSimulation {
        pub stacks_data: Vec<i32>,
        stack_pointers: (StackPointers, StackPointers, StackPointers),
    }

    impl StackSimulation {
        pub fn new() -> Self {
            Self {
                stacks_data: Vec::from([0, 0, 0, 0, 0, 0]),
                stack_pointers: (
                    StackPointers::new(0, 1),
                    StackPointers::new(2, 3),
                    StackPointers::new(4, 5),
                ),
            }
        }

        pub fn push(&mut self, stack_number: usize, value: i32) {
            match stack_number {
                1 => {
                    self.stacks_data
                        .insert(self.stack_pointers.0.top_of_stack, value);
                    self.stack_pointers.0.top_of_stack += 1;
                    self.stack_pointers.1.end_of_stack += 1;
                    self.stack_pointers.1.top_of_stack += 1;
                    self.stack_pointers.2.end_of_stack += 1;
                    self.stack_pointers.2.top_of_stack += 1;
                }
                2 => {
                    self.stacks_data
                        .insert(self.stack_pointers.1.top_of_stack, value);
                    self.stack_pointers.1.top_of_stack += 1;
                    self.stack_pointers.2.end_of_stack += 1;
                    self.stack_pointers.2.top_of_stack += 1;
                }
                3 => {
                    self.stacks_data
                        .insert(self.stack_pointers.2.top_of_stack, value);
                    self.stack_pointers.2.top_of_stack += 1;
                }
                _ => {
                    return;
                }
            }
        }

        pub fn is_empty(&self, stack_number: usize) -> bool {
            match stack_number {
                1 => self.stack_pointers.0.top_of_stack == self.stack_pointers.0.end_of_stack + 1,
                2 => self.stack_pointers.1.top_of_stack == self.stack_pointers.1.end_of_stack + 1,
                3 => self.stack_pointers.2.top_of_stack == self.stack_pointers.2.end_of_stack + 1,
                _ => true,
            }
        }

        pub fn pop(&mut self, stack_number: usize) -> Option<i32> {
            match stack_number {
                1 => {
                    if !self.is_empty(stack_number) {
                        self.stack_pointers.0.top_of_stack -= 1;
                        self.stack_pointers.1.end_of_stack -= 1;
                        self.stack_pointers.1.top_of_stack -= 1;
                        self.stack_pointers.2.end_of_stack -= 1;
                        self.stack_pointers.2.top_of_stack -= 1;

                        Some(self.stacks_data.remove(self.stack_pointers.0.top_of_stack))
                    } else {
                        None
                    }
                }
                2 => {
                    if !self.is_empty(stack_number) {
                        self.stack_pointers.1.top_of_stack -= 1;
                        self.stack_pointers.2.end_of_stack -= 1;
                        self.stack_pointers.2.top_of_stack -= 1;

                        Some(self.stacks_data.remove(self.stack_pointers.1.top_of_stack))
                    } else {
                        None
                    }
                }
                3 => {
                    if !self.is_empty(stack_number) {
                        self.stack_pointers.2.top_of_stack -= 1;
                        Some(self.stacks_data.remove(self.stack_pointers.2.top_of_stack))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solution::StackSimulation;

    #[test]
    fn test_is_empty() {
        let my_stacks = StackSimulation::new();

        assert_eq!(true, my_stacks.is_empty(1));
        assert_eq!(true, my_stacks.is_empty(2));
        assert_eq!(true, my_stacks.is_empty(3));
    }

    #[test]
    fn test_push() {
        let mut my_stacks = StackSimulation::new();

        my_stacks.push(1, 10);
        my_stacks.push(1, 10);
        my_stacks.push(1, 10);
        my_stacks.push(1, 10);

        my_stacks.push(3, 30);
        my_stacks.push(3, 30);
        my_stacks.push(3, 30);

        my_stacks.push(2, 20);
        my_stacks.push(2, 20);
        my_stacks.push(2, 20);

        my_stacks.push(1, 10);
        my_stacks.push(1, 10);
        my_stacks.push(1, 10);
        my_stacks.push(1, 10);
        my_stacks.push(1, 10);
        my_stacks.push(1, 10);
        my_stacks.push(1, 10);

        my_stacks.push(2, 20);
        my_stacks.push(2, 20);
        my_stacks.push(2, 20);

        my_stacks.push(2, 20);
        my_stacks.push(2, 20);
        my_stacks.push(2, 20);

        my_stacks.push(3, 30);
        my_stacks.push(3, 30);
        my_stacks.push(3, 30);
        my_stacks.push(3, 30);
        my_stacks.push(3, 30);
        my_stacks.push(3, 30);
        my_stacks.push(3, 30);
        my_stacks.push(3, 30);
        my_stacks.push(3, 30);

        assert_eq!(
            Vec::from([
                0, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 0, 0, 20, 20, 20, 20, 20, 20, 20,
                20, 20, 0, 0, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 0
            ]),
            my_stacks.stacks_data.clone()
        )
    }

    #[test]
    fn test_pop() {
        let mut my_stacks = StackSimulation::new();

        assert_eq!(None, my_stacks.pop(1));

        my_stacks.push(1, 10);
        my_stacks.push(1, 10);
        my_stacks.push(1, 10);
        my_stacks.push(1, 10);
        assert_eq!(Some(10), my_stacks.pop(1));
    }
}
