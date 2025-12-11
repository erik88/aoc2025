use std::collections::HashMap;

/// The test file for day11 should include both p1 and p2,
/// separated by an empty line!
pub fn day11(_lines: Vec<String>, test: bool) {
    let (nodes_p1, nodes_p2) = parse_nodes(_lines, test);
    let p1 = count_paths("you", &nodes_p1);
    let p2 = count_paths_p2("svr", &nodes_p2, false, false, vec![]);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn count_paths_p2(
    id: &str,
    nodes: &HashMap<String, Vec<String>>,
    visited_dac: bool,
    visited_fft: bool,
    mut visited: Vec<String>,
) -> u64 {
    if id == "out" {
        return if visited_dac && visited_fft { 1 } else { 0 };
    }
    if visited.contains(&id.to_string()) {
        return 0;
    }
    visited.push(id.to_string());
    let exits = nodes.get(id).unwrap();
    exits
        .iter()
        .map(|exit_id| {
            let dac = visited_dac || id == "dac";
            let fft = visited_fft || id == "fft";
            count_paths_p2(exit_id, nodes, dac, fft, visited.clone())
        })
        .sum()
}

fn count_paths(id: &str, nodes: &HashMap<String, Vec<String>>) -> u64 {
    if id == "out" {
        return 1;
    }
    let exits = nodes.get(id).unwrap();
    exits
        .iter()
        .map(|exit_id| count_paths(exit_id, nodes))
        .sum()
}

fn parse_nodes(
    lines: Vec<String>,
    test: bool,
) -> (HashMap<String, Vec<String>>, HashMap<String, Vec<String>>) {
    if !test {
        let hash_nodes = parse_node_section(&lines);
        return (hash_nodes.clone(), hash_nodes);
    } else {
        let mut split = lines.split(|l| l.is_empty());
        let p1 = split.next().unwrap();
        let p2 = split.next().unwrap();
        let hash_nodes_p1 = parse_node_section(&p1.to_vec());
        let hash_nodes_p2 = parse_node_section(&p2.to_vec());
        return (hash_nodes_p1, hash_nodes_p2);
    }
}

fn parse_node_section(lines: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut hash_nodes: HashMap<String, Vec<String>> = HashMap::new();
    lines.iter().for_each(|l| {
        let (id, strcon) = l.split_once(':').unwrap();
        let connections = &strcon[1..]
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        hash_nodes.insert(id.to_string(), connections.clone());
    });
    hash_nodes
}
