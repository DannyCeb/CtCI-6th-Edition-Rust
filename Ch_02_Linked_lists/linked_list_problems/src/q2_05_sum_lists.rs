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
                res.pop_back();
            }
        }

        res
    }
}
