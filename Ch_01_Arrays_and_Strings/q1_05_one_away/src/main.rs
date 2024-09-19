use q1_05_one_away::solution::one_away;

fn main() {
    println!(
        "Is \"CCI\" one away from \"CtCI\"?: {}",
        one_away("CtCI", "CCI")
    );
}
