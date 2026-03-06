use std::collections::HashMap;

mod dfs_and_bfs;
mod dijkstras;
mod grids;
mod priority;
// mod spatial_trees;

fn main() {
    let graph = HashMap::<&str, Vec<&'static str>>::from([
        ("A", vec!["B", "F", "H"]),
        ("B", vec!["C"]),
        ("C", vec!["D", "E"]),
        ("F", vec!["G", "I"]),
    ]);
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
    let mut visited: Vec<&str> = Vec::new();

    println!("Performing a breadth first search starting at node A");
    dfs_and_bfs::breadth_first_search(&graph, "A", &mut visited);
}
