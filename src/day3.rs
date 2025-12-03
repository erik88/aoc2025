pub fn day3(lines: Vec<String>) {
    let p1: u64 = lines.iter().map(|line| find_joltage(line, 2)).sum();
    let p2: u64 = lines.iter().map(|line| find_joltage(line, 12)).sum();
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn find_joltage(line: &str, items: usize) -> u64 {
    let mut total = 0_u64;
    let mut string_position = 0_usize;

    for i in 0..items {
        let num_position = items-i;
        let (offset_in_slice, val) = find_highest(&line[string_position..line.len()-num_position+1]);
        string_position += offset_in_slice + 1;
        total += val * 10_u64.pow(num_position as u32-1) as u64;
    }

    println!("{} {}" ,line, total);
    return total;
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
