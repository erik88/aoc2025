use std::collections::BTreeSet;

#[derive(Clone, Copy, Ord, Eq, PartialEq, PartialOrd, Debug)]
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
    let (intervals, items) = parse_input(&lines);
    let mut fresh_count = 0;

    for item in items {
        for interval in &intervals {
            if interval.contains(item) {
                fresh_count += 1;
                break;
            }
        }
    }

    let non_overlapping = merge_all_intervals(&intervals);
    let all_count: u64 = non_overlapping.iter().map(|i| i.size()).sum();

    println!("Part 1: {}", fresh_count);
    println!("Part 2: {}", all_count);
    // Too low!  344638686630694
    // Too high! 358655166017626
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
        } else { // parsing item
            items.push(line.parse().unwrap());
        }
    }

    (intervals, items)
}

fn merge_all_intervals(intervals: &Vec<Interval>) -> Vec<Interval> {
    let mut uncleared = BTreeSet::from_iter(intervals);
    let mut done: Vec<Interval> = Vec::new();
    let mut highest_cleared = 0;

    while let Some(i1) = uncleared.pop_first() {
        if i1.end_inclusive <= highest_cleared {
            // Drop it
            continue;
        }

        if let Some(i2) = uncleared.first() {
            if i1.end_inclusive < i2.start {
                done.push(*i1);
                highest_cleared = i1.end_inclusive;
            } else if i1.start == i2.start {
                // i2 is larger, drop i1
            } else {
                if i1.end_inclusive > i2.end_inclusive {
                    uncleared.pop_first(); // Remove i2
                    uncleared.insert(i1); // Try i1 again against next
                } else {
                    let shortened = Interval { start: i1.start, end_inclusive: i2.start - 1};
                    if shortened.size() > 0 {
                        done.push(shortened);
                        highest_cleared = shortened.end_inclusive;
                    }
                };
            }
        } else {
            done.push(*i1);
        }
    }
    assert!(uncleared.len() == 0, "There were still uncleared left!?");
    done
}
