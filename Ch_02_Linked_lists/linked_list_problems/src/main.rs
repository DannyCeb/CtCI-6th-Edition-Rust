use linked_list_problems::{
    linked_list::structures::MyLinkedList, q2_04_partition::solution::partition,
    q2_05_sum_lists::solution::sum_lists_in_reverse,
};

fn main() {
    let mut l1: MyLinkedList<i32> = MyLinkedList::new();

    for l in [4, 2, 7, 8, 9, 1, 2, 3, 5, 0, 9, 1, 3, 7, 2, 6, 5, 9, 1, 3] {
        l1.push_back(l);
    }

    let mut l2 = MyLinkedList::<i32>::new();

    for l in [8, 4, 5, 1, 2, 3, 7, 9, 2, 1, 3, 5, 0, 9, 8, 2, 4, 6, 1, 2] {
        l2.push_back(l);
    }

    let mut res = MyLinkedList::<i32>::new();

    for l in [2, 7, 2, 0, 2, 5, 9, 2, 8, 1, 2, 7, 3, 7, 1, 9, 9, 5, 3, 5] {
        res.push_back(l);
    }

    assert_eq!(res, sum_lists_in_reverse(&l1, &l2));
}
