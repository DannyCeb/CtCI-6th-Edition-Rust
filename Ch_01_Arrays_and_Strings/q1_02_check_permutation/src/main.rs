use q1_02_check_permutation::solution::check_permutation;

fn main() {
    println!(
        "\"cracking\" and \"rckcagin\" are permutations? r: {}",
        check_permutation("cracking".to_string(), "rckcagin".to_string())
    );
}
