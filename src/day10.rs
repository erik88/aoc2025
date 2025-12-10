use std::collections::VecDeque;

#[derive(Debug)]
struct Machine {
    state: usize,
    buttons: Vec<usize>,
}

pub fn day10(lines: Vec<String>) {
    let mut machines = parse_lines(&lines);
    let mut button_presses = 0;
    for machine in &mut machines {
        button_presses += solve(machine);
    }
    println!("Part 1: {}", button_presses);
}

fn solve(machine: &mut Machine) -> usize {
    let mut frontier = VecDeque::new();
    frontier.push_back((machine.state, 0_usize));
    let mut shortest_distance = vec![9999; 1025];
    let mut shortest_distance_to_goal = 9999;

    while let Some((state, distance)) = frontier.pop_front() {
        if distance >= shortest_distance_to_goal || distance >= shortest_distance[state] {
            continue;
        }
        shortest_distance[state] = distance;
        if state == 0 {
            shortest_distance_to_goal = distance;
        } else {
            for button in &machine.buttons {
                frontier.push_back((state ^ button, distance + 1));
            }
        }
    }
    shortest_distance_to_goal
}

fn parse_lines(lines: &[String]) -> Vec<Machine> {
    lines
        .iter()
        .map(|l| {
            let (first, rest) = l.split_once(']').unwrap();
            let (mid, end) = rest.split_once('{').unwrap();

            Machine {
                state: parse_state(&first[1..]),
                buttons: parse_buttons(&mid[1..mid.len() - 1]),
            }
        })
        .collect()
}

fn parse_buttons(input: &str) -> Vec<usize> {
    input
        .split(" ")
        .map(|s| parse_button(&s[1..s.len() - 1]))
        .collect()
}

fn parse_button(input: &str) -> usize {
    input.split(',').fold(0, |total, strnum| {
        total + 2_usize.pow(strnum.parse().unwrap())
    })
}

fn parse_state(s: &str) -> usize {
    s.chars().enumerate().fold(0, |prev, (index, c)| {
        prev + if c == '#' {
            2_usize.pow(index as u32)
        } else {
            0
        }
    })
}
