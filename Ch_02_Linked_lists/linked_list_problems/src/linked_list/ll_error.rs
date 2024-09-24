#[derive(Debug)]
pub enum LinkedListError {
    NotFound,
    EmptyList,
}

impl PartialEq for LinkedListError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LinkedListError::NotFound, LinkedListError::NotFound)
            | (LinkedListError::EmptyList, LinkedListError::EmptyList) => true,
            (_, _) => false,
        }
    }
}
