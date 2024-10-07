pub mod solution {
    use std::{borrow::Borrow, cell::RefCell, rc::Rc};

    use crate::q4_02_minimal_tree::solution::BinNode;

    pub fn validate_bst(node: Option<Rc<RefCell<BinNode>>>) -> bool {
        let node = node.clone().unwrap();
        let node_value = node.as_ref().borrow().borrow().value;
        match (
            node.clone().as_ref().borrow().left.clone(),
            node.clone().as_ref().borrow().right.clone(),
        ) {
            (Some(l), Some(r)) => {
                let l_val = l.as_ref().borrow().value;
                let r_val = r.as_ref().borrow().value;

                node_value >= l_val
                    && node_value <= r_val
                    && validate_bst(Some(l.clone()))
                    && validate_bst(Some(r.clone()))
            }
            (Some(l), None) => {
                let l_val = l.as_ref().borrow().value;
                node_value >= l_val && validate_bst(Some(l.clone()))
            }
            (None, Some(r)) => {
                let r_val = r.as_ref().borrow().value;
                node_value <= r_val && validate_bst(Some(r.clone()))
            }
            (None, None) => true,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        q4_02_minimal_tree::solution::create_binary_search_tree,
        q4_05_validate_bst::solution::validate_bst,
    };

    #[test]
    fn test_validate_bst() {
        let numbers = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let root = create_binary_search_tree(numbers);

        assert_eq!(true, validate_bst(root));

        let numbers = &[100, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let root = create_binary_search_tree(numbers);

        assert_eq!(false, validate_bst(root));
    }
}
