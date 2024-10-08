pub mod structs;

pub mod q3_01_three_in_one;
pub mod q3_02_stack_min;
pub mod q3_03_stack_of_plates;
pub mod q3_04_queue_via_stacks;
pub mod q3_05_sort_stack;
pub mod q3_06_animal_shelter;

pub trait NodeItemTraits: Copy + Clone {}
impl NodeItemTraits for i32 {}
impl NodeItemTraits for char {}
