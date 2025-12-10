use std::collections::VecDeque;
use std::fs;
use std::process::Command;

#[derive(Debug)]
struct Machine {
    state: usize,
    buttons: Vec<usize>,
    buttons_p2: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

pub fn day10(lines: Vec<String>) {
    let machines = parse_lines(&lines);
    let mut button_presses_p1 = 0;
    let mut button_presses_p2 = 0;
    let mut count = 0;
    for machine in &machines {
        button_presses_p1 += solve_p1(machine);
        count = count + 1;
        print_gmpl(machine, count);
        button_presses_p2 += solve_gmpl(count);
    }
    println!("Part 1: {}", button_presses_p1);
    println!("Part 2: {}", button_presses_p2);
}

fn solve_gmpl(line: usize) -> usize {
    let mut command = Command::new("glpsol");
    command
        .arg("--model")
        .arg(format!("day10-gmpl/{}.mod", { line }));
    let p = command.output().unwrap();
    let file = String::from_utf8_lossy(&p.stdout);
    let lines = file.split("\n").collect::<Vec<&str>>();
    let second_to_last_line = lines.iter().nth_back(2).unwrap();
    println!("Step: {}", second_to_last_line);
    second_to_last_line.parse::<f64>().unwrap().round() as usize
}

fn print_gmpl(machine: &Machine, count: usize) {
    let variable_count = machine.buttons_p2.len();
    let mut s = String::new();

    for i in 0..variable_count {
        s += &format!("var x{} integer >= 0;\n", i);
    }

    let objective =
        (1..variable_count).fold("x0".to_string(), |acc, x| acc + &format!(" + x{}", x));
    s += &format!("minimize objective: {};\n", objective);

    // subject to constraints: x + y <= 4 and 2 * x + y >= 2.
    for joltage_index in 0..machine.joltage.len() {
        let mut vars = Vec::new();
        for (button_index, button) in machine.buttons_p2.iter().enumerate() {
            if button.contains(&joltage_index) {
                vars.push(button_index);
            }
        }
        let lhs = vars
            .iter()
            .map(|&b| format!("x{}", b))
            .collect::<Vec<String>>()
            .join(" + ");

        s += &format!(
            "s.t. constraint{}: {} = {};\n",
            joltage_index + 1,
            lhs,
            machine.joltage[joltage_index]
        );
    }

    s += "solve;\n";

    s += &format!("printf \"%.4f\\n\", {};", objective);
    s += "end;";
    let path = &format!("day10-gmpl/{}.mod", count);
    fs::write(path, s).expect(&format!("Should be able to write to `{}`", path));
}

fn solve_p1(machine: &Machine) -> usize {
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
            let (buttons, buttons_p2) = parse_buttons(&mid[1..mid.len() - 1]);

            Machine {
                state: parse_state(&first[1..]),
                buttons,
                buttons_p2,
                joltage: parse_joltage(&end[..end.len() - 1]),
            }
        })
        .collect()
}

fn parse_joltage(input: &str) -> Vec<usize> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn parse_buttons(input: &str) -> (Vec<usize>, Vec<Vec<usize>>) {
    (
        input
            .split(" ")
            .map(|s| parse_button(&s[1..s.len() - 1]))
            .collect(),
        input
            .split(" ")
            .map(|s| parse_button_p2(&s[1..s.len() - 1]))
            .collect(),
    )
}

fn parse_button(input: &str) -> usize {
    input.split(',').fold(0, |total, strnum| {
        total + 2_usize.pow(strnum.parse().unwrap())
    })
}

fn parse_button_p2(input: &str) -> Vec<usize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
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
