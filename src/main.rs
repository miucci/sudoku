use std::{fmt::Display, io::Write, thread};

#[derive(Debug)]
struct Board {
    b: Vec<Vec<Option<u8>>>,
}

struct RowIter<'a> {
    b: &'a Board,
    next_line: usize,
}

impl<'a> Iterator for RowIter<'a> {
    type Item = Vec<Option<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_line != 9 {
            self.next_line += 1;
            Some(self.b.b[self.next_line - 1].clone())
        } else {
            None
        }
    }
}

struct ColumnIter<'a> {
    b: &'a Board,
    next_column: usize,
}

impl<'a> Iterator for ColumnIter<'a> {
    type Item = Vec<Option<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_column != 9 {
            let mut res = Vec::with_capacity(9);
            for i in 0..9 {
                res.push(self.b.b[i][self.next_column]);
            }
            self.next_column += 1;
            Some(res)
        } else {
            None
        }
    }
}

struct SubsquareIter<'a> {
    b: &'a Board,
    next_subsquare: usize,
}

impl<'a> Iterator for SubsquareIter<'a> {
    type Item = Vec<Option<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_subsquare != 9 {
            let mut res = Vec::with_capacity(9);
            let mut row = (self.next_subsquare / 3) * 3;
            let column = (self.next_subsquare % 3) * 3;
            for _ in 0..3 {
                for i in 0..3 {
                    res.push(self.b.b[row][column + i]);
                }
                row += 1;
            }
            self.next_subsquare += 1;
            Some(res)
        } else {
            None
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for (i, row) in self.rows().enumerate() {
            if i % 3 == 0 {
                s.push_str("-------------\n");
            }
            for (i, c) in row.iter().enumerate() {
                if i % 3 == 0 {
                    s.push('|');
                }

                s.push_str(&match c {
                    None => " ".to_string(),
                    Some(val) => val.to_string(),
                });
            }
            s.push_str("|\n");
        }
        s.push_str("-------------\n");
        write!(f, "{s}")
    }
}

impl Board {
    fn rows(&self) -> RowIter {
        RowIter {
            b: self,
            next_line: 0,
        }
    }

    fn columns(&self) -> ColumnIter {
        ColumnIter {
            b: self,
            next_column: 0,
        }
    }

    fn subsquares(&self) -> SubsquareIter {
        SubsquareIter {
            b: self,
            next_subsquare: 0,
        }
    }

    fn is_valid(&self) -> bool {
        for row in self.rows() {
            let mut found: [bool; 10] = [false; 10];
            for val in row.into_iter().flatten() {
                if found[val as usize] {
                    return false;
                }
                found[val as usize] = true;
            }
        }

        for column in self.columns() {
            let mut found: [bool; 10] = [false; 10];
            for val in column.into_iter().flatten() {
                if found[val as usize] {
                    return false;
                }
                found[val as usize] = true;
            }
        }

        for subsquare in self.subsquares() {
            let mut found: [bool; 10] = [false; 10];
            for val in subsquare.into_iter().flatten() {
                if found[val as usize] {
                    return false;
                }
                found[val as usize] = true;
            }
        }
        true
    }

    fn solve_recursive(&mut self, row: usize, column: usize) -> bool {
        if row > 8 || column > 8 {
            return true;
        }
        let (next_row, next_column) = if column == 8 {
            (row + 1, 0)
        } else {
            (row, column + 1)
        };

        match self.b[row][column] {
            None => {
                for i in 1..=9 {
                    self.b[row][column] = Some(i);
                    if self.is_valid() && self.solve_recursive(next_row, next_column) {
                        return true;
                    }
                }
                self.b[row][column] = None;
            }
            _ => {
                return self.solve_recursive(next_row, next_column);
            }
        }
        false
    }

    fn solve(&mut self) {
        _ = self.solve_recursive(0, 0);
    }

    fn from_str(src: &str) -> Option<Board> {
        if src.len() != 81 {
            return None;
        }
        let mut s = src;
        let mut sudoku = Board {
            b: Vec::with_capacity(9),
        };
        for _ in 0..9 {
            let (row, rest) = s.split_at(9);
            sudoku.b.push(
                row.chars()
                    .map(|c| match c {
                        '.' => None,
                        num => Some(num.to_digit(10)? as u8),
                    })
                    .collect(),
            );
            s = rest;
        }
        Some(sudoku)
    }

    fn to_format_string(&self) -> String {
        let mut res = String::new();
        for row in self.rows() {
            for square in row {
                res.push(match square {
                    Some(i) => (i + 48) as char,
                    None => '.',
                })
            }
        }
        res
    }
}

fn main() {
    let data = match std::fs::read_to_string("sudoku.txt") {
        Ok(str) => str,
        Err(e) => {
            eprintln!("ERROR: {e}");
            std::process::exit(1);
        }
    };
    let mut handles = Vec::new();
    let lines: Vec<String> = data.lines().map(|l| l.to_string()).collect();
    for thread_data in lines.chunks(lines.len() / 20).map(|c| c.to_vec()) {
        handles.push(thread::spawn(move || {
            let mut res = Vec::new();
            for line in thread_data {
                let mut sudoku = match Board::from_str(&line) {
                    Some(s) => s,
                    None => {
                        eprintln!("ERROR: invalid sudoku");
                        std::process::exit(1);
                    }
                };
                sudoku.solve();
                assert!(sudoku.is_valid());
                res.push(sudoku.to_format_string());
            }
            res
        }));
    }
    let mut thread_results = Vec::new();
    for handle in handles {
        thread_results.append(&mut handle.join().unwrap());
    }
    match std::fs::File::options()
        .write(true)
        .create(true)
        .open("results.txt")
    {
        Ok(mut f) => {
            if f.write_all(thread_results.join("\n").as_bytes()).is_err() {
                eprintln!("ERROR: failed writing results to file");
                std::process::exit(1);
            }
        }
        Err(_) => {
            eprintln!("ERROR: failed creating result file");
            std::process::exit(1);
        }
    }
}
