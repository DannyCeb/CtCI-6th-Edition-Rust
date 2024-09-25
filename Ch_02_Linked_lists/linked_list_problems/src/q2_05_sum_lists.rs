pub mod solution {

    use crate::linked_list::structures::MyLinkedList;

    pub fn sum_lists_in_reverse(
        l1: &MyLinkedList<i32>,
        l2: &MyLinkedList<i32>,
    ) -> MyLinkedList<i32> {
        let mut res = MyLinkedList::<i32>::new();

        let mut iter1 = l1.clone().into_iter();
        let mut iter2 = l2.clone().into_iter();

        let mut carry = 0;
        let mut l_bool = true;

        while l_bool {
            let sum = match (iter1.next(), iter2.next()) {
                (Some(val1), Some(val2)) => {
                    val1.as_ref().borrow().item + val2.as_ref().borrow().item + carry
                }
                (Some(val1), None) => val1.as_ref().borrow().item + carry,
                (None, Some(val2)) => val2.as_ref().borrow().item + carry,
                (None, None) => {
                    l_bool = false;
                    carry
                }
            };

            let value = sum % 10;
            carry = sum / 10;
            res.push_back(value);
        }

        if let Some(item) = res.last.clone() {
            if item.as_ref().borrow().item == 0 {
                let _ = res.pop_back();
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use crate::{
        linked_list::structures::MyLinkedList, q2_05_sum_lists::solution::sum_lists_in_reverse,
    };

    #[test]
    fn test_sum_list_reverse_book_case() {
        let mut l1: MyLinkedList<i32> = MyLinkedList::new();

        for l in [7, 1, 6] {
            l1.push_back(l);
        }

        let mut l2 = MyLinkedList::<i32>::new();

        for l in [5, 9, 2] {
            l2.push_back(l);
        }

        let mut res = MyLinkedList::<i32>::new();

        for l in [2, 1, 9] {
            res.push_back(l);
        }

        assert_eq!(res, sum_lists_in_reverse(&l1, &l2));
    }

    #[test]
    fn test_sum_list_reverse_different_lenght() {
        let mut l1: MyLinkedList<i32> = MyLinkedList::new();

        for l in [9, 1, 1] {
            l1.push_back(l);
        }

        let mut l2 = MyLinkedList::<i32>::new();

        for l in [0, 1] {
            l2.push_back(l);
        }

        let mut res = MyLinkedList::<i32>::new();

        for l in [9, 2, 1] {
            res.push_back(l);
        }

        assert_eq!(res, sum_lists_in_reverse(&l1, &l2));
    }

    #[test]
    fn test_sum_list_big_case() {
        // [4, 2, 7, 8, 9, 1, 2, 3, 5, 0, 9, 1, 3, 7, 2, 6, 5, 9, 1, 3]
        // [8, 4, 5, 1, 2, 3, 7, 9, 2, 1, 3, 5, 0, 9, 8, 2, 4, 6, 1, 2]
        // [2, 7, 2, 0, 2, 5, 9, 2, 8, 1, 2, 7, 3, 6, 1, 9, 9, 5, 3, 5]

        let mut l1: MyLinkedList<i32> = MyLinkedList::new();

        for l in [4, 2, 7, 8, 9, 1, 2, 3, 5, 0, 9, 1, 3, 7, 2, 6, 5, 9, 1, 3] {
            l1.push_back(l);
        }

        let mut l2 = MyLinkedList::<i32>::new();

        for l in [8, 4, 5, 1, 2, 3, 7, 9, 2, 1, 3, 5, 0, 9, 8, 2, 4, 6, 1, 2] {
            l2.push_back(l);
        }

        let mut res = MyLinkedList::<i32>::new();

        for l in [2, 7, 2, 0, 2, 5, 9, 2, 8, 1, 2, 7, 3, 6, 1, 9, 9, 5, 3, 5] {
            res.push_back(l);
        }

        assert_eq!(res, sum_lists_in_reverse(&l1, &l2));
    }
}
