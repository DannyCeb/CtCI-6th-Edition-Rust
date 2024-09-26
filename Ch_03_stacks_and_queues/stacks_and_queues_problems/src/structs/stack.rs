use std::{borrow::Borrow, cell::RefCell, rc::Rc};

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

    pub fn push(&mut self, value: T) -> T {
        let node = Some(Rc::new(RefCell::new(Node::new(value, self.top.clone()))));

        self.top = node.clone();
        self.iter = self.top.clone();

        value
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
