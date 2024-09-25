use linked_list_problems::linked_list::structures::MyLinkedList;

fn main() {
    let mut l1: MyLinkedList<i32> = MyLinkedList::new();

    for l in [1, 2, 3] {
        l1.push_back(l);
    }

    let l2 = l1.clone();

    l1.push_front(0);
    l1.push_front(-1);
    l1.push_front(-2);

    println!("l1: {}\nl2: {}", &l1, &l2);

    for l in l1.clone().into_iter() {
        for k in l2.clone().into_iter() {
            println!("{}", std::ptr::eq(l.as_ptr(), k.as_ptr()));
        }
    }
}
