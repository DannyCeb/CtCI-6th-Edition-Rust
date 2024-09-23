use linked_list_problems::linked_list::structures::MyLinkedList;

fn main() {
    let mut my_linked_list: MyLinkedList<i32> = MyLinkedList::new();
    println!("LL: {}", &my_linked_list);

    my_linked_list.push_back(0);
    my_linked_list.push_back(1);
    my_linked_list.push_front(-1);
    my_linked_list.push_back(2);
    my_linked_list.push_front(0);
    my_linked_list.push_back(2);

    println!("LL: {}", &my_linked_list);

    let _ = my_linked_list.remove_dups();

    println!("LL: {}", &my_linked_list);
}
