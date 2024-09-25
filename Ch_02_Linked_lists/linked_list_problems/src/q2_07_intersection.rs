pub mod solution {
    use crate::{linked_list::structures::MyLinkedList, NodeItemTraits};

    pub fn intersection<T: NodeItemTraits>(l1: &MyLinkedList<T>, l2: &MyLinkedList<T>) -> bool {
        for pointer_1 in l1.clone().into_iter() {
            for pointer_2 in l2.clone().into_iter() {
                if std::ptr::eq(pointer_1.as_ptr(), pointer_2.as_ptr()) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        linked_list::structures::MyLinkedList, q2_07_intersection::solution::intersection,
    };

    #[test]
    fn test_same_values_different_lists() {
        let mut l1 = MyLinkedList::<i32>::new();
        let mut l2 = MyLinkedList::<i32>::new();

        for l in [1, 2, 3] {
            l1.push_back(l);
            l2.push_back(l);
        }

        assert_eq!(false, intersection(&l1, &l2));
    }

    #[test]
    fn test_sub_list() {
        let mut l1 = MyLinkedList::<i32>::new();

        for l in [1, 2, 3] {
            l1.push_back(l);
        }

        let l2 = l1.clone();

        l1.push_front(-1);
        l1.push_front(-2);
        l1.push_back(4);

        assert_eq!(true, intersection(&l1, &l2));
    }
}
