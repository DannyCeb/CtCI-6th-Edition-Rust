use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub enum State {
    Unvisited,
    Visited,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (State::Visited, State::Visited) | (State::Unvisited, State::Unvisited) => true,
            _ => false,
        }
    }
}

impl Eq for State {}

pub struct Node {
    pub value: i32,
    pub state: State,
    pub nodes: Vec<Rc<RefCell<Node>>>, // list of nodes that can be visited by the node
}

impl Node {
    pub fn new(value: i32, nodes: Vec<Rc<RefCell<Node>>>) -> Self {
        Self {
            value,
            nodes,
            state: State::Unvisited,
        }
    }
}

pub struct MyGraph {
    pub nodes: HashMap<Rc<RefCell<Node>>, Vec<Rc<RefCell<Node>>>>,
}

impl MyGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }
}
