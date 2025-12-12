struct Region {
    width: usize,
    height: usize,
    no_presents: Vec<usize>,
}

pub fn day12(lines: Vec<String>) {
    let iter = lines.iter();

    // By inspection
    let present_sizes = vec![6, 5, 7, 7, 7, 7];

    let regions: Vec<Region> = iter
        .skip(30)
        .map(|l| {
            let (first, second) = l.split_once(": ").unwrap();
            let (w, h) = first.split_once('x').unwrap();
            let presents = second
                .split(' ')
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            Region {
                width: w.parse().unwrap(),
                height: h.parse().unwrap(),
                no_presents: presents,
            }
        })
        .collect();

    println!("Total regions: {}", regions.iter().count());

    let regions_space_total: Vec<Region> = regions
        .into_iter()
        .filter(|r| {
            r.width * r.height
                >= r.no_presents
                    .iter()
                    .enumerate()
                    .map(|(index, n)| *n * present_sizes[index])
                    .sum::<usize>()
        })
        .collect();

    println!(
        "Regions (when impossible filtered out): {}",
        regions_space_total.iter().count()
    );

    let mut count = 0;
    for r in regions_space_total {
        if (r.width / 3) * (r.height / 3) >= r.no_presents.iter().sum::<usize>() {
            count += 1;
        }
    }
    println!(
        "Number of regions where presents will definitely fit: {}",
        count
    );
}
