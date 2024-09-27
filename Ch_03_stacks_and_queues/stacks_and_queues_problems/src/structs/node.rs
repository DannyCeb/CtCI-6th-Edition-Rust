use std::{cell::RefCell, rc::Rc};

use crate::NodeItemTraits;

#[derive(Debug)]
pub struct Node<T: NodeItemTraits> {
    pub item: T,
    pub previous: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: NodeItemTraits> Node<T> {
    pub fn new(item: T, previous: Option<Rc<RefCell<Node<T>>>>) -> Self {
        Self { item, previous }
    }
}
