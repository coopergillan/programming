// use std::collections::HashMap;

// For later
use std::collections::{HashMap, HashSet};
// use std::collections::{HashMap, VecDeque};

fn depth_first_search<'a>(
    graph: &HashMap<&str, Vec<&'static str>>,
    start: &'a str,
    visited: &mut HashSet<&'a str>,
) {
    if visited.contains(start) {
        return;
    }
    // Visit the node
    println!("Now visiting node: {:?}", start);
    visited.insert(start);
    // Check that there are any neighbors (the key in the HashMap)
    if let Some(neighbors) = graph.get(start) {
        for neighbor in neighbors {
            depth_first_search(graph, neighbor, visited);
        }
    }
}

fn main() {
    let my_graph = HashMap::<&str, Vec<&'static str>>::from([
        ("A", vec!["B", "F", "H"]),
        ("B", vec!["C"]),
        ("C", vec!["D", "E"]),
        ("F", vec!["G", "I"]),
    ]);
    let mut visited: HashSet<&'static str> = HashSet::new();

    depth_first_search(&my_graph, "A", &mut visited);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dfs_setup() -> HashMap<&'static str, Vec<&'static str>> {
        HashMap::<&str, Vec<&'static str>>::from([
            ("A", vec!["B", "F", "H"]),
            ("B", vec!["C"]),
            ("C", vec!["D", "E"]),
            ("F", vec!["G", "I"]),
        ])
    }

    #[ignore]
    #[test]
    fn test_hash_map_behavior_basics() {
        let mut myhash = HashMap::new();
        myhash.insert("name".to_string(), "Cooper".to_string());
        assert_eq!(myhash.get("name"), Some(&"Cooper".to_string()));
    }

    #[ignore]
    #[test]
    fn test_dfs_setup() {
        let nodes = dfs_setup();

        // Check that they all made it in
        assert_eq!(nodes.len(), 4);
        // Check that B was correctly assigned to a Vector with only "C" in it
        assert_eq!(nodes.get("B"), Some(&vec!["C"]));
        // Check that G returns nothing since it has no children
        assert_eq!(nodes.get("G"), None);
    }

    #[test]
    fn test_dfs() {
        let graph = dfs_setup();
        let mut visited: HashSet<&'static str> = HashSet::new();
        dbg!(&graph);

        depth_first_search(&graph, "F", &mut visited);
    }
}
