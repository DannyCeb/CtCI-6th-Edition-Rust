pub mod solution {
    use std::{cell::RefCell, rc::Rc};

    use crate::structures::graph::{Node, State};

    pub fn search(start: Rc<RefCell<Node>>, end: Rc<RefCell<Node>>) -> bool {
        if std::ptr::eq(start.as_ptr(), end.as_ptr()) {
            true
        } else {
            let mut node_queue: Vec<Rc<RefCell<Node>>> = Vec::new();

            node_queue.push(start.clone());

            while !node_queue.is_empty() {
                let actual_node = node_queue.remove(0);

                if actual_node.clone().as_ref().borrow().state == State::Visited {
                    continue;
                } else {
                    actual_node.as_ref().borrow_mut().state = State::Visited;

                    if std::ptr::eq(actual_node.as_ptr(), end.as_ptr()) {
                        return true;
                    } else {
                        node_queue.append(&mut actual_node.clone().as_ref().borrow().nodes.clone());
                    }
                }
            }

            false
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{q4_01_route_between_nodes::solution::search, structures::graph::Node};

    #[test]
    fn test_true() {
        let node_1 = Rc::new(RefCell::new(Node::new(1, Vec::new())));
        let node_2 = Rc::new(RefCell::new(Node::new(2, Vec::new())));
        let node_3 = Rc::new(RefCell::new(Node::new(3, Vec::new())));
        let node_4 = Rc::new(RefCell::new(Node::new(4, Vec::new())));
        let node_5 = Rc::new(RefCell::new(Node::new(5, Vec::new())));
        let node_6 = Rc::new(RefCell::new(Node::new(6, Vec::new())));
        let node_7 = Rc::new(RefCell::new(Node::new(7, Vec::new())));
        let node_8 = Rc::new(RefCell::new(Node::new(8, Vec::new())));

        node_1.as_ref().borrow_mut().nodes.push(node_2.clone());
        node_1.as_ref().borrow_mut().nodes.push(node_5.clone());

        node_2.as_ref().borrow_mut().nodes.push(node_4.clone());

        node_3.as_ref().borrow_mut().nodes.push(node_1.clone());
        node_3.as_ref().borrow_mut().nodes.push(node_7.clone());

        node_4.as_ref().borrow_mut().nodes.push(node_3.clone());
        node_4.as_ref().borrow_mut().nodes.push(node_6.clone());

        node_5.as_ref().borrow_mut().nodes.push(node_3.clone());
        node_5.as_ref().borrow_mut().nodes.push(node_6.clone());

        node_6.as_ref().borrow_mut().nodes.push(node_5.clone());

        node_8.as_ref().borrow_mut().nodes.push(node_7.clone());

        assert_eq!(true, search(node_1.clone(), node_7.clone()));
    }
}
