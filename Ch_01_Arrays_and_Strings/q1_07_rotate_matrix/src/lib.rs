pub mod solution {

    pub fn rotate_matrix(matrix: &mut Vec<Vec<i32>>) -> bool {
        if matrix.len() != matrix[0].len() {
            false
        } else {
            let n = matrix.len();

            for layer in 0..(n / 2) {
                for counter in 0..n - 1 - 2 * layer {
                    /*
                     * To understand the code consider the following:
                     *
                     *      upper left iterator:   matrix[layer][layer + counter]
                     *      upper right iterator:  matrix[layer + counter][n - layer - 1]
                     *      bottom left iterator:  matrix[n - layer - 1 - counter][layer]
                     *      bottom right position: matrix[n - layer - 1][n - layer - 1 - counter]
                     */
                    // saves the bottom left value
                    let aux = matrix[n - layer - 1 - counter][layer];

                    // moves the bottom right value to the bottom left position
                    matrix[n - layer - 1 - counter][layer] =
                        matrix[n - layer - 1][n - layer - 1 - counter];

                    // moves the upper right value to the bottom right position
                    matrix[n - layer - 1][n - layer - 1 - counter] =
                        matrix[layer + counter][n - layer - 1];

                    // moves the upper left value to the upper right position
                    matrix[layer + counter][n - layer - 1] = matrix[layer][layer + counter];

                    // sets the saved bottom left value to the upper left position
                    matrix[layer][layer + counter] = aux;
                }
            }

            true
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::rotate_matrix;

    #[test]
    fn test_rotate_2x2_matrix() {
        let mut m = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(true, rotate_matrix(&mut m));

        assert_eq!(vec![vec![3, 1], vec![4, 2]], m);
    }

    #[test]
    fn test_rotate_3x3_matrix() {
        let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(true, rotate_matrix(&mut m));

        assert_eq!(vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]], m);
    }

    #[test]
    fn test_rotate_10x10_matrix() {
        let mut m = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25, 26, 27, 28, 29, 30],
            vec![31, 32, 33, 34, 35, 36, 37, 38, 39, 40],
            vec![41, 42, 43, 44, 45, 46, 47, 48, 49, 50],
            vec![51, 52, 53, 54, 55, 56, 57, 58, 59, 60],
            vec![61, 62, 63, 64, 65, 66, 67, 68, 69, 70],
            vec![71, 72, 73, 74, 75, 76, 77, 78, 79, 80],
            vec![81, 82, 83, 84, 85, 86, 87, 88, 89, 90],
            vec![91, 92, 93, 94, 95, 96, 97, 98, 99, 100],
        ];
        assert_eq!(true, rotate_matrix(&mut m));

        assert_eq!(
            vec![
                vec![91, 81, 71, 61, 51, 41, 31, 21, 11, 1],
                vec![92, 82, 72, 62, 52, 42, 32, 22, 12, 2],
                vec![93, 83, 73, 63, 53, 43, 33, 23, 13, 3],
                vec![94, 84, 74, 64, 54, 44, 34, 24, 14, 4],
                vec![95, 85, 75, 65, 55, 45, 35, 25, 15, 5],
                vec![96, 86, 76, 66, 56, 46, 36, 26, 16, 6],
                vec![97, 87, 77, 67, 57, 47, 37, 27, 17, 7],
                vec![98, 88, 78, 68, 58, 48, 38, 28, 18, 8],
                vec![99, 89, 79, 69, 59, 49, 39, 29, 19, 9],
                vec![100, 90, 80, 70, 60, 50, 40, 30, 20, 10],
            ],
            m
        );
    }

    #[test]
    fn test_no_square() {
        let mut m = vec![vec![1, 2]];

        assert_eq!(false, rotate_matrix(&mut m));
    }
}
