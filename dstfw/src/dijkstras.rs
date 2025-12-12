use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    node_position: usize,
    cost: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare node positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node_position.cmp(&other.node_position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Edge {
    pub node: usize,
    pub cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
pub fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        node_position: start,
    });
    dbg!(&dist, &heap);

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State {
        cost,
        node_position,
    }) = heap.pop()
    {
        // Alternatively we could have continued to find all shortest paths
        dbg!(
            "@@@@ Back to the top of the while loop",
            &cost,
            &node_position
        );
        if node_position == goal {
            println!(
                "Found the goal - node_position {:?} - goal {:?}",
                &node_position, &goal
            );
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[node_position] {
            dbg!("Already have a better way", &cost, &node_position, &dist);
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        dbg!(
            "Ready to search nodes in adj_list[node_position]",
            &adj_list[node_position]
        );
        for edge in &adj_list[node_position] {
            let next = State {
                cost: cost + edge.cost,
                node_position: edge.node,
            };
            dbg!("Checking edge and next", &edge, &next);

            // If so, add it to the frontier and continue
            if next.cost < dist[next.node_position] {
                println!(
                    "Found a lower cost in next.cost {:?} than the distance of next.node_position {:?} in dist: {:?}",
                    &next.cost, &next.node_position, &dist[next.node_position]
                );
                heap.push(next);
                // We have now found a better way - right?
                dist[next.node_position] = next.cost;
            } else {
                dbg!(
                    "Already have a better way?",
                    &next.cost,
                    &dist[next.node_position]
                );
            }
            dbg!("Finished this loop of edge", &heap, &dist);
        }
    }

    // Goal not reachable
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstras() {
        // This is the directed graph we're going to use.
        // The node numbers correspond to the different states,
        // and the edge weights symbolize the cost of moving
        // from one node to another.
        // Note that the edges are one-way.
        //
        //                  7
        //          +-----------------+
        //          |                 |
        //          v   1        2    |  2
        //          0 -----> 1 -----> 3 ---> 4
        //          |        ^        ^      ^
        //          |        | 1      |      |
        //          |        |        | 3    | 1
        //          +------> 2 -------+      |
        //           10      |               |
        //                   +---------------+
        //
        // The graph is represented as an adjacency list where each index,
        // corresponding to a node value, has a list of outgoing edges.
        // Chosen for its efficiency.
        let graph = vec![
            // Node 0
            vec![Edge { node: 2, cost: 10 }, Edge { node: 1, cost: 1 }],
            // Node 1
            vec![Edge { node: 3, cost: 2 }],
            // Node 2
            vec![
                Edge { node: 1, cost: 1 },
                Edge { node: 3, cost: 3 },
                Edge { node: 4, cost: 1 },
            ],
            // Node 3
            vec![Edge { node: 4, cost: 2 }, Edge { node: 0, cost: 7 }],
            // Node 4
            vec![],
        ];

        // Assert all scenarios to be sure this is understood well
        assert_eq!(shortest_path(&graph, 0, 0), Some(0));
        assert_eq!(shortest_path(&graph, 0, 1), Some(1));
        assert_eq!(shortest_path(&graph, 0, 2), Some(10));
        assert_eq!(shortest_path(&graph, 0, 3), Some(3));
        assert_eq!(shortest_path(&graph, 0, 4), Some(5));
        assert_eq!(shortest_path(&graph, 1, 0), Some(9));
        assert_eq!(shortest_path(&graph, 1, 1), Some(0));
        assert_eq!(shortest_path(&graph, 1, 2), Some(19));
        assert_eq!(shortest_path(&graph, 1, 3), Some(2));
        assert_eq!(shortest_path(&graph, 1, 4), Some(4));
        assert_eq!(shortest_path(&graph, 2, 0), Some(10));
        assert_eq!(shortest_path(&graph, 2, 1), Some(1));
        assert_eq!(shortest_path(&graph, 2, 2), Some(0));
        assert_eq!(shortest_path(&graph, 2, 3), Some(3));
        assert_eq!(shortest_path(&graph, 2, 4), Some(1));
        assert_eq!(shortest_path(&graph, 3, 0), Some(7));
        assert_eq!(shortest_path(&graph, 3, 1), Some(8));
        assert_eq!(shortest_path(&graph, 3, 2), Some(17));
        assert_eq!(shortest_path(&graph, 3, 3), Some(0));
        assert_eq!(shortest_path(&graph, 3, 4), Some(2));
        assert_eq!(shortest_path(&graph, 4, 0), None);
        assert_eq!(shortest_path(&graph, 4, 1), None);
        assert_eq!(shortest_path(&graph, 4, 2), None);
        assert_eq!(shortest_path(&graph, 4, 3), None);
        assert_eq!(shortest_path(&graph, 4, 4), Some(0));
    }
}
