use q1_03_urlify::solution::urlify;

fn main() {
    let mut s = "Cracking the coding interview".to_string();
    println!("string before urlify: {}", &s);
    let s_ref = &mut s;
    println!("pointer to s: {:p}", s_ref);
    urlify(s_ref);
    println!("string after urlify: {}", *s_ref);
    println!("pointer to s: {:p}", s_ref);
}
