pub mod structs;

pub mod q3_01_three_in_one;
pub mod q3_02_stack_min;

pub trait NodeItemTraits: Copy + std::fmt::Display + Default + Clone {}
impl NodeItemTraits for i32 {}
impl NodeItemTraits for char {}
