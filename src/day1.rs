pub fn day1(lines: Vec<String>) {
    let mut current: i64 = 50;
    let mut zero_touch_count = 0;
    let mut zero_cross_count = 0;

    lines.iter().for_each(|line| {
        let turn_value = parse_instruction(line);

        let old = current;
        let current_unwrapped = current + turn_value;
        current = current_unwrapped.rem_euclid(100);

        if current == 0 {
            zero_touch_count += 1;
        }

        // Every time it crosses over...
        let mut crosses = current_unwrapped
            .div_euclid(100)
            .abs_diff(old.div_euclid(100));
        if (current == 0 && turn_value > 0) || (old == 0 && turn_value < 0) {
            crosses -= 1;
        }
        zero_cross_count += crosses;
    });

    println!("Part 1: {}", zero_touch_count);
    println!("Part 2: {}", zero_touch_count + zero_cross_count);
}

fn parse_instruction(line: &str) -> i64 {
    let direction = line.chars().next().unwrap();
    let value_str = &line[1..];
    let value = value_str.parse::<i64>().unwrap();
    match direction {
        'L' => -value,
        'R' => value,
        _ => panic!("invalid direction: {}", direction),
    }
}
