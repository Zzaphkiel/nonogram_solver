use nonogram_solver::Board;

fn main() {
    let mut board = Board::new(
        // height
        25,
        // width
        25,
        // row_limits
        vec![
            vec![0],
            vec![4, 4],
            vec![5, 5],
            vec![9],
            vec![9],
            //
            vec![3, 11, 3],
            vec![6, 6],
            vec![4, 4],
            vec![5, 5],
            vec![5, 9, 5],
            //
            vec![6, 5, 6],
            vec![5, 1, 1, 1, 1, 5],
            vec![4, 1, 1, 1, 1, 4],
            vec![3, 3, 3, 3],
            vec![1, 3, 3, 1],
            //
            vec![3, 3],
            vec![2, 2],
            vec![1, 1],
            vec![2, 7, 2],
            vec![4, 3, 4],
            //
            vec![1, 1, 2, 1, 1],
            vec![1, 1],
            vec![2, 2],
            vec![4, 4],
            vec![7, 7],
        ],
        // col_limits
        vec![
            vec![0],
            vec![1],
            vec![1, 3, 1],
            vec![2, 4, 2],
            vec![13, 2, 1],
            //
            vec![8, 2, 1],
            vec![1, 8, 2, 2],
            vec![2, 4, 1, 2],
            vec![6, 2, 1, 3],
            vec![5, 1, 4, 1, 4],
            //
            vec![4, 2, 2, 1, 1],
            vec![4, 6, 1],
            vec![3, 2, 3],
            vec![4, 6, 3],
            vec![4, 2, 2, 2, 1],
            //
            vec![5, 1, 4, 1, 4],
            vec![6, 2, 1, 3],
            vec![2, 4, 1, 2],
            vec![1, 8, 2, 2],
            vec![8, 2, 1],
            //
            vec![13, 2, 1],
            vec![2, 4, 2],
            vec![1, 3, 1],
            vec![1],
            vec![0],
        ],
    );

    board.solve_and_print();
}
