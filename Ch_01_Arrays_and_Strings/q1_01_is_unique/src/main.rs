use q1_01_is_unique::solution::*;
fn main() {
    let s = String::from("this is not a unique string :(");

    println!("String: {} is unique: {}", &s, is_unique(&s))
}
