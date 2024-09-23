pub mod solution {

    pub struct Position {
        row: usize,
        column: usize,
    }

    impl Position {
        fn new(row: usize, column: usize) -> Self {
            Self { row, column }
        }
    }

    // set zeros on the matrix
    pub fn set_zeros(matrix: &mut Vec<Vec<i32>>, pos: Position) {
        matrix[pos.row] = matrix[pos.row].iter().map(|_| 0).collect();

        for l in 0..matrix.len() {
            matrix[l][pos.column] = 0;
        }
    }

    pub fn zero_matrix(matrix: &mut Vec<Vec<i32>>) {
        let mut vec_positions: Vec<Position> = Vec::new();

        // find zeros
        for l in 0..matrix.len() {
            for k in 0..matrix[0].len() {
                if matrix[l][k] == 0 {
                    vec_positions.push(Position::new(l, k));
                }
            }
        }

        // fill zeros
        for l in vec_positions {
            set_zeros(matrix, l);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::zero_matrix;

    #[test]
    fn test_4x4_matrix() {
        let mut matrix = vec![
            vec![1, 2, 0, 4],
            vec![5, 6, 7, 8],
            vec![9, 0, 11, 12],
            vec![13, 14, 15, 16],
        ];

        zero_matrix(&mut matrix);

        assert_eq!(
            vec![
                vec![0, 0, 0, 0],
                vec![5, 0, 0, 8],
                vec![0, 0, 0, 0],
                vec![13, 0, 0, 16]
            ],
            matrix
        );
    }

    #[test]
    fn test_2x4() {
        let mut matrix = vec![vec![1, 2, 0, 4], vec![5, 6, 7, 8]];

        zero_matrix(&mut matrix);

        assert_eq!(vec![vec![0, 0, 0, 0], vec![5, 6, 0, 8],], matrix);
    }
}
