use linked_list_problems::linked_list::structures::MyLinkedList;

fn main() {
    let mut l1: MyLinkedList<i32> = MyLinkedList::new();

    for l in [1, 2, 3] {
        l1.push_back(l);
    }

    l1.push_front(0);
    l1.push_front(-1);
    l1.push_front(-2);

    let mut v = vec![];

    for l in l1.clone().into_iter() {
        v.push(format!("{:p}", l));
    }

    println!("{:?}", v);
}
