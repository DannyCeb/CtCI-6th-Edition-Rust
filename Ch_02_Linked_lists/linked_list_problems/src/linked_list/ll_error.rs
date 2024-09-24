use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum LinkedListError {
    NotFound,
    EmptyList,
}

impl Display for LinkedListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinkedListError::EmptyList => write!(f, "The linked list is empty"),
            LinkedListError::NotFound => write!(f, "Item/Node not found"),
        }
    }
}

impl Error for LinkedListError {}

impl PartialEq for LinkedListError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LinkedListError::NotFound, LinkedListError::NotFound)
            | (LinkedListError::EmptyList, LinkedListError::EmptyList) => true,
            (_, _) => false,
        }
    }
}
