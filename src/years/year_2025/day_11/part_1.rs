use std::collections::HashMap;

type Graph = HashMap<String, Vec<String>>;

#[must_use]
pub fn solve(input: &str) -> u64 {
    let graph = parse(input);
    let mut cache = HashMap::new();
    count_paths(&graph, &mut cache, "you", "out")
}

// DFS
fn count_paths(graph: &Graph, cache: &mut HashMap<String, u64>, node: &str, end: &str) -> u64 {
    if node == end {
        return 1;
    }

    if let Some(&cached) = cache.get(node) {
        return cached;
    }

    let result = graph
        .get(node)
        .into_iter()
        .flatten()
        .map(|neighbor| count_paths(graph, cache, neighbor, end))
        .sum();

    cache.insert(node.to_string(), result);
    result
}

fn parse(input: &str) -> Graph {
    input
        .lines()
        .map(|line| {
            let (node, neighbors) = line.split_once(':').unwrap();
            let neighbors = neighbors
                .split_whitespace()
                .map(ToString::to_string)
                .collect();
            (node.trim().to_string(), neighbors)
        })
        .collect()
}
