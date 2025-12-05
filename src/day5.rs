#[derive(Clone, Copy)]
struct Interval {
    start: u64,
    end_inclusive: u64
}

impl Interval {
    fn contains(&self, item: u64) -> bool {
        return self.start <= item && item <= self.end_inclusive;
    }

    fn size(&self) -> u64 {
        return self.end_inclusive - self.start + 1;
    }
}

pub fn day5(lines: Vec<String>) {
    let (mut intervals, items) = parse_input(&lines);
    let mut fresh_count = 0;

    for item in items {
        for interval in &intervals {
            if interval.contains(item) {
                fresh_count += 1;
                break;
            }
        }
    }

    let mut all_count = 0;
    merge_all_intervals(&mut intervals);
    let all_count = intervals.iter().map(|i| i.size()).sum();

    println!("Part 1: {}", fresh_count);
    println!("Part 2: {}", all_count);
}

fn parse_input(lines: &Vec<String>) -> (Vec<Interval>, Vec<u64>) {
    let mut parsing_intervals = true;
    let mut intervals: Vec<Interval> = Vec::new();
    let mut items: Vec<u64> = Vec::new();

    for line in lines {
        if line.is_empty() {
            parsing_intervals = false;
            continue;
        }

        if parsing_intervals {
            let mut s = line.split('-');

            intervals.push(Interval {
                start: s.next().unwrap().parse().unwrap(),
                end_inclusive: s.next().unwrap().parse().unwrap()
            })
        } else { // parsing items
            items.push(line.parse().unwrap());
        }
    }

    (intervals, items)
}

fn merge_all_intervals(intervals: &mut Vec<Interval>) {
    !todo!("not yet implements");
}

fn merge_intervals(i1: Interval, i2: Interval) -> (Option<Interval>, Option<Interval>) {
    // i1 completely contains i2
    if i1.start <= i2.start && i1.end_inclusive >= i2.end_inclusive {
        return (Some(i1), None);
    }

    // i2 completely contains i1
    if i2.start <= i1.start && i2.end_inclusive >= i1.end_inclusive {
        return (Some(i2), None);
    }

    // Check if intervals overlap or are adjacent
    if i1.end_inclusive + 1 >= i2.start && i2.end_inclusive + 1 >= i1.start {
        // Merge overlapping or adjacent intervals
        return (Some(Interval {
            start: i1.start.min(i2.start),
            end_inclusive: i1.end_inclusive.max(i2.end_inclusive)
        }), None);
    }

    // No overlap - return both intervals
    (Some(i1), Some(i2))
}
