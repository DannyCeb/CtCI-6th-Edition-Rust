pub mod solution {

    use crate::{linked_list::structures::MyLinkedList, NodeItemTraits};

    pub fn return_kth_to_last<T: NodeItemTraits>(list: &MyLinkedList<T>, k: usize) -> Option<T> {
        let mut counter: usize = 0;

        for l in list.clone().rev() {
            if counter == k {
                return Some(l.borrow().item);
            }
            counter += 1;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        linked_list::structures::MyLinkedList,
        q2_02_return_kth_to_last::solution::return_kth_to_last,
    };

    #[test]
    fn test_n_element() {
        let mut my_l = MyLinkedList::<i32>::new();

        for l in 0..10 {
            my_l.push_back(l);
        }

        assert_eq!(Some(8), return_kth_to_last(&my_l, 1));

        assert_eq!(Some(2), return_kth_to_last(&my_l, 7));

        assert_eq!(None, return_kth_to_last(&my_l, 20));
    }
}
