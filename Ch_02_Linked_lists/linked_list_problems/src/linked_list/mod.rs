use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

/**
* Struct Node
* Contains any type of value which implements the Copy and the Display trait
* points to next and previous using
*      option enum (For none variant)
*      Rc For different refferences
*      RefCell for inner mutability
*/

pub struct Node<T: Copy + Display> {
    item: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    previous: Option<Rc<RefCell<Node<T>>>>,
}

// Impl to create new nodes
impl<T: Copy + Display> Node<T> {
    pub fn new(
        item: T,
        next: Option<Rc<RefCell<Node<T>>>>,
        previous: Option<Rc<RefCell<Node<T>>>>,
    ) -> Self {
        Self {
            item,
            next,
            previous,
        }
    }
}

/**
 * Struct LinkedList
 * Contains any number of nodes
 * Keep track of the first and the last node
 * its able to:
 *      Push back
 *      Push front
 *      
 *      Be displayed to the console using the Display trait
 *
*/

pub struct LinkedList<T: Copy + Display> {
    first: Option<Rc<RefCell<Node<T>>>>,
    last: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Copy + Display> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            first: None,
            last: None,
        }
    }

    pub fn push_front(&mut self, item: T) {
        let new_node = Rc::new(RefCell::new(Node::new(item, self.first.clone(), None)));

        match &self.first {
            Some(first_node) => {
                first_node.borrow_mut().previous = Some(new_node.clone());
                self.first = Some(new_node.clone());
            }
            None => {
                self.first = Some(new_node.clone());
            }
        }

        if self.last.is_none() {
            self.last = Some(new_node);
        }
    }

    pub fn push_back(&mut self, item: T) {
        let new_node = Rc::new(RefCell::new(Node::new(item, None, self.last.clone())));

        match &self.last {
            Some(last_node) => {
                last_node.borrow_mut().next = Some(new_node.clone());
                self.last = Some(new_node.clone());
            }
            None => {
                self.last = Some(new_node.clone());
            }
        }

        if self.first.is_none() {
            self.first = Some(new_node)
        }
    }
}

impl<T: Copy + Display> std::fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut aux_node = self.first.clone();
        let mut str_res = "[".to_string();
        let mut aux = 0;

        loop {
            match &aux_node {
                Some(node) => {
                    let val = node.borrow().item;
                    if aux != 0 {
                        str_res = format!("{},{}", str_res, val);
                    } else {
                        str_res = format!("{}{}", str_res, val);
                        aux = 1;
                    }
                }
                None => {
                    break;
                }
            }

            aux_node = aux_node.unwrap().borrow().next.clone();
        }

        str_res.push_str("]");

        write!(f, "{}", str_res)
    }
}
