mod solution {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        linked_list::structures::{MyLinkedList, Node},
        NodeItemTraits,
    };

    pub fn detect_loop<T: NodeItemTraits>(l1: &MyLinkedList<T>) -> bool {
        let mut v_ref: Vec<Rc<RefCell<Node<T>>>> = vec![];

        for pointer in l1.clone().into_iter() {
            for pointer2 in &v_ref {
                if std::ptr::eq(pointer.clone().as_ptr(), pointer2.clone().as_ptr()) {
                    return true;
                }
            }
            v_ref.push(pointer.clone());
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use crate::{
        linked_list::structures::MyLinkedList, q2_08_loop_detection::solution::detect_loop,
    };

    #[test]
    fn test_no_loop() {
        let mut l1 = MyLinkedList::<i32>::new();

        for l in [0, 1, 2, 3] {
            l1.push_back(l);
        }

        assert_eq!(false, detect_loop(&l1));
    }

    #[test]
    fn test_loop() {
        let mut l1 = MyLinkedList::<i32>::new();

        for l in [0, 1, 2, 3, 4, 5] {
            l1.push_back(l);
        }

        let second_value = l1.first.clone();
        let second_value = second_value.unwrap();
        let second_value = second_value.as_ref().borrow().next.clone();

        l1.last
            .clone()
            .as_ref()
            .borrow_mut()
            .unwrap()
            .as_ref()
            .borrow_mut()
            .next = second_value;

        assert_eq!(true, detect_loop(&l1));
    }
}
