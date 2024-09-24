pub mod solution {
    use crate::{linked_list::structures::MyLinkedList, NodeItemTraits};

    pub fn partition<T: NodeItemTraits>(list: &mut MyLinkedList<T>, value: T) -> MyLinkedList<T> {
        let mut prev_list: MyLinkedList<T> = list
            .into_iter()
            .filter_map(|v| {
                if v.as_ref().borrow().item < value {
                    Some(v.as_ref().borrow().item)
                } else {
                    None
                }
            })
            .collect();

        let following = list.into_iter().filter_map(|v| {
            if v.as_ref().borrow().item >= value {
                Some(v.as_ref().borrow().item)
            } else {
                None
            }
        });

        for l in following {
            prev_list.push_back(l);
        }

        prev_list
    }
}

#[cfg(test)]
mod tests {
    use crate::{linked_list::structures::MyLinkedList, q2_04_partition::solution::partition};

    #[test]
    fn test_partition() {
        let mut my_linked_list: MyLinkedList<i32> = MyLinkedList::new();

        for l in [3, 5, 8, 5, 10, 2, 1] {
            my_linked_list.push_back(l);
        }

        println!("list before partition: {}", &my_linked_list);

        let res = partition(&mut my_linked_list, 5);

        println!("result list after partition: {}", res);
    }
}
