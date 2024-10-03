pub mod solution {
    use std::{cell::RefCell, rc::Rc};

    #[derive(Debug)]
    pub struct BinNode {
        pub value: i32,
        pub left: Option<Rc<RefCell<BinNode>>>,
        pub right: Option<Rc<RefCell<BinNode>>>,
    }

    impl BinNode {
        pub fn new(
            value: i32,
            left: Option<Rc<RefCell<BinNode>>>,
            right: Option<Rc<RefCell<BinNode>>>,
        ) -> Self {
            Self { value, left, right }
        }
    }

    pub fn create_binary_search_tree(numbers: &[i32]) -> Option<Rc<RefCell<BinNode>>> {
        let pivot = numbers.len() / 2;
        match pivot {
            0 => Some(Rc::new(RefCell::new(BinNode::new(
                numbers[pivot],
                None,
                None,
            )))),
            _ => Some(Rc::new(RefCell::new(BinNode::new(
                numbers[pivot],
                create_binary_search_tree(&numbers[..pivot]),
                create_binary_search_tree(&numbers[pivot..]),
            )))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::solution::create_binary_search_tree;

    #[test]
    fn test_tree_creation() {
        let numbers = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let root = create_binary_search_tree(numbers);

        println!("{:?}", root.unwrap().borrow());
    }
}
