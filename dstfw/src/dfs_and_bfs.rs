// use std::collections::HashMap;

// For later
// use std::collections::{HashMap, HashSet};
use std::collections::{HashMap, VecDeque};

fn depth_first_search<'a>(
    // Accepts a graph that is a HashMap along with a starting node and a Vector for visited nodes
    // Returns a vector of the nodes in the order they were visited the first time
    graph: &HashMap<&str, Vec<&'static str>>,
    start: &'a str,
    visited: &mut Vec<&'a str>,
) -> Vec<&'a str> {
    if visited.contains(&start) {
        return visited.to_vec();
    }
    // Visit the node
    println!("Now visiting node: {:?}", &start);
    visited.push(start);
    // Check that there are any neighbors (the key in the HashMap)
    if let Some(neighbors) = graph.get(start) {
        for neighbor in neighbors {
            depth_first_search(graph, neighbor, visited);
        }
    }
    visited.to_vec()
}

fn depth_first_search_with_vecdeque<'a>(
    // Accepts a graph that is a HashMap along with a starting node and a Vector for visited nodes
    // Returns a vector of the nodes in the order they were visited the first time
    graph: &HashMap<&str, Vec<&'static str>>,
    start: &'a str,
    visited: &mut Vec<&'a str>,
) -> Vec<&'a str> {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_back() {
        if !visited.contains(&node) {
            println!("Now visiting node {:?}", &node);
            visited.push(&node);

            // If there are neighbors, put them into the queue to be looked at
            if let Some(neighbors) = graph.get(node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    visited.to_vec()
}

fn breadth_first_search<'a>(
    graph: &HashMap<&str, Vec<&'static str>>,
    start: &'a str,
    visited: &mut Vec<&'a str>,
) -> Vec<&'a str> {
    // Start a double-ended queue to maintain unvisited nodes
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if !visited.contains(&node) {
            println!("Now visiting node: {:?}", node);
            visited.push(node);
            if let Some(neighbors) = graph.get(node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    visited.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph_setup() -> HashMap<&'static str, Vec<&'static str>> {
        // This represents an undirected graph of simple nodes and edges:
        //
        // +---+    +---+     +---+     +---+     +---+
        // | H |<---| A |---->| B |---->| C |---->| D |
        // +---+    +---+     +---+     +---+     +---+
        //            |                   |
        //            v                   v
        //          +---+     +---+     +---+
        //          | F |---->| G |     | E |
        //          +---+     +---+     +---+
        //            |         ^
        //            v         |
        //          +---+       |
        //          | I |<------+
        //          +---+
        //
        HashMap::<&str, Vec<&'static str>>::from([
            ("A", vec!["B", "F", "H"]),
            ("B", vec!["C"]),
            ("C", vec!["D", "E"]),
            ("F", vec!["G", "I"]),
        ])
    }

    #[test]
    fn test_hash_map_behavior_basics() {
        let mut myhash = HashMap::new();
        myhash.insert("name".to_string(), "Cooper".to_string());
        assert_eq!(myhash.get("name"), Some(&"Cooper".to_string()));
    }

    #[test]
    fn test_dfs_setup() {
        let nodes = graph_setup();

        // Check that they all made it in
        assert_eq!(nodes.len(), 4);
        // Check that B was correctly assigned to a Vector with only "C" in it
        assert_eq!(nodes.get("B"), Some(&vec!["C"]));
        // Check that G returns nothing since it has no children
        assert_eq!(nodes.get("G"), None);
    }

    #[test]
    fn run_dfs() {
        let graph = graph_setup();
        let mut visited: Vec<&'static str> = Vec::new();
        dbg!(&graph);

        depth_first_search(&graph, "F", &mut visited);
    }

    #[test]
    fn test_dfs_small_step() {
        let graph = graph_setup();
        let mut visited: Vec<&'static str> = Vec::new();
        dbg!(&graph);

        depth_first_search(&graph, "B", &mut visited);
        assert_eq!(visited, Vec::from(["B", "C", "D", "E"]));
    }

    #[test]
    fn test_dfs_full() {
        let graph = graph_setup();
        let mut visited: Vec<&'static str> = Vec::new();
        dbg!(&graph);

        depth_first_search(&graph, "A", &mut visited);
        assert_eq!(visited, vec!["A", "B", "C", "D", "E", "F", "G", "I", "H"]);
    }

    #[test]
    fn run_small_hfs() {
        let graph = graph_setup();
        let mut visited: Vec<&'static str> = Vec::new();
        dbg!(&graph);

        breadth_first_search(&graph, "F", &mut visited);
        assert_eq!(visited, vec!["F", "G", "I"]);
    }

    #[test]
    fn run_full_hfs() {
        let graph = graph_setup();
        let mut visited: Vec<&'static str> = Vec::new();
        dbg!(&graph);

        breadth_first_search(&graph, "A", &mut visited);
        assert_eq!(visited, vec!["A", "B", "F", "H", "C", "G", "I", "E", "D"]);
    }

    #[test]
    fn test_dfs_full_with_vecdeque() {
        let graph = HashMap::<&str, Vec<&'static str>>::from([
            ("A", vec!["H", "F", "B"]),
            ("B", vec!["C"]),
            ("C", vec!["E", "D"]),
            ("F", vec!["I", "G"]),
        ]);
        let mut visited: Vec<&'static str> = Vec::new();
        dbg!(&graph);

        depth_first_search_with_vecdeque(&graph, "A", &mut visited);
        assert_eq!(visited, vec!["A", "B", "C", "D", "E", "F", "G", "I", "H"]);
    }
}
