use std::cell::RefCell;

use std::rc::Rc;

use crate::NodeItemTraits;

use super::ll_error::LinkedListError;

/*
* Struct Node
* Contains any type of value which implements the Copy, Hash, Eq and the Display trait
* points to next and previous using
*      option enum (For none variant)
*      Rc For different refferences
*      RefCell for inner mutability
*/
#[derive(Debug)]
pub struct Node<T: NodeItemTraits> {
    pub item: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub previous: Option<Rc<RefCell<Node<T>>>>,
}

// Impl to create new nodes
impl<T: NodeItemTraits> Node<T> {
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
 *      Be displayed to the console using the Display trait
 *
*/
#[derive(Clone)]
pub struct MyLinkedList<T: NodeItemTraits> {
    pub first: Option<Rc<RefCell<Node<T>>>>,
    pub last: Option<Rc<RefCell<Node<T>>>>,
    iter: Option<Rc<RefCell<Node<T>>>>,
    iter_back: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: NodeItemTraits> MyLinkedList<T> {
    pub fn new() -> Self {
        MyLinkedList {
            first: None,
            last: None,
            iter: None,
            iter_back: None,
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

        self.iter = self.first.clone();
        self.iter_back = self.last.clone();
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
        self.iter = self.first.clone();
        self.iter_back = self.last.clone();
    }

    pub fn pop_back(&mut self) -> Result<T, LinkedListError> {
        if let Some(pointer) = self.last.clone() {
            let t_val = pointer.as_ref().borrow().item;

            if pointer.clone().as_ref().borrow().previous.is_none() {
                self.first = None;
                self.last = None;
            } else {
                let prev_p = pointer.clone().as_ref().borrow().previous.clone().unwrap();
                prev_p.as_ref().borrow_mut().next = None;
                self.last = Some(prev_p.clone());
            }

            Ok(t_val)
        } else {
            Err(LinkedListError::EmptyList)
        }
    }
}

#[allow(unused_assignments)]
impl<T: NodeItemTraits> Iterator for MyLinkedList<T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.clone() {
            Some(res) => {
                self.iter = self.iter.clone().unwrap().as_ref().borrow().next.clone();
                Some(res)
            }
            None => None,
        }
    }
}

#[allow(unused_assignments)]
impl<T: NodeItemTraits> DoubleEndedIterator for MyLinkedList<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.iter_back.clone() {
            Some(res) => {
                self.iter_back = self
                    .iter_back
                    .clone()
                    .unwrap()
                    .as_ref()
                    .borrow()
                    .previous
                    .clone();
                Some(res)
            }
            None => None,
        }
    }
}

impl<T: NodeItemTraits> std::fmt::Display for MyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut aux_node = self.first.clone();
        let mut str_res = "[".to_string();
        let mut aux = 0;

        loop {
            match &aux_node {
                Some(node) => {
                    let val = node.as_ref().borrow().item;
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

            aux_node = aux_node.unwrap().as_ref().borrow().next.clone();
        }

        str_res.push_str("]");

        write!(f, "{}", str_res)
    }
}

impl<T: NodeItemTraits> FromIterator<T> for MyLinkedList<T> {
    fn from_iter<K: IntoIterator<Item = T>>(iter: K) -> Self {
        let mut list: MyLinkedList<T> = MyLinkedList::new();

        for l in iter {
            list.push_back(l);
        }

        list
    }
}

impl<T: NodeItemTraits> PartialEq for MyLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        let mut myself = self.clone();
        let mut other_list = other.clone();

        loop {
            match (myself.next(), other_list.next()) {
                (Some(val1), Some(val2)) => {
                    if val1.as_ref().borrow().item != val2.as_ref().borrow().item {
                        return false;
                    }
                }
                (None, None) => {
                    break;
                }
                (_, _) => {
                    return false;
                }
            }
        }
        true
    }
}

impl<T: NodeItemTraits> core::fmt::Debug for MyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut aux_node = self.first.clone();
        let mut str_res = "[".to_string();
        let mut aux = 0;

        loop {
            match &aux_node {
                Some(node) => {
                    let val = node.as_ref().borrow().item;
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

            aux_node = aux_node.unwrap().as_ref().borrow().next.clone();
        }

        str_res.push_str("]");

        write!(f, "{}", str_res)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::{MyLinkedList, Node};

    #[test]
    fn test_insert() {
        let mut my_list: MyLinkedList<i32> = MyLinkedList::new();

        my_list.push_back(7);

        let mut res_ll: MyLinkedList<i32> = MyLinkedList::new();

        res_ll.first = Some(Rc::new(RefCell::new(Node::new(7, None, None))));
        res_ll.last = res_ll.first.clone();
        res_ll.iter = res_ll.first.clone();
        res_ll.iter_back = res_ll.last.clone();

        assert_eq!(my_list, res_ll);
    }
}
