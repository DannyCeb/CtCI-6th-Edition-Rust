pub mod solution {
    use std::{
        cell::RefCell,
        rc::{Rc, Weak},
    };

    pub struct BinNodeLink {
        pub value: i32,
        pub left: Option<Rc<RefCell<BinNodeLink>>>,
        pub right: Option<Rc<RefCell<BinNodeLink>>>,
        pub father: Option<Weak<RefCell<BinNodeLink>>>,
    }

    impl BinNodeLink {
        pub fn new(
            value: i32,
            left: Option<Rc<RefCell<BinNodeLink>>>,
            right: Option<Rc<RefCell<BinNodeLink>>>,
            father: Option<Weak<RefCell<BinNodeLink>>>,
        ) -> Self {
            Self {
                value,
                left,
                right,
                father,
            }
        }
    }

    pub fn get_top_left(
        node: Option<Rc<RefCell<BinNodeLink>>>,
    ) -> Option<Rc<RefCell<BinNodeLink>>> {
        let mut res = node.clone();
        while let Some(n) = res.clone() {
            res = n.clone().as_ref().borrow().left.clone();
        }
        res
    }

    pub fn successor(node: Option<Rc<RefCell<BinNodeLink>>>) -> Option<Rc<RefCell<BinNodeLink>>> {
        if let Some(n) = node.clone() {
            match n.clone().as_ref().borrow().right.clone() {
                Some(n_r) => get_top_left(Some(n_r.clone())),
                None => {
                    let father_left = n
                        .clone()
                        .as_ref()
                        .borrow()
                        .father
                        .as_ref()
                        .unwrap()
                        .upgrade()
                        .unwrap()
                        .as_ref()
                        .borrow()
                        .left
                        .clone();

                    if std::ptr::eq(n.as_ptr(), father_left.unwrap().as_ptr()) {
                        n.clone()
                            .as_ref()
                            .borrow()
                            .father
                            .clone()
                            .unwrap()
                            .upgrade()
                    } else {
                        let grand_father = n
                            .clone()
                            .as_ref()
                            .borrow()
                            .father
                            .as_ref()
                            .unwrap()
                            .upgrade()
                            .unwrap()
                            .as_ref()
                            .borrow()
                            .father
                            .as_ref()
                            .unwrap()
                            .upgrade();
                        successor(grand_father)
                    }
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::q4_06_successor::solution::successor;

    use super::solution::BinNodeLink;

    #[test]
    fn test_successor() {
        let left_child = Rc::new(RefCell::new(BinNodeLink::new(2, None, None, None)));
        let root = Rc::new(RefCell::new(BinNodeLink::new(
            1,
            Some(left_child.clone()),
            None,
            None,
        )));

        left_child.clone().as_ref().borrow_mut().father = Some(Rc::downgrade(&root.clone()));

        assert_eq!(
            1,
            successor(Some(left_child.clone()))
                .clone()
                .unwrap()
                .as_ref()
                .borrow()
                .value
        );
    }
}
