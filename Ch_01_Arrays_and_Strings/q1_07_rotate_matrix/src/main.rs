use q1_07_rotate_matrix::solution::rotate_matrix;

fn main() {
    let mut m = &mut vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];

    println!("Matrix before rotate: {:?} ", &m);
    rotate_matrix(&mut m);

    println!("Matrix after rotate: {:?}", m)
}
