pub fn day7(lines: Vec<String>) {
    let mut iter = lines.iter();
    let first_row = iter.next().unwrap();
    let row_width = first_row.len();

    let mut beams: Vec<u64> = vec![0; row_width];
    let beam_start_x = first_row.find('S').unwrap();
    beams[beam_start_x] = 1;

    let mut split_count = 0;

    for row in iter {
        let mut new_beams: Vec<u64> = vec![0; row_width];
        for (col_index, beam) in beams.iter().enumerate() {
            if *beam == 0 {
                continue;
            }
            let c = row.chars().nth(col_index).unwrap();
            if c == '^' {
                split_count += 1;
                // Looking at the input data, the splitters are never at the edge
                // so it is safe to do col_index ± 1
                new_beams[col_index - 1] += beams[col_index];
                new_beams[col_index + 1] += beams[col_index];
            } else {
                new_beams[col_index] += beams[col_index];
            }
        }
        beams = new_beams;
    }

    println!("Part 1: {}", split_count);
    println!("Part 2: {}", beams.iter().sum::<u64>());
}
