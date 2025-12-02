struct EqualSplit<'a> {
    text: &'a str,
    position: usize,
    size: usize,
}

impl<'a> EqualSplit<'a> {
    fn new(text: &'a str, size: usize) -> Self {
        EqualSplit {
            text,
            position: 0,
            size,
        }
    }
}

impl<'a> Iterator for EqualSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.text.len() {
            return None;
        }

        self.position += self.size;
        Some(&self.text[(self.position - self.size)..self.position])
    }
}

// ----------------------------------------------

pub fn day2(lines: Vec<String>) {
    let input = lines.join("");
    let ranges = parse_ranges(&input);

    let mut sum_p1 = 0;
    let mut sum_p2 = 0;
    for r in ranges {
        sum_p1 += find_invalid_in_range_p1(r);
        sum_p2 += find_invalid_in_range_p2(r);
    }

    println!("Part 1: {}", sum_p1);
    println!("Part 2: {}", sum_p2)
}

fn find_invalid_in_range_p1(r: (u64, u64)) -> u64 {
    let mut invalid = 0;
    let (low, high) = r;
    for i in low..=high {
        let s = i.to_string();
        if s.len() % 2 == 1 {
            continue;
        }
        let split = s.split_at(s.len() / 2);
        if split.0 == split.1 {
            invalid += i;
        }
    }
    invalid
}

fn find_invalid_in_range_p2(r: (u64, u64)) -> u64 {
    let mut invalid = 0;
    let (low, high) = r;
    for i in low..=high {
        let s = i.to_string();
        if is_invalid_p2(&s) {
            invalid += i;
        }
    }
    invalid
}

fn is_invalid_p2(s: &str) -> bool {
    for part_size in 1..=(s.len() / 2) {
        if s.len() % part_size == 0 {
            let parts = EqualSplit::new(&s, part_size);
            if are_all_same(parts) {
                return true;
            }
        }
    }
    false
}

fn are_all_same(split: EqualSplit) -> bool {
    let mut iter = split.into_iter();
    let first = iter.next().unwrap();
    for s in iter {
        if s != first {
            return false;
        }
    }
    return true;
}

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .split(",")
        .map(|rng| {
            rng.split("-")
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .map(|v| (v[0], v[1]))
        .collect()
}
