use q1_06_string_compression::solution::compress_string;

fn main() {
    println!(
        "aabcccccaaa compressed is: {}",
        compress_string(String::from("aabcccccaaa"))
    );
}
