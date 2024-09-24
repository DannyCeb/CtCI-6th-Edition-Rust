use linked_list_problems::{
    linked_list::structures::MyLinkedList, q2_04_partition::solution::partition,
};

fn main() {
    let mut my_linked_list: MyLinkedList<i32> = MyLinkedList::new();

    for l in [3, 5, 8, 5, 10, 2, 1] {
        my_linked_list.push_back(l);
    }

    println!("LL: {}", &my_linked_list);

    let res = partition(&mut my_linked_list, 5);

    println!("result: {}", res);
}
