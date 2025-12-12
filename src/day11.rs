use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    name: String,
    next: Vec<usize>,
    prev: Vec<usize>,
}

#[derive(Clone)]
struct Search {
    size: u64,
    prev_counted: usize,
    visited_fft: bool,
    visited_dac: bool,
}

/// The test file for day11 should include both p1 and p2,
/// separated by an empty line!
pub fn day11(_lines: Vec<String>, test: bool) {
    let (_, hash_nodes_p2) = parse_nodes(_lines, test);
    let nodes: Vec<Node> = parse_real_nodes(&hash_nodes_p2);
    let mut searches: Vec<Search> = vec![
        Search {
            size: 0,
            prev_counted: 0,
            visited_fft: false,
            visited_dac: false
        };
        nodes.len()
    ];

    // println!("{:?}", nodes);

    let start_id = nodes.iter().find(|n| n.name == "svr").unwrap().id;
    let mut start_search = searches[start_id].clone();
    start_search.size = 1;
    go_to(start_id, start_search, &mut searches, &nodes);

    let end_node = nodes.iter().find(|n| n.name == "out").unwrap();
    let end_search = &searches[end_node.id];
    println!("Part 2: {}", end_search.size);
    // 1769761226520 is too low
    // 9463462054500 is also too low...

    print_graph(&nodes, &searches);
}

fn print_graph(nodes: &Vec<Node>, searches: &Vec<Search>) {
    let mut sn = String::new();
    let mut se = String::new();
    for i in 0..searches.len() {
        sn += &format!(
            "{}[style=\"filled\",color=\"{}\"];\n",
            nodes[i].name,
            get_color(&searches[i])
        );
        se += &format!(
            "{} -> {{ {} }}\n",
            nodes[i].name,
            nodes[i]
                .next
                .iter()
                .map(|&ni| nodes[ni].name.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }

    fs::write(
        "day11/graph_searched.dot",
        format!("digraph G {{\n{}\n{}\n}}", sn, se),
    )
    .unwrap();
}

fn get_color(s: &Search) -> &str {
    if s.visited_fft && s.visited_dac {
        "#ffffcc"
    } else if s.visited_dac {
        "#cd8632"
    } else if s.visited_fft {
        "#aaffcc"
    } else {
        "#bbbbbb"
    }
}

fn go_to(id: usize, prev_search: Search, searches: &mut Vec<Search>, nodes: &Vec<Node>) {
    let search = &mut searches[id];
    let node = &nodes[id];

    // if node.name == "ccc" {
    //     println!("ccc");
    // }

    if node.name == "fft" {
        search.visited_fft = true;
        search.size += prev_search.size;
    } else if node.name == "dac" {
        if prev_search.visited_fft {
            search.visited_fft = true;
        }
        search.visited_dac = true;
        search.size += prev_search.size;
    } else if prev_search.visited_fft && (!search.visited_fft) {
        // Clear all previous paths, they are now irrelevant!
        search.size = prev_search.size;
        search.visited_fft = true;
        if prev_search.visited_dac {
            search.visited_dac = true;
        }
    } else if prev_search.visited_dac && (!search.visited_dac) {
        // Clear all previous paths, they are now irrelevant!
        search.size = prev_search.size;
        search.visited_dac = true;
        if prev_search.visited_fft {
            search.visited_dac = true;
        }
    } else if (search.visited_fft && (!prev_search.visited_fft))
        || (search.visited_dac && (!prev_search.visited_dac))
    {
        // Don't count the incoming paths, they are irrelevant!
    } else {
        search.size += prev_search.size;
    }
    search.prev_counted += 1;
    // println!("At {} -> {}", node.name, search.size);

    let pass_along = search.clone();
    // First node will have +1 for prev_counted, so >= is necessary
    if search.prev_counted >= node.prev.len() {
        // println!("Done with {} -> {}", node.name, search.size);
        for next_id in &node.next {
            go_to(*next_id, pass_along.clone(), searches, nodes);
        }
    }
}

fn parse_real_nodes(hash_nodes: &HashMap<String, Vec<String>>) -> Vec<Node> {
    let mut nodes: Vec<Option<Node>> = vec![None; hash_nodes.len() + 1];
    let mut next_index = 0;
    let mut index_map: HashMap<String, usize> = HashMap::new();
    for (node_name, node_next) in hash_nodes {
        let index = get_or_create_index(node_name, &mut index_map, &mut next_index);
        let node = Node {
            id: index,
            name: node_name.to_string(),
            next: node_next
                .iter()
                .map(|next_name| get_or_create_index(next_name, &mut index_map, &mut next_index))
                .collect(),
            prev: vec![],
        };

        nodes[index] = Some(node);
    }
    let out_index = *index_map.get("out").unwrap();
    nodes[out_index] = Some(Node {
        id: out_index,
        name: "out".to_string(),
        next: vec![],
        prev: vec![],
    });

    let mut n: Vec<Node> = nodes.into_iter().map(|i| i.unwrap()).collect();

    for id in 0..n.len() {
        let node = &n[id];
        let next_ids = node.next.clone();
        for idn in next_ids {
            n[idn].prev.push(id);
        }
    }

    n
}

fn get_or_create_index(
    name: &str,
    index_map: &mut HashMap<String, usize>,
    next_index: &mut usize,
) -> usize {
    match index_map.get_mut(name) {
        Some(&mut index) => index,
        None => {
            let index = *next_index;
            *next_index += 1;
            index_map.insert(name.to_string(), index);
            index
        }
    }
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

        (hash_nodes.clone(), hash_nodes)
    } else {
        let mut split = lines.split(|l| l.is_empty());
        let p1 = split.next().unwrap();
        let p2 = split.next().unwrap();
        let hash_nodes_p1 = parse_node_section(&p1.to_vec());
        let hash_nodes_p2 = parse_node_section(&p2.to_vec());

        (hash_nodes_p1, hash_nodes_p2)
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
