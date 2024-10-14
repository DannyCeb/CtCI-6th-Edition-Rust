pub mod solution {

    use std::{cell::RefCell, rc::Rc};

    #[derive(Debug, Clone)]
    pub struct Task {
        pub value: char,
        pub dependency: Vec<Rc<RefCell<Task>>>,
    }

    impl Task {
        pub fn new(value: char) -> Self {
            Self {
                value,
                dependency: Vec::new(),
            }
        }
    }

    pub fn set_dependencies(
        tasks: &mut Vec<Rc<RefCell<Task>>>,
        dependency_values: &Vec<(Rc<RefCell<Task>>, Rc<RefCell<Task>>)>,
    ) {
        for dep in dependency_values {
            for t in tasks.into_iter() {
                if t.clone().as_ref().borrow().value == dep.1.clone().as_ref().borrow().value {
                    t.as_ref().borrow_mut().dependency.push(dep.0.clone());
                }
            }
        }
    }

    pub struct TaskGraph {
        pub nodes: Vec<Rc<RefCell<Task>>>,
    }

    impl TaskGraph {
        pub fn new(nodes: Vec<Rc<RefCell<Task>>>) -> Self {
            Self { nodes }
        }

        fn check_dep_status(&self) -> bool {
            for l in &self.nodes {
                if !l.as_ref().borrow().dependency.is_empty() {
                    return true;
                }
            }

            false
        }

        fn validate_dependencies(&mut self) -> Vec<char> {
            let mut res: Vec<char> = vec![];

            let mut length = self.nodes.len();

            let mut counter = 0;

            while counter < length {
                let item = self.nodes[counter].clone();
                if item.as_ref().borrow().dependency.is_empty() {
                    res.push(item.as_ref().borrow().value);
                    self.nodes.remove(counter);
                    length -= 1;
                } else {
                    counter += 1;
                }
            }

            res
        }

        fn remove_dependencies(&self, dependencies: &Vec<char>) {
            for node in self.nodes.clone().iter() {
                let mut counter: usize = 0;
                let mut length = { node.as_ref().borrow().dependency.len() };
                while counter < length {
                    let item: char;
                    {
                        let item_ = &node.as_ref().borrow();
                        let item_ = item_.dependency.get(counter);
                        item = item_.unwrap().as_ref().borrow().value;
                    }
                    if dependencies.contains(&item) {
                        node.borrow_mut().dependency.remove(counter);
                        length -= 1;
                    } else {
                        counter += 1;
                    }
                }
            }
        }

        pub fn build(&mut self) -> Vec<char> {
            let mut res: Vec<char> = vec![];

            while self.check_dep_status() {
                res.append(&mut self.validate_dependencies());

                self.remove_dependencies(&res);
            }

            res.append(&mut self.validate_dependencies());

            res
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::q4_07_build_order::solution::TaskGraph;

    use super::solution::{set_dependencies, Task};

    #[test]
    fn test_sort_tasks() {
        let mut v_tasks: Vec<Rc<RefCell<Task>>> = Vec::new();

        for l in 'a'..='f' {
            v_tasks.push(Rc::new(RefCell::new(Task::new(l))));
        }

        let dependencies = vec![
            (v_tasks[0].clone(), v_tasks[3].clone()),
            (v_tasks[5].clone(), v_tasks[1].clone()),
            (v_tasks[1].clone(), v_tasks[3].clone()),
            (v_tasks[5].clone(), v_tasks[0].clone()),
            (v_tasks[3].clone(), v_tasks[2].clone()),
        ];

        set_dependencies(&mut v_tasks, &dependencies);

        for (index, value) in v_tasks.clone().iter().enumerate() {
            print!(
                "[{}]: {} depends of: ",
                index,
                value.clone().as_ref().borrow().value
            );

            for l in value.clone().borrow().dependency.clone() {
                print!(" {} ", l.as_ref().borrow().value);
            }

            println!("");
        }

        let mut g_t = TaskGraph::new(v_tasks);

        assert_eq!(vec!['e', 'f', 'a', 'b', 'd', 'c'], g_t.build());
    }
}
