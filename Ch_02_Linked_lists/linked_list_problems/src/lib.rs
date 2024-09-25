pub mod linked_list;
pub mod q2_01_remove_dups;
pub mod q2_02_return_kth_to_last;
pub mod q2_03_delete_middle_node;
pub mod q2_04_partition;
pub mod q2_05_palindrome;
pub mod q2_05_sum_lists;

pub trait NodeItemTraits:
    Copy + std::fmt::Display + std::hash::Hash + Eq + Default + std::cmp::PartialOrd
{
}
impl NodeItemTraits for i32 {}
impl NodeItemTraits for char {}
