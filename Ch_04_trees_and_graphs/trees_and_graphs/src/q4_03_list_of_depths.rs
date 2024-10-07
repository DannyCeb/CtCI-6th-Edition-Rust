pub mod solution {
    use std::{cell::RefCell, rc::Rc};

    use crate::q4_02_minimal_tree::solution::BinNode;

    pub fn list_of_depths(
        node: Option<Rc<RefCell<BinNode>>>,
        depths: &mut Vec<Vec<i32>>,
        depth: usize,
    ) {
        match node.clone() {
            Some(n) => {
                if depth >= depths.len() {
                    depths.push(Vec::new());
                }
                depths[depth].push(n.as_ref().borrow().value);
                list_of_depths(n.borrow().left.clone(), depths, depth + 1);
                print!("{} ", n.as_ref().borrow().value);
                list_of_depths(n.borrow().right.clone(), depths, depth + 1);
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::q4_02_minimal_tree::solution::create_binary_search_tree;

    use super::solution::list_of_depths;

    #[test]
    fn test_list() {
        let numbers = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let root = create_binary_search_tree(numbers);

        let mut depths: Vec<Vec<i32>> = Vec::new();

        list_of_depths(root, &mut depths, 0);

        println!("\n{:?}", depths);
    }
}
