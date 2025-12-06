#[derive(PartialEq, Copy, Clone, Debug)]
enum Operation {
    Add = 0,
    Multiply = 1,
}

impl Operation {
    fn from_string(s: &str) -> Operation {
        match s {
            "*" => Operation::Multiply,
            "+" => Operation::Add,
            _ => panic!("Unknown operation!"),
        }
    }

    fn neutral_element(&self) -> u64 {
        *self as u64
    }

    fn apply(&self, lhs: &mut u64, rhs: u64) {
        match self {
            Operation::Add => {
                *lhs = *lhs + rhs;
            }
            Operation::Multiply => {
                *lhs = *lhs * rhs;
            }
        }
    }
}

#[derive(Debug)]
struct DataColumn {
    operation: Operation,
    values: Vec<u64>,
}

impl DataColumn {
    fn calculate(&self) -> u64 {
        let mut total = self.operation.neutral_element();
        for val in &self.values {
            self.operation.apply(&mut total, *val);
        }
        total
    }
}

pub fn day6(lines: Vec<String>) {
    let p1 = part1(&lines);
    let p2 = part2(&lines);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(lines: &Vec<String>) -> u64 {
    let grid: Vec<Vec<&str>> = lines
        .iter()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    (0..grid[0].len())
        .map(|col_index| parse_data_column_p1(&grid, col_index))
        .map(|column| column.calculate())
        .sum()
}

fn part2(lines: &Vec<String>) -> u64 {
    get_text_column_boundaries(lines)
        .iter()
        .map(|(from, to)| parse_data_column_p2(lines, *from, *to))
        .map(|column| column.calculate())
        .sum()
}

fn parse_data_column_p1(grid: &Vec<Vec<&str>>, col_index: usize) -> DataColumn {
    let col_height = grid.len();

    let operation = Operation::from_string(grid[col_height - 1][col_index]);
    let values = grid
        .iter()
        .take(grid.len() - 1)
        .map(|item| item[col_index].parse::<u64>().unwrap())
        .collect();

    DataColumn { operation, values }
}

fn get_text_column_boundaries(lines: &Vec<String>) -> Vec<(usize, usize)> {
    let mut scan_pos = 0;
    let mut from = 0;
    let mut boundaries = Vec::new();
    let max_row_length = lines
        .iter()
        .max_by(|l1, l2| l1.len().cmp(&l2.len()))
        .unwrap()
        .len();

    // We need <= in order to catch the last column,
    // it will find that each row returns None for the character
    while scan_pos <= max_row_length {
        let is_text_column_empty = lines.iter().all(|l| {
            let c = l.chars().nth(scan_pos);
            c == None || c == Some(' ')
        });

        if is_text_column_empty {
            assert!(from != scan_pos, "Found column of size 0: {}", scan_pos);
            boundaries.push((from, scan_pos));
            from = scan_pos + 1;
        }

        scan_pos += 1;
    }

    boundaries
}

fn parse_data_column_p2(lines: &[String], from: usize, to: usize) -> DataColumn {
    let mut scan_pos = to - 1;
    let mut numbers: Vec<u64> = Vec::new();

    while scan_pos >= from {
        let mut s = String::new();
        for line_index in 0..lines.len() - 1 {
            let maybe_c = lines[line_index].chars().nth(scan_pos);
            if let Some(c) = maybe_c
                && c != ' '
            {
                s.push(c);
            }
        }
        numbers.push(s.parse().unwrap());
        if scan_pos == 0 {
            break;
        }
        scan_pos -= 1;
    }

    let last_line = lines.last().unwrap();
    DataColumn {
        operation: Operation::from_string(
            &last_line[from..std::cmp::min(to, last_line.len())].trim(),
        ),
        values: numbers,
    }
}
