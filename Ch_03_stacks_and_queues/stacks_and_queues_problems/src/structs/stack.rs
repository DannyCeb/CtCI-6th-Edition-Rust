use std::{cell::RefCell, rc::Rc};

use crate::NodeItemTraits;

use super::node::Node;

pub struct MyStack<T: NodeItemTraits> {
    top: Option<Rc<RefCell<Node<T>>>>,
    iter: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: NodeItemTraits> MyStack<T> {
    pub fn new() -> Self {
        Self {
            top: None,
            iter: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.top.clone() {
            Some(_) => false,
            None => true,
        }
    }

    pub fn push(&mut self, value: T) {
        let node = Some(Rc::new(RefCell::new(Node::new(value, self.top.clone()))));

        self.top = node.clone();
        self.iter = self.top.clone();
    }

    pub fn peek(&self) -> Option<T> {
        match self.is_empty() {
            true => None,
            false => Some(self.top.clone().unwrap().as_ref().borrow().item.clone()),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.is_empty() {
            true => None,
            false => {
                let res = self.top.clone().unwrap().as_ref().borrow().item;
                self.top = self.top.clone().unwrap().as_ref().borrow().previous.clone();
                self.iter = self.top.clone();
                Some(res.clone())
            }
        }
    }
}

impl<T: NodeItemTraits> Iterator for MyStack<T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.clone() {
            Some(node) => {
                self.iter = self
                    .iter
                    .clone()
                    .unwrap()
                    .as_ref()
                    .borrow()
                    .previous
                    .clone();

                Some(node.clone())
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MyStack;

    #[test]
    fn test_is_empty() {
        let mut my_stack = MyStack::<i32>::new();

        assert_eq!(true, my_stack.is_empty());

        for l in 0..3 {
            my_stack.push(l);
        }

        assert_eq!(false, my_stack.is_empty());
    }

    #[test]
    fn test_iterator() {
        let mut my_stack = MyStack::<i32>::new();
        let mut v_aux = Vec::<i32>::new();

        assert_eq!(true, my_stack.is_empty());

        for l in 0..10 {
            v_aux.push(l);
            my_stack.push(l);
        }

        let mut counter = 10;
        for node in my_stack.into_iter() {
            counter -= 1;
            assert_eq!(node.clone().as_ref().borrow().item, v_aux[counter]);
        }
    }

    #[test]
    fn test_peek() {
        let mut my_stack = MyStack::<i32>::new();
        assert_eq!(None, my_stack.peek());

        my_stack.push(0);

        assert_eq!(Some(0), my_stack.peek());
    }
}
