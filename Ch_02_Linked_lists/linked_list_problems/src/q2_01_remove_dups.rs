mod solution {

    use std::{cell::RefCell, cmp::Eq, collections::HashMap, fmt::Display, hash::Hash, rc::Rc};

    use crate::linked_list::{
        ll_error::LinkedListError,
        structures::{MyLinkedList, Node},
    };

    impl<T: Copy + Display + Hash + Eq> MyLinkedList<T> {
        pub fn remove_dups(&mut self) -> Result<String, LinkedListError> {
            let mut buffer: HashMap<T, u8> = HashMap::new();

            let mut v_to_delete: Vec<Rc<RefCell<Node<T>>>> = Vec::new();

            // find duplicates
            for l in self.clone() {
                let i = l.as_ref().borrow().item;

                match buffer.get(&i) {
                    Some(_) => {
                        // save the reference to the dup node
                        v_to_delete.push(l.clone());
                    }
                    None => {
                        buffer.insert(i, 1);
                    }
                };
            }

            // remove duplicates
            for l in 0..v_to_delete.len() {
                // Save the prev and the next node pointers
                let prev = v_to_delete[l].as_ref().borrow().previous.clone();
                let next = v_to_delete[l].as_ref().borrow().next.clone();

                // verify if the prev node is None
                match prev.clone() {
                    Some(prev_node) => prev_node.as_ref().borrow_mut().next = next.clone(),
                    None => self.first = next.clone(),
                }

                // verify if the next node is None
                match next {
                    Some(next_node) => next_node.as_ref().borrow_mut().previous = prev.clone(),
                    None => self.last = prev.clone(),
                }
            }

            Ok("".to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::linked_list::structures::MyLinkedList;

    #[test]
    fn test_remove_dup() {
        let mut my_linked_list: MyLinkedList<i32> = MyLinkedList::new();

        my_linked_list.push_back(0);
        my_linked_list.push_back(1);
        my_linked_list.push_front(-1);
        my_linked_list.push_back(2);
        my_linked_list.push_front(0);
        my_linked_list.push_back(2);

        assert_eq!(Ok("".to_string()), my_linked_list.remove_dups());

        let mut result_list: MyLinkedList<i32> = MyLinkedList::new();

        result_list.push_front(2);
        result_list.push_front(1);
        result_list.push_front(-1);
        result_list.push_front(0);

        assert_eq!(my_linked_list, result_list);
    }
}
