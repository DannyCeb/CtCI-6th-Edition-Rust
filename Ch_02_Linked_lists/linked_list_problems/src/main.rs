use linked_list_problems::{
    linked_list::structures::MyLinkedList, q2_04_partition::solution::partition,
    q2_05_sum_lists::solution::sum_lists_in_reverse,
};

fn main() {
    let mut my_linked_list: MyLinkedList<i32> = MyLinkedList::new();

    for l in [7, 1, 6] {
        my_linked_list.push_back(l);
    }

    let mut l2 = MyLinkedList::<i32>::new();

    for l in [5, 9, 9] {
        l2.push_back(l);
    }

    println!("Sum: {}", sum_lists_in_reverse(&my_linked_list, &l2));
}
