pub mod structs;

pub mod q3_01_three_in_one;
pub mod q3_02_stack_min;
pub mod q3_03_stack_of_plates;

pub trait NodeItemTraits: Copy + std::fmt::Display + Default + Clone {}
impl NodeItemTraits for i32 {}
impl NodeItemTraits for char {}
