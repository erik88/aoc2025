use std::{collections::HashMap, fmt::Display};

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    fn from_string(s: &str) -> Point {
        let mut split = s.split(',');
        Point {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
            z: split.next().unwrap().parse().unwrap(),
        }
    }

    fn distance(&self, other: Self) -> f64 {
        ((self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)) as f64)
            .sqrt()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

struct Distance {
    p1: Point,
    p2: Point,
    size: f64,
}

impl Distance {
    fn between_points(p1: Point, p2: Point) -> Distance {
        Distance {
            p1,
            p2,
            size: p1.distance(p2),
        }
    }
}

struct Circuit {
    id: usize,
    points: Vec<Point>,
}

impl Circuit {
    fn new(id: usize) -> Self {
        Circuit {
            id,
            points: Vec::new(),
        }
    }

    fn add(&mut self, p: Point) {
        self.points.push(p);
    }

    fn size(&self) -> usize {
        self.points.len()
    }
}

impl Display for Circuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.id,
            self.points
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

// ==============================================================

pub fn day8(lines: Vec<String>, is_test: bool) {
    let num_connections_p1 = if is_test { 10 } else { 1000 };
    let points = parse_points(&lines);
    let distances = get_sorted_distances_between_points(&points);

    let mut circuits: Vec<Circuit> = Vec::new();
    let mut hashmap: HashMap<Point, usize> = HashMap::new();
    let mut circuit_id_count = 0;
    let (mut last_x1, mut last_x2) = (0, 0);
    let mut count_iterations = 0;

    for d in distances {
        last_x1 = d.p1.x;
        last_x2 = d.p2.x;
        let _circuit1 = hashmap.get(&d.p1).cloned();
        let _circuit2 = hashmap.get(&d.p2).cloned();

        match (_circuit1, _circuit2) {
            (Some(circuit1_id), Some(circuit2_id)) => {
                if circuit1_id == circuit2_id {
                    // Do nothing, both points already on the same circuit
                } else {
                    // Merge circuits
                    let old_index = circuits.iter().position(|c| c.id == circuit1_id).unwrap();
                    let new_circuit = circuits.iter().position(|c| c.id == circuit2_id).unwrap();
                    let points = circuits[old_index].points.clone();
                    for p in points {
                        hashmap.insert(p, circuit2_id);
                        circuits[new_circuit].add(p);
                    }
                    circuits.remove(old_index);
                }
            }
            (Some(circuit_id), None) => {
                let c = circuits.iter_mut().find(|c| c.id == circuit_id).unwrap();
                c.add(d.p2);
                hashmap.insert(d.p2, circuit_id);
            }
            (None, Some(circuit_id)) => {
                let c = circuits.iter_mut().find(|c| c.id == circuit_id).unwrap();
                c.add(d.p1);
                hashmap.insert(d.p1, circuit_id);
            }
            (None, None) => {
                let mut circuit = Circuit::new(circuit_id_count);
                circuit_id_count += 1;
                circuit.add(d.p1);
                circuit.add(d.p2);
                hashmap.insert(d.p1, circuit.id);
                hashmap.insert(d.p2, circuit.id);
                circuits.push(circuit);
            }
        }

        // p1 ---------------------------------
        count_iterations += 1;
        if count_iterations == num_connections_p1 {
            circuits.sort_by(|a, b| a.size().cmp(&b.size()));
            circuits.reverse();

            let res = circuits
                .iter()
                .take(3)
                .map(|c| c.size())
                .fold(1, |a, b| a * b);

            println!("Part 1: {}", res);
        }

        // p2, stopping condition --------------
        if circuits[0].size() == points.len() {
            break;
        }
    }

    println!("Part 2: {}", last_x1 * last_x2);
}

// ==============================================================

fn parse_points(lines: &Vec<String>) -> Vec<Point> {
    lines.iter().map(|l| Point::from_string(l)).collect()
}

fn get_sorted_distances_between_points(points: &Vec<Point>) -> Vec<Distance> {
    let mut distances: Vec<Distance> = Vec::with_capacity(500_000);
    for p1 in 0..points.len() - 1 {
        for p2 in p1 + 1..points.len() {
            distances.push(Distance::between_points(points[p1], points[p2]));
        }
    }
    distances.sort_by(|d1, d2| d1.size.total_cmp(&d2.size));

    distances
}
