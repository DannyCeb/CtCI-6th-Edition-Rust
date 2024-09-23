use linked_list_problems::linked_list::LinkedList;

fn main() {
    let mut my_linked_list: LinkedList<i32> = LinkedList::new();

    my_linked_list.push_back(0);
    my_linked_list.push_back(1);
    my_linked_list.push_front(-1);
    my_linked_list.push_back(2);

    println!("LL: {}", &my_linked_list);
}
