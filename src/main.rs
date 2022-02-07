use nonogram_solver::solve_nonogram;

fn main() {
    let row_limits = vec![
        vec![3, 14, 3],
        vec![2, 15, 2],
        vec![2, 15, 1],
        vec![1, 18],
        vec![19],
        //
        vec![20],
        vec![20],
        vec![21],
        vec![5, 3, 7],
        vec![5, 2, 6],
        //
        vec![5, 2, 5],
        vec![5, 2, 7],
        vec![5, 2, 7],
        vec![6, 2, 7],
        vec![6, 2, 6],
        //
        vec![6, 3, 6],
        vec![5, 5, 6],
        vec![5, 7, 6],
        vec![5, 9, 5],
        vec![4, 3, 5, 5],
        //
        vec![4, 3, 6, 6],
        vec![4, 3, 14],
        vec![4, 3, 14],
        vec![12, 14],
        vec![29],
        //
        vec![27],
        vec![12, 9],
        vec![9, 7],
        vec![6, 7],
        vec![5, 6],
        //
        vec![5, 6],
        vec![4, 6],
        vec![3, 6],
        vec![3, 5],
        vec![3, 6],
        //
        vec![3, 6],
        vec![3, 6],
        vec![4, 5],
        vec![4, 5],
        vec![4, 6],
        //
        vec![4, 6],
        vec![5, 5],
        vec![4, 5, 1],
        vec![4, 5, 1],
        vec![4, 5, 1],
        //
        vec![4, 5, 1],
        vec![1, 5, 4, 2],
        vec![1, 11, 3],
        vec![2, 10, 3],
        vec![3, 8, 4],
    ];

    let column_limits = vec![
        vec![4, 4],
        vec![3, 2],
        vec![1, 8, 1],
        vec![11],
        vec![15],
        //
        vec![18],
        vec![10, 6],
        vec![9, 6],
        vec![11, 8],
        vec![11, 19],
        //
        vec![10, 22],
        vec![9, 26],
        vec![8, 12, 12],
        vec![8, 10, 9],
        vec![8, 4, 4, 5],
        //
        vec![8, 4, 3, 4],
        vec![19, 2, 3],
        vec![19, 2, 3],
        vec![9, 5, 2, 3],
        vec![8, 5, 3, 6],
        //
        vec![8, 10, 11],
        vec![8, 11, 15],
        vec![9, 29],
        vec![9, 26],
        vec![7, 24],
        //
        vec![9, 20],
        vec![10, 16],
        vec![10, 12],
        vec![8, 7],
        vec![17],
        //
        vec![15],
        vec![14, 1],
        vec![1, 13, 3],
        vec![2, 8, 4],
        vec![3, 2, 8],
    ];

    solve_nonogram(&row_limits, &column_limits);
}
