pub mod solution {
    use std::ops::Deref;

    use crate::{structs::stack::MyStack, NodeItemTraits};

    #[derive(Debug)]
    pub struct SizedStack<T: NodeItemTraits> {
        stack: MyStack<T>,
        capacity: u32,
        size: u32,
    }

    impl<T: NodeItemTraits> Deref for SizedStack<T> {
        type Target = MyStack<T>;
        fn deref(&self) -> &Self::Target {
            &self.stack
        }
    }

    impl<T: NodeItemTraits> SizedStack<T> {
        pub fn new(capacity: u32) -> Self {
            Self {
                stack: MyStack::new(),
                capacity,
                size: 0,
            }
        }

        pub fn is_full(&self) -> bool {
            match self.capacity.cmp(&self.size) {
                std::cmp::Ordering::Equal | std::cmp::Ordering::Less => true,
                _ => false,
            }
        }

        pub fn push(&mut self, value: T) {
            match self.is_full() {
                false => {
                    self.stack.push(value);
                    self.size += 1;
                }
                _ => {}
            }
        }

        pub fn pop(&mut self) -> Option<T> {
            match self.stack.pop() {
                Some(v) => {
                    self.size -= 1;
                    Some(v)
                }
                _ => None,
            }
        }
    }

    #[derive(Debug)]
    pub struct StackOfPlates<T: NodeItemTraits> {
        pub stacks: Vec<SizedStack<T>>,
        capacity: u32,
    }

    impl<T: NodeItemTraits> StackOfPlates<T> {
        pub fn new(capacity: u32) -> Self {
            Self {
                stacks: vec![SizedStack::new(capacity)],
                capacity,
            }
        }

        pub fn push(&mut self, value: T) {
            for l in 0..self.stacks.len() {
                if !self.stacks[l].is_full() {
                    self.stacks[l].push(value);
                    return;
                }
            }

            let mut stack = SizedStack::new(self.capacity);
            stack.push(value);
            self.stacks.push(stack);
        }

        pub fn pop(&mut self) -> Option<T> {
            let index = self.stacks.len() - 1;
            let res = self.stacks[index].pop();

            let is_last_empty = self.stacks[index].is_empty();

            match is_last_empty {
                true => {
                    self.stacks.pop();
                }
                _ => {}
            }

            res
        }

        pub fn pop_at(&mut self, index: usize) -> Option<T> {
            if index > self.stacks.len() {
                None
            } else {
                self.stacks[index].pop()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solution::StackOfPlates;

    #[test]
    fn test_push_pop() {
        let mut my_stack_of_plates = StackOfPlates::<i32>::new(3);

        for l in 0..9 {
            my_stack_of_plates.push(l);
        }

        assert_eq!(3, my_stack_of_plates.stacks.len());
        assert_eq!(Some(8), my_stack_of_plates.pop());
        assert_eq!(Some(7), my_stack_of_plates.pop());
        assert_eq!(Some(6), my_stack_of_plates.pop());
        assert_eq!(2, my_stack_of_plates.stacks.len());

        for l in 6..15 {
            my_stack_of_plates.push(l);
        }

        assert_eq!(5, my_stack_of_plates.stacks.len());
    }

    #[test]
    fn test_pop_pop_at() {
        let mut my_stack_of_plates = StackOfPlates::<i32>::new(3);

        for l in 0..9 {
            my_stack_of_plates.push(l);
        }

        assert_eq!(Some(2), my_stack_of_plates.pop_at(0));
        my_stack_of_plates.push(15);
        assert_eq!(3, my_stack_of_plates.stacks.len());
        assert_eq!(Some(15), my_stack_of_plates.pop_at(0));
    }
}
