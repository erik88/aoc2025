use std::{collections::HashMap, fmt::Display, panic};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: u64,
    y: u64,
    z: u64
}

impl Point {
    fn from_string(s: &str) -> Point {
        let mut split = s.split(',');
        Point {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
            z: split.next().unwrap().parse().unwrap()
        }
    }

    fn abs_diff(&self, other: Self) -> f64 {
        ((self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)) as f64).sqrt()
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
    size: f64
}

impl Distance {
    fn between_points(p1: Point, p2: Point) -> Distance {
        Distance {
            p1,
            p2,
            size: p1.abs_diff(p2)
        }
    }
}

struct Circuit {
    id: usize,
    points: Vec<Point>
}

impl Circuit {
    fn new(id: usize) -> Self {
        Circuit { id, points: Vec::new() }
    }

    fn includes(&self, p: Point) -> bool {
        return self.points.iter().any(|point| *point == p);
    }

    fn add(&mut self, p: Point) {
        self.points.push(p);
    }
}

pub fn day8(lines: Vec<String>, is_test: bool) {
    let num_connections = if is_test { 10 } else { 1000 };
    let points: Vec<Point> = lines.iter().map(|l| Point::from_string(l)).collect();
    //points.sort_by(|p1, p2| p1.x.cmp(&p2.x));
    //
    let mut distances: Vec<Distance> = Vec::with_capacity(500_000);
    for p1 in 0..points.len()-1 {
        for p2 in p1+1..points.len() {
            distances.push(Distance::between_points(points[p1], points[p2]));
        }
    }

    distances.sort_by(|d1, d2| d1.size.total_cmp(&d2.size));

    let circuits: Vec<Circuit> = Vec::new();
    for d in distances.iter().take(num_connections) {
        match circuits_from_points(d.p1, d.p2) {
            (Some(circuit1_id), Some(circuit2_id)) => {

            },
            (Some(circuit_id), None) => {
                let c = &mut circuits[*circuit_id];
                c.push(d.p1);
            },
            (None, Some(circuit_id)) => {
                let c = &mut circuits[*circuit_id];
                c.push(d.p2);
            },
            (None, None) => {
                circuits.push(vec![d.p1, d.p2]);
                circuit_id_count += 1;
            },
        }
    }

    println!("Part 1: {}", 0);
    println!("Part 2: {}", 0);
}

fn circuits_from_points<'a>(circuits: &'a mut Vec<Circuit>, p1: Point, p2: Point) -> (&'a mut Option<Circuit>, &'a mut Option<usize>) {
    todo!()
}
