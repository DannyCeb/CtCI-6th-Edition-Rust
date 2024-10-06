pub mod solution {
    use std::{cell::RefCell, rc::Rc};

    use crate::q4_02_minimal_tree::solution::BinNode;

    pub fn is_balanced(node: Option<Rc<RefCell<BinNode>>>, result: &mut bool) -> u32 {
        match node.clone() {
            Some(n) => {
                let sum: u32 = match (
                    n.clone().as_ref().borrow().left.clone(),
                    n.clone().as_ref().borrow().right.clone(),
                ) {
                    (Some(l), Some(r)) => {
                        let d_r = is_balanced(Some(r.clone()), result);
                        let d_l = is_balanced(Some(l.clone()), result);

                        if d_r.abs_diff(d_l) > 1 {
                            *result = false;
                        }

                        std::cmp::max(d_r, d_l)
                    }
                    (Some(l), None) => {
                        let d_l = is_balanced(Some(l.clone()), result);

                        if d_l > 1 {
                            *result = false;
                        }

                        d_l
                    }
                    (None, Some(r)) => {
                        let d_r = is_balanced(Some(r.clone()), result);

                        if d_r > 1 {
                            *result = false;
                        }

                        d_r
                    }
                    (None, None) => 0,
                };

                sum + 1
            }
            None => {
                *result = true;
                0
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        q4_02_minimal_tree::solution::{create_binary_search_tree, BinNode},
        q4_04_check_balanced::solution::is_balanced,
    };

    #[test]
    fn test_is_balanced() {
        let numbers = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let root = create_binary_search_tree(numbers);

        let mut res = true;

        assert_eq!(4, is_balanced(root, &mut res));
        assert_eq!(true, res);
    }

    #[test]
    fn test_is_not_balanced() {
        let l4 = Some(Rc::new(RefCell::new(BinNode::new(1, None, None))));
        let l3 = Some(Rc::new(RefCell::new(BinNode::new(1, l4.clone(), None))));
        let l2 = Some(Rc::new(RefCell::new(BinNode::new(1, l3.clone(), None))));
        let l1 = Some(Rc::new(RefCell::new(BinNode::new(1, l2.clone(), None))));

        let r1 = Some(Rc::new(RefCell::new(BinNode::new(1, None, None))));

        let root = Some(Rc::new(RefCell::new(BinNode::new(
            1,
            l1.clone(),
            r1.clone(),
        ))));

        let mut res = true;

        is_balanced(root, &mut res);

        assert_eq!(false, res);
    }
}
