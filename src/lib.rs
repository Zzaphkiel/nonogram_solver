#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Unknown,
    Empty,
    Full,
}

#[derive(Debug, Clone)]
struct CellLine {
    cells: Vec<Cell>,
}

#[derive(Debug, Clone)]
struct BoolLine {
    cells: Vec<bool>,
}

struct Nonogram<'a> {
    rows: Vec<CellLine>,
    columns: Vec<CellLine>,

    row_limits: &'a Vec<Vec<u32>>,
    col_limits: &'a Vec<Vec<u32>>,
}

#[derive(Debug, Clone)]
struct DataSet {
    lines: Vec<BoolLine>,
}

impl CellLine {
    fn new(len: usize) -> Self {
        Self {
            cells: vec![Cell::Unknown; len],
        }
    }

    fn update(&mut self, data_set: &DataSet) {
        let len = self.len();

        let mut empties = vec![true; len];
        let mut fulls = vec![true; len];

        for line in data_set.iter() {
            for (i, &b) in line.iter().enumerate() {
                match b {
                    true => empties[i] = false,
                    false => fulls[i] = false,
                };
            }
        }

        for (i, (&e, &f)) in empties.iter().zip(fulls.iter()).enumerate() {
            match (e, f) {
                (true, false) => self[i] = Cell::Empty,
                (false, true) => self[i] = Cell::Full,
                _ => (),
            }
        }
    }
}

use std::fmt;
impl fmt::Display for CellLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, &cell) in self.iter().enumerate() {
            let ch = match cell {
                Cell::Empty => '☒',
                Cell::Full => '■',
                Cell::Unknown => '□',
            };

            write!(f, "{} ", ch)?;

            if (i + 1) % 5 == 0 {
                write!(f, "  ")?;
            }
        }

        write!(f, " ")
    }
}

use std::ops;
impl ops::Deref for CellLine {
    type Target = Vec<Cell>;

    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

impl ops::DerefMut for CellLine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cells
    }
}

impl BoolLine {
    fn new(len: usize) -> Self {
        Self {
            cells: vec![false; len],
        }
    }

    fn check(&self, line: &CellLine) -> bool {
        assert_eq!(self.len(), line.len());

        for (&b, &c) in self.iter().zip(line.iter()) {
            match c {
                Cell::Full if !b => return false,
                Cell::Empty if b => return false,
                _ => (),
            }
        }

        true
    }
}

impl ops::Deref for BoolLine {
    type Target = Vec<bool>;

    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

impl ops::DerefMut for BoolLine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cells
    }
}

impl<'a> Nonogram<'a> {
    pub fn new(row_limits: &'a Vec<Vec<u32>>, col_limits: &'a Vec<Vec<u32>>) -> Self {
        let height = row_limits.len();
        let width = col_limits.len();

        Self {
            rows: vec![CellLine::new(width); height],
            columns: vec![CellLine::new(height); width],
            row_limits,
            col_limits,
        }
    }

    fn finished(&self) -> bool {
        for (line, limit) in self.columns.iter().zip(self.col_limits.iter()) {
            let true_count = line.iter().filter(|&c| *c == Cell::Full).count();
            let finished_true_count = limit.iter().sum::<u32>() as usize;

            if true_count != finished_true_count {
                return false;
            }
        }

        true
    }

    // fn solve(&mut self) {
    //     if self.finished() {
    //         return;
    //     }

    //     let (mut row_data_set, mut col_data_set) = self.generate_data_set();

    //     loop {
    //         for (i, (row, data_set)) in
    //             self.rows.iter_mut().zip(row_data_set.iter()).enumerate()
    //         {
    //             row.update(data_set);

    //             for (j, &cell) in row.iter().enumerate() {
    //                 self.columns[j][i] = cell;
    //             }
    //         }

    //         if self.finished() {
    //             break;
    //         }

    //         for (data_set, col) in col_data_set.iter_mut().zip(self.columns.iter()) {
    //             data_set.update(col);
    //         }

    //         for (i, (col, data_set)) in
    //             self.columns.iter_mut().zip(col_data_set.iter()).enumerate()
    //         {
    //             col.update(data_set);

    //             for (j, &cell) in col.iter().enumerate() {
    //                 self.rows[j][i] = cell;
    //             }
    //         }

    //         if self.finished() {
    //             break;
    //         }

    //         for (data_set, row) in row_data_set.iter_mut().zip(self.rows.iter()) {
    //             data_set.update(row);
    //         }
    //     }
    // }

    pub fn solve_and_print(&mut self) {
        if self.finished() {
            return;
        }

        let mut iteration = 0;

        use std::time::Instant;
        let begin = Instant::now();
        let mut now = Instant::now();

        let mut time_spent = Vec::new();
        let (mut row_data_set, mut col_data_set) = self.generate_data_set();

        time_spent.push(now.elapsed().as_secs_f64());
        now = Instant::now();

        loop {
            for (i, (row, data_set)) in
                self.rows.iter_mut().zip(row_data_set.iter()).enumerate()
            {
                row.update(data_set);

                for (j, &cell) in row.iter().enumerate() {
                    self.columns[j][i] = cell;
                }
            }

            println!("iteration {}", iteration);
            println!("{}", self);
            iteration += 1;

            time_spent.push(now.elapsed().as_secs_f64());
            now = Instant::now();

            if self.finished() {
                break;
            }

            for (data_set, col) in col_data_set.iter_mut().zip(self.columns.iter()) {
                data_set.update(col);
            }

            for (i, (col, data_set)) in
                self.columns.iter_mut().zip(col_data_set.iter()).enumerate()
            {
                col.update(data_set);

                for (j, &cell) in col.iter().enumerate() {
                    self.rows[j][i] = cell;
                }
            }

            println!("iteration {}", iteration);
            println!("{}", self);
            iteration += 1;

            time_spent.push(now.elapsed().as_secs_f64());
            now = Instant::now();

            if self.finished() {
                break;
            }

            for (data_set, row) in row_data_set.iter_mut().zip(self.rows.iter()) {
                data_set.update(row);
            }
        }

        println!("time spent:");
        for (i, time) in time_spent.iter().enumerate() {
            println!("iteration: {:3}, time spent: {:2.6}sec", i, time);
        }

        println!("----------------------------------------");
        println!(
            "total time spent:           {:2.6}sec",
            begin.elapsed().as_secs_f64()
        );
    }

    fn generate_data_set(&self) -> (Vec<DataSet>, Vec<DataSet>) {
        let height = self.rows.len();
        let width = self.columns.len();

        (
            self.row_limits
                .iter()
                .map(|limit| DataSet::new(width, limit))
                .collect(),
            self.col_limits
                .iter()
                .map(|limit| DataSet::new(height, limit))
                .collect(),
        )
    }
}

impl<'a> fmt::Display for Nonogram<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, line) in self.rows.iter().enumerate() {
            writeln!(f, "{}", line)?;

            if (i + 1) % 5 == 0 {
                writeln!(f)?;
            }
        }

        write!(f, "\n")
    }
}

impl DataSet {
    fn new(len: usize, limit: &Vec<u32>) -> Self {
        let limit_count = limit.len();

        let bool_line = BoolLine::new(len);
        if limit_count == 1 && limit[0] == 0 {
            return Self {
                lines: vec![bool_line],
            };
        }

        let mut result = vec![];
        let mut frontier = vec![(bool_line, 0, 0)];

        while let Some((line, index, begin)) = frontier.pop() {
            if begin == len && index != limit_count {
                continue;
            }

            if index == limit_count {
                result.push(line);
                continue;
            }

            for i in begin..=len - limit[index] as usize {
                let mut new_line = line.clone();
                for j in 0..limit[index] as usize {
                    new_line[i + j] = true;
                }

                frontier.push((new_line, index + 1, limit[index] as usize + i + 1));
            }
        }

        Self { lines: result }
    }

    fn update(&mut self, cell_line: &CellLine) {
        if self.lines.len() == 1 {
            return;
        }

        self.lines = self
            .iter()
            .filter(|line| line.check(cell_line))
            .map(|line| line.clone())
            .collect();
    }
}

impl ops::Deref for DataSet {
    type Target = Vec<BoolLine>;

    fn deref(&self) -> &Self::Target {
        &self.lines
    }
}

impl ops::DerefMut for DataSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lines
    }
}

pub fn solve_nonogram(row_limits: &Vec<Vec<u32>>, col_limits: &Vec<Vec<u32>>) {
    Nonogram::new(row_limits, col_limits).solve_and_print();
}

use std::ops::Range;

pub fn solve_a_line(
    len: usize,
    empty: &Vec<Range<u32>>,
    full: &Vec<Range<u32>>,
    limit: &Vec<u32>,
) {
    let mut cell_line = CellLine::new(len);

    for range in empty.iter() {
        for i in range.start..range.end {
            cell_line[i as usize] = Cell::Empty;
        }
    }

    for range in full.iter() {
        for i in range.start..range.end {
            cell_line[i as usize] = Cell::Full;
        }
    }

    let mut data_set = DataSet::new(len, limit);

    data_set.update(&cell_line);
    cell_line.update(&data_set);

    println!("{}", cell_line);
}
