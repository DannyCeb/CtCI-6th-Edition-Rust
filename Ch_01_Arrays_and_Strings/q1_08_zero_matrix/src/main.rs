use q1_08_zero_matrix::solution::zero_matrix;

fn main() {
    let mut matrix = vec![vec![1, 2, 0, 4], vec![5, 6, 7, 8]];

    zero_matrix(&mut matrix);

    println!("{:?}", matrix);
}
