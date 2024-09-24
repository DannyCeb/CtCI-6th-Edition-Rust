mod solution {

    use crate::linked_list::structures::MyLinkedList;

    pub fn return_kth_to_last<
        T: Copy + std::fmt::Display + std::hash::Hash + std::cmp::Eq + Default,
    >(
        list: &mut MyLinkedList<T>,
        k: usize,
    ) -> T {
        let mut counter: usize = 0;
        let mut res: T = T::default();
        for l in list.rev() {
            if counter == k {
                res = l.borrow().item;
                break;
            }
            counter += 1;
        }

        res
    }
}
