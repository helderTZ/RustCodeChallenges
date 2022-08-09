use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

// from solution
use std::cmp::Ordering;
use std::collections::BinaryHeap;

type Node = usize;
type Cost = usize;

struct Graph {
    edges: HashMap<Node, Vec<(Node, Cost)>>,
    nodes: HashSet<Node>,
}

impl Graph {
    fn from_edge_list(edge_list: &Vec<(Node, Node, Cost)>) -> Self {
        let mut adjacency_list: HashMap<Node, Vec<(Node, Cost)>> = HashMap::new();
        let mut nodes = HashSet::new();

        for &(source, destination, cost) in edge_list.iter() {
            let destinations = adjacency_list
                .entry(source)
                .or_insert_with(|| Vec::new());

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

// from solution
#[derive(PartialEq, Eq)]
struct Step {
    cost: Cost,
    position: Node,
    history: Vec<Node>,
}

// from solution
impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost)
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// from solution
impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


// from solution
fn shortest_path_sol(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Step { cost: 0, position: start, history: vec![] });

    let mut distances: HashMap<Node, Cost> = g.nodes
        .iter()
        .map(|&x| {
            if x == start {
                (x, 0)
            } else {
                (x, usize::MAX)
            }
        })
        .collect();

    while let Some(Step { cost, position, mut history }) = priority_queue.pop() {
        if position == goal {
            history.push(goal);
            return Some((history, cost));
        }

        if let Some(destinations) = &g.edges.get(&position) {
            for &(next_dest, next_cost) in destinations.iter() {
                if next_cost < distances[&next_dest] {
                    let mut next_step = Step {
                        position: next_dest,
                        cost: cost + next_cost,
                        history: history.clone(),
                    };

                    next_step.history.push(position);
                    priority_queue.push(next_step);

                    // fuckery to update value in hashmap
                    if let Some(old_cost) = distances.get_mut(&next_dest) {
                        *old_cost = next_cost;
                    }
                }
            }
        }
    }

    // No paths between start and goal
    None
}


fn get_neighbors<'a>(g: &'a Graph, node: &Node) -> &'a Vec<(Node, Cost)> {
    g.edges.get(node).unwrap()
}


fn find_lowest_cost_node(priority_queue: &mut Vec<(Node, Cost)>) -> Node {
    let mut lowest_cost = usize::MAX;
    let mut lowest_cost_node: Option<Node> = None;
    let mut index = 0;
    let mut index_to_remove = 0;
    for (node, cost) in priority_queue.iter() {
        if *cost < lowest_cost {
            lowest_cost = *cost;
            lowest_cost_node = Some(*node);
            index_to_remove = index;
        }
        index += 1;
    }
    priority_queue.remove(index_to_remove);
    lowest_cost_node.unwrap()
}


// my implementation
fn shortest_path(g: &Graph, start: Node, goal: Node) -> Option<(Vec<Node>, Cost)> {
    // priority queue
    let mut priority_queue: Vec<(Node, Cost)> = vec![];

    // table with the cost of each node (every node starts with Inf except the first which is zero)
    let mut dist: Vec<Cost> = vec![usize::MAX; g.nodes.len()];
    dist[start] = 0;

    // table with the parent of each node (begins with all parents to None)
    let mut parents: HashMap<Node, Option<Node>> = HashMap::with_capacity(g.nodes.len());
    for node in g.nodes.iter() {
        parents.insert(*node, None);
    }

    // points to the current node
    let mut curr_node: Node = start;

    priority_queue.push((curr_node, 0));

    while !priority_queue.is_empty() {
        curr_node = find_lowest_cost_node(&mut priority_queue);
        if curr_node == goal {
            break;
        }
        for (neighbor, neighbor_cost) in get_neighbors(g, &curr_node) {
            if dist[*neighbor] > dist[curr_node] + *neighbor_cost {
                dist[*neighbor] = dist[curr_node] + *neighbor_cost;
                *parents.get_mut(neighbor).unwrap() = Some(curr_node);  // fuckery to replace value in hashmap
                priority_queue.push((*neighbor, dist[*neighbor]));
            }
        }
    }

    // reached goal node, recreate path from backtracking parents
    let mut path: Vec<Node> = vec![];
    let mut u: Option<Node> = Some(goal);
    if parents[&u.unwrap()].is_some() {
        while u.is_some() {
            path.push(u.unwrap());
            u = parents[&u.unwrap()];
        }
        let reverse_path: Vec<Node> = path.into_iter().rev().collect();
        return Some((reverse_path, dist[curr_node]));
    }

    None
}


fn main() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    let now = Instant::now();
    // if let Some((path, cost)) = shortest_path(&g, 1000, 9000) {
        if let Some((path, cost)) = shortest_path_sol(&g, 1000, 9000) {
        println!("1000->9000, {:?} {}", path, cost);
    };
    let elapsed: Duration = now.elapsed();
    println!("Took {:.2?}", elapsed);
}


#[test]
fn large_graph() {
    let edge_list = include!("large_graph.in");
    let g = Graph::from_edge_list(&edge_list);

    let path = shortest_path(&g, 1000, 9000);
    assert!(path.is_some());
    assert_eq!(path.unwrap().1, 24); 
}