struct Machine {
    state: u64,
    buttons: Vec<u64>
}

pub fn day10(lines: &Vec<String>) {
    let mut machines = parse_lines(&lines);
}

fn parse_lines(lines: &[String]) -> Vec<Machine> {
    lines.iter().map(|l| {
        let (first, rest) = l.split_once(']').unwrap();
        let (mid, end) = rest.split_once('{').unwrap();

        Machine {
            state: parse_state(&first[1..]),
            buttons: parse_buttons(&mid[1..mid.len()-1])
        }
    }).collect()
}

fn parse_buttons(input: &str) -> Vec<u64> {
    input.split(" ").map(|s| parse_button(&s[1..s.len()-1])).collect()
}

fn parse_button(input: &str) -> u64 {
    input.split(',')
        .fold(
            0,
            |total, strnum| total + 2_u64.pow(strnum.parse().unwrap())
        )
}

fn parse_state(s: &str) -> u64 {
    s.chars().enumerate().fold(0, |prev, (index, c)| prev + if c == '#' { 2_u64.pow(index as u32) } else { 0 } )
}
