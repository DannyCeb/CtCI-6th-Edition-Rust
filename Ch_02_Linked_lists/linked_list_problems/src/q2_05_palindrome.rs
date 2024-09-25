mod solution {
    use crate::linked_list::structures::MyLinkedList;

    pub fn is_palindrome(list: &MyLinkedList<char>) -> bool {
        let mut front_iter = list.clone().into_iter();
        let mut back_iter = list.clone().rev().into_iter();

        loop {
            match (front_iter.next().clone(), back_iter.next().clone()) {
                (Some(char_1), Some(char_2)) => {
                    match char_1
                        .as_ref()
                        .borrow()
                        .item
                        .cmp(&char_2.as_ref().borrow().item)
                    {
                        std::cmp::Ordering::Equal => {
                            continue;
                        }
                        _ => {
                            return false;
                        }
                    }
                }
                (_, _) => break,
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::{linked_list::structures::MyLinkedList, q2_05_palindrome::solution::is_palindrome};

    #[test]
    fn test_is_palindrome() {
        let mut list = MyLinkedList::<char>::new();

        for l in "aaaoooiiioooaaa".chars() {
            list.push_back(l);
        }

        assert_eq!(true, is_palindrome(&list));
    }
}
