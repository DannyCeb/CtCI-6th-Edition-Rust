pub mod structs;

pub trait NodeItemTraits: Copy + std::fmt::Display + Default + Clone {}
impl NodeItemTraits for i32 {}
impl NodeItemTraits for char {}
