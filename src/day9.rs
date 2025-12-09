use std::cmp::{max, min};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Rect {
    startx: usize,
    starty: usize,
    endx_inclusive: usize,
    endy_inclusive: usize,
}

impl Rect {
    fn from_points(p1: Point, p2: Point) -> Self {
        Rect {
            startx: min(p1.x, p2.x),
            starty: min(p1.y, p2.y),
            endx_inclusive: max(p1.x, p2.x),
            endy_inclusive: max(p1.y, p2.y),
        }
    }

    fn is_inside(&self, point: Point) -> bool {
        self.startx < point.x
            && point.x < self.endx_inclusive
            && self.starty < point.y
            && point.y < self.endy_inclusive
    }

    fn width(&self) -> usize {
        self.endx_inclusive - self.startx + 1
    }

    fn height(&self) -> usize {
        self.endy_inclusive - self.starty + 1
    }
}

pub fn day9(lines: Vec<String>) {
    let points = parse_lines(&lines);

    let mut highest_p1 = 0;
    let mut highest_p2 = 0;

    for &p1 in &points {
        for &p2 in &points {
            let size = (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1);
            if size > highest_p1 {
                highest_p1 = size;
            }

            let r = Rect::from_points(p1, p2);
            if is_any_boundary_point_inside_rect(r, &points) {
                continue;
            }
            if rect_is_intersecting_boundary_line(r, &points) {
                continue;
            }
            if !is_any_point_in_rect_inside_polygon(r, &points) {
                continue;
            }

            if size > highest_p2 {
                highest_p2 = size;
            }
        }
    }

    println!("Part 1: {}", highest_p1);
    // With only the point-inside limitation: 4629432496 (too high, du-uh)
    println!("Part 2: {}", highest_p2);
}

fn rect_is_intersecting_boundary_line(r: Rect, boundary: &Vec<Point>) -> bool {
    let mut prev = boundary.last().cloned().unwrap();
    for &curr in boundary {
        if prev.x == curr.x {
            // Vertical line
            if r.startx < prev.x
                && prev.x < r.endx_inclusive
                && min(prev.y, curr.y) <= r.starty
                && max(prev.y, curr.y) >= r.endy_inclusive
            {
                return true;
            }
        } else if curr.y == prev.y {
            // Horizontal line
            if r.starty < prev.y
                && prev.y < r.endy_inclusive
                && min(prev.x, curr.x) <= r.startx
                && max(prev.x, curr.x) >= r.endx_inclusive
            {
                return true;
            }
        } else {
            panic!("Boundary points without 90 degree lines!?")
        }
        prev = curr;
    }
    false
}

fn is_any_boundary_point_inside_rect(r: Rect, boundary: &Vec<Point>) -> bool {
    for p in boundary {
        if r.is_inside(*p) {
            return true;
        }
    }
    false
}

fn is_any_point_in_rect_inside_polygon(r: Rect, boundary: &Vec<Point>) -> bool {
    // We cannot determine for a point on the edge,
    // we need a point actually _inside_ the rect!
    if r.width() < 3 || r.height() > 3 {
        return true;
    }
    // ray cast in x-direction
    // x-coordinates have 2667 and 2668 - but no y coordinates differ by only one
    // In other words, the ray cast will never hit a straight line by offsetting one of the rect's x corners by one
    let raycast_y = r.starty + 1;
    let raycast_end_x = r.startx + 1;

    let mut count_intersections = 0;
    let mut prev = boundary.last().cloned().unwrap();
    for &curr in boundary {
        if line_intersects_raycast(prev, curr, raycast_y, raycast_end_x) {
            count_intersections += 1;
        }
        prev = curr;
    }

    count_intersections % 2 == 1
}

fn line_intersects_raycast(p1: Point, p2: Point, raycast_y: usize, raycast_end_x: usize) -> bool {
    if p1.y == p2.y {
        // Horizontal line, same as raycast. Will never intersect.
        return false;
    }
    let line_x = p1.x;
    let line_start_y = min(p1.y, p2.y);
    let line_end_y = max(p1.y, p2.y);

    (line_start_y..line_end_y).contains(&raycast_y) && (..raycast_end_x).contains(&line_x)
}

fn parse_lines(lines: &Vec<String>) -> Vec<Point> {
    let mut points = Vec::new();
    for line in lines {
        let mut split = line.split(',');
        let p = Point {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
        };
        points.push(p);
    }
    points
}
