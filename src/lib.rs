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

pub struct Board {
    rows: Vec<CellLine>,
    columns: Vec<CellLine>,

    row_limits: Vec<Vec<u32>>,
    col_limits: Vec<Vec<u32>>,
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
        let len = self.cells.len();

        let mut empties = vec![true; len];
        let mut fulls = vec![true; len];

        for line in data_set.lines.iter() {
            for (i, &b) in line.cells.iter().enumerate() {
                match b {
                    true => empties[i] = false,
                    false => fulls[i] = false,
                };
            }
        }

        for (i, (&e, &f)) in empties.iter().zip(fulls.iter()).enumerate() {
            match (e, f) {
                (true, false) => self.cells[i] = Cell::Empty,
                (false, true) => self.cells[i] = Cell::Full,
                _ => (),
            }
        }
    }
}

use std::fmt;
impl fmt::Display for CellLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, &cell) in self.cells.iter().enumerate() {
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

impl BoolLine {
    fn new(len: usize) -> Self {
        Self {
            cells: vec![false; len],
        }
    }

    fn check(&self, line: &CellLine) -> bool {
        assert_eq!(self.cells.len(), line.cells.len());

        for (&b, &c) in self.cells.iter().zip(line.cells.iter()) {
            match c {
                Cell::Full if !b => return false,
                Cell::Empty if b => return false,
                _ => (),
            }
        }

        true
    }
}

impl Board {
    pub fn new(
        heigth: usize,
        width: usize,
        row_limits: Vec<Vec<u32>>,
        col_limits: Vec<Vec<u32>>,
    ) -> Self {
        Self {
            rows: vec![CellLine::new(width); heigth],
            columns: vec![CellLine::new(heigth); width],
            row_limits,
            col_limits,
        }
    }

    fn finished(&self) -> bool {
        for (line, limit) in self.columns.iter().zip(self.col_limits.iter()) {
            let true_count = line.cells.iter().filter(|&c| *c == Cell::Full).count();
            let finished_true_count = limit.iter().sum::<u32>() as usize;

            if true_count != finished_true_count {
                return false;
            }
        }

        true
    }

    pub fn solve(&mut self) {
        if self.finished() {
            return;
        }

        let (mut row_data_set, mut col_data_set) = self.generate_data_set();

        loop {
            for (i, (row, data_set)) in
                self.rows.iter_mut().zip(row_data_set.iter()).enumerate()
            {
                row.update(data_set);

                for (j, &cell) in row.cells.iter().enumerate() {
                    self.columns[j].cells[i] = cell;
                }
            }

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

                for (j, &cell) in col.cells.iter().enumerate() {
                    self.rows[j].cells[i] = cell;
                }
            }

            if self.finished() {
                break;
            }

            for (data_set, row) in row_data_set.iter_mut().zip(self.rows.iter()) {
                data_set.update(row);
            }
        }
    }

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

                for (j, &cell) in row.cells.iter().enumerate() {
                    self.columns[j].cells[i] = cell;
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

                for (j, &cell) in col.cells.iter().enumerate() {
                    self.rows[j].cells[i] = cell;
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

impl fmt::Display for Board {
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
                    new_line.cells[i + j] = true;
                }

                frontier.push((new_line, index + 1, limit[index] as usize + i + 1));
            }
        }

        Self { lines: result }
    }

    fn update(&mut self, cell_line: &CellLine) {
        self.lines = self
            .lines
            .iter()
            .filter(|line| line.check(cell_line))
            .map(|line| line.clone())
            .collect();
    }
}
