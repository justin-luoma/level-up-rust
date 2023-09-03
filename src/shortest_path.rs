use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &[(Node, Node, Cost)]) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list
                .entry(source)
                .or_insert_with(Vec::new);

            destinations.push((destination, cost));

            nodes.insert(source);
            nodes.insert(destination);
        }

        Graph {
            edges: adjacency_list,
            nodes,
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct Visit {
    distance: Cost,
    position: Node,
    history: Vec<Node>,
}

impl Visit {
    fn new(position: Node, distance: Cost, history: Vec<Node>) -> Self {
        Self {
            distance,
            position,
            history
        }
    }

    fn from_start(start: Node) -> Self {
        Visit::new(start, 0, Default::default())
    }
}

impl PartialOrd<Self> for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialEq for Visit {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    let mut distances = HashMap::from([(start, 0)]);
    let mut visit = BinaryHeap::new();
    visit.push(Visit::from_start(start));

    while let Some(Visit { distance, position, mut history}) = visit.pop() {
        if position == goal {
            history.push(goal);
            return Some((history, distance));
        }

        if let Some(neighbors) = g.edges.get(&position) {
            for (neighbor, cost) in neighbors {
                let new_distance = distance + cost;
                let is_shorter = distances
                    .get(neighbor)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    let mut next = Visit::new(*neighbor, new_distance, history.clone());
                    next.history.push(position);
                    visit.push(next);
                    distances.insert(*neighbor, new_distance);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_graph() {
        let edge_list = include!("large_graph.in");
        let g = Graph::from_edge_list(&edge_list);

        let path = shortest_path(&g, 1000, 9000);
        assert!(path.is_some());
        assert_eq!(path.unwrap().1, 24);
    }
}
