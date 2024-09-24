pub mod solution {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        linked_list::{ll_error::LinkedListError, structures::Node},
        NodeItemTraits,
    };

    pub fn delete_middle_node<T: NodeItemTraits>(
        node: Rc<RefCell<Node<T>>>,
    ) -> Result<String, LinkedListError> {
        let prev = node.as_ref().borrow().previous.clone();
        let next = node.as_ref().borrow().next.clone();

        match (&prev, &next) {
            (Some(node_prev), Some(node_next)) => {
                node_prev.as_ref().borrow_mut().next = next.clone();
                node_next.as_ref().borrow_mut().previous = prev.clone();
                Ok(format!(
                    "Element with the value: {} removed",
                    node.as_ref().borrow().item
                ))
            }
            (_, _) => Err(LinkedListError::NotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        linked_list::structures::MyLinkedList,
        q2_03_delete_middle_node::solution::delete_middle_node,
    };

    #[test]
    fn test_delete_middle_node() {
        let mut l1: MyLinkedList<char> = MyLinkedList::new();
        l1.push_back('a');
        l1.push_back('b');
        l1.push_back('b');
        let node = l1.first.clone().unwrap().as_ref().borrow().next.clone(); // get the second value

        assert_eq!(
            Ok("Element with the value: b removed".to_string()),
            delete_middle_node(node.unwrap().clone())
        );

        let mut l2: MyLinkedList<char> = MyLinkedList::new();
        l2.push_back('a');
        l2.push_back('b');

        assert_eq!(l1, l2)
    }
}
