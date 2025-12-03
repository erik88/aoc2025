pub fn day3(lines: Vec<String>) {
    let p1: u64 = lines.iter().map(|line| find_joltage(line)).sum();
    println!("Part 1: {}", p1);
}

fn find_joltage(line: &str) -> u64 {
    let (ten_index, ten_val) = find_highest(&line[..line.len()-1]);
    let (_, single_val) = find_highest(&line[(ten_index+1)..]);

    return ten_val*10 + single_val;
}

fn find_highest(line: &str) -> (usize, u64) {
    let mut highest: char = '0';
    let mut highest_index: usize = 0;
    for (index, c) in line.chars().enumerate() {
        if c > highest {
            highest = c;
            highest_index = index;
            if highest == '9' {
                break;
            }
        }
    }
    (highest_index, highest.to_digit(10).unwrap() as u64)
}
