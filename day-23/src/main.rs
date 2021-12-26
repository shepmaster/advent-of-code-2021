#![feature(map_first_last)]

use petgraph::graph::{NodeIndex, UnGraph};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
    sync::Arc,
};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", minimum_energy_to_organize(INPUT));
}

fn minimum_energy_to_organize(s: &str) -> usize {
    let g = parse_graph(s);
    let state = find_minimum_cost(g).expect("No minimum cost found");
    state.cost
}

fn parse_graph(s: &str) -> MyGraph {
    use {Amphipod::*, Node::*};

    let mut starts = vec![];

    for line in s.lines() {
        for c in line.trim().chars() {
            let a = match c {
                'A' => Amber,
                'B' => Bronze,
                'C' => Copper,
                'D' => Desert,
                _ => continue,
            };
            starts.push(a);
        }
    }

    let starts: Box<[Amphipod; 8]> = starts
        .into_boxed_slice()
        .try_into()
        .expect("incorrect number of starting amphipods");

    let mut g = MyGraph::new_undirected();

    let h1 = g.add_node(Hallway(None));
    let h2 = g.add_node(Hallway(None));
    let i1 = g.add_node(Intersection(None));
    let h3 = g.add_node(Hallway(None));
    let i2 = g.add_node(Intersection(None));
    let h4 = g.add_node(Hallway(None));
    let i3 = g.add_node(Intersection(None));
    let h5 = g.add_node(Hallway(None));
    let i4 = g.add_node(Intersection(None));
    let h6 = g.add_node(Hallway(None));
    let h7 = g.add_node(Hallway(None));

    let ra1 = g.add_node(Room(Amber, Some(starts[0])));
    let rb1 = g.add_node(Room(Bronze, Some(starts[1])));
    let rc1 = g.add_node(Room(Copper, Some(starts[2])));
    let rd1 = g.add_node(Room(Desert, Some(starts[3])));

    let ra2 = g.add_node(Room(Amber, Some(starts[4])));
    let rb2 = g.add_node(Room(Bronze, Some(starts[5])));
    let rc2 = g.add_node(Room(Copper, Some(starts[6])));
    let rd2 = g.add_node(Room(Desert, Some(starts[7])));

    g.add_edge(h1, h2, ());
    g.add_edge(h2, i1, ());
    g.add_edge(i1, h3, ());
    g.add_edge(h3, i2, ());
    g.add_edge(i2, h4, ());
    g.add_edge(h4, i3, ());
    g.add_edge(i3, h5, ());
    g.add_edge(h5, i4, ());
    g.add_edge(i4, h6, ());
    g.add_edge(h6, h7, ());

    g.add_edge(i1, ra1, ());
    g.add_edge(i2, rb1, ());
    g.add_edge(i3, rc1, ());
    g.add_edge(i4, rd1, ());

    g.add_edge(ra1, ra2, ());
    g.add_edge(rb1, rb2, ());
    g.add_edge(rc1, rc2, ());
    g.add_edge(rd1, rd2, ());

    g
}

fn find_minimum_cost(g: MyGraph) -> Option<State> {
    let mut graphs = BinaryHeap::from_iter([Reverse(State::new(g))]);
    let mut already_seen = BTreeMap::new();

    #[allow(unused_variables)]
    let mut n_explored = 0;

    while let Some(Reverse(s)) = graphs.pop() {
        // Cheapest complete path we've seen
        if s.is_complete() {
            return Some(s);
        }

        let parent = Arc::new(s.clone());
        let State { graph, cost, .. } = s;

        let starting_nodes = graph
            .node_indices()
            .flat_map(|n| graph[n].amphipod().map(|&a| (n, a)));

        for (n0, a0) in starting_nodes {
            let paths = all_valid_paths(&graph, n0, a0);

            // Calculate cost of this path
            for path in &paths {
                let steps = path.len() - 1;
                let cost_delta = steps * a0.cost();
                let cost = cost + cost_delta;

                let last = *path.last().unwrap();

                let mut graph = graph.clone();

                let from = &mut graph[n0];
                let a = from.take();
                let to = &mut graph[last];
                to.put(a);

                let parent = Some(Arc::clone(&parent));

                let next_state = State {
                    parent,
                    graph,
                    cost,
                };

                use std::collections::btree_map::Entry::*;
                let encoded = next_state.encode();
                match already_seen.entry(encoded) {
                    Vacant(e) => {
                        e.insert(cost);
                    }
                    Occupied(mut e) => {
                        if cost < *e.get() {
                            e.insert(cost);
                        } else {
                            continue;
                        }
                    }
                }

                graphs.push(Reverse(next_state));
            }
        }

        n_explored += 1;
    }

    None
}

type Path = Vec<NodeIndex>;

fn all_valid_paths(graph: &MyGraph, n0: NodeIndex, a0: Amphipod) -> Vec<Path> {
    use petgraph::visit::{
        depth_first_search,
        Control::{self, *},
        DfsEvent::*,
    };
    use Node::*;

    let node0 = &graph[n0];

    if let Room(e, _) = node0 {
        if e == &a0 && room_is_effective_end(graph, n0) {
            return vec![];
        }
    }

    let mut paths = vec![];
    let mut path = vec![];

    depth_first_search(graph, Some(n0), |event| -> Control<()> {
        match event {
            Discover(n, _) => {
                path.push(n);
                Continue
            }
            TreeEdge(_, n) => match graph[n] {
                Hallway(Some(_)) => Prune,
                Hallway(None) => Continue,
                Intersection(Some(_)) => Prune,
                Intersection(None) => Continue,
                Room(_, Some(_)) => Prune,
                Room(_, None) => Continue,
            },
            BackEdge(_, _) => Prune,
            CrossForwardEdge(_, _) => todo!(),
            Finish(_, _) => {
                paths.push(path.clone());
                path.pop();
                Continue
            }
        }
    });

    // TODO: don't move into room1 if room2 is open

    paths.retain(|path| -> bool {
        // Doesn't move anywhere
        if path.len() <= 1 {
            return false;
        }

        let l = *path.last().unwrap();
        let n = &graph[l];

        match (node0, n) {
            (_, Intersection(_)) => false,
            (Intersection(_), _) => unreachable!(),

            (Hallway(_), Hallway(_)) => false,
            (_, Room(b, _)) if &a0 != b => false,
            (_, Room(_, _)) => room_is_effective_end(graph, l),
            (Room(e, Some(a)), Hallway(_)) => {
                let is_room_2 = graph.neighbors(n0).count() == 1;
                !(is_room_2 && e == a)
            }
            (Room(_, None), _) => unreachable!(),
        }
    });

    paths
}

fn room_is_effective_end(
    graph: &petgraph::Graph<Node, (), petgraph::Undirected>,
    n0: NodeIndex,
) -> bool {
    use Node::*;

    let mut count = 0;
    let mut neighbor_room_correct = false;

    for n in graph.neighbors(n0) {
        count += 1;
        neighbor_room_correct =
            neighbor_room_correct || matches!(graph[n], Room(e, Some(a)) if e == a);
    }

    count == 1 || // end room
        neighbor_room_correct
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn cost(self) -> usize {
        use Amphipod::*;

        match self {
            Amber => 1,
            Bronze => 10,
            Copper => 100,
            Desert => 1000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
    Hallway(Option<Amphipod>),
    Intersection(Option<Amphipod>),
    Room(Amphipod, Option<Amphipod>),
}

impl Node {
    fn take(&mut self) -> Option<Amphipod> {
        self.amphipod_mut().take()
    }

    fn put(&mut self, a: Option<Amphipod>) {
        *self.amphipod_mut() = a;
    }

    fn amphipod(&self) -> Option<&Amphipod> {
        use Node::*;

        match self {
            Hallway(a) => a,
            Intersection(a) => a,
            Room(_, a) => a,
        }
        .as_ref()
    }

    fn amphipod_mut(&mut self) -> &mut Option<Amphipod> {
        use Node::*;

        match self {
            Hallway(a) => a,
            Intersection(a) => a,
            Room(_, a) => a,
        }
    }
}

type MyGraph = UnGraph<Node, ()>;
type EncodedGraph = Box<[Option<Amphipod>; 19]>;

#[allow(dead_code)]
fn dump_graph(g: &MyGraph) {
    use Amphipod::*;

    let e = encode_graph(g);

    let chr = |v| match v {
        Some(Amber) => 'A',
        Some(Bronze) => 'B',
        Some(Copper) => 'C',
        Some(Desert) => 'D',
        None => '.',
    };

    eprintln!(
        "{}{}{}{}{}{}{}{}{}{}{}",
        chr(e[0]),
        chr(e[1]),
        chr(e[2]),
        chr(e[3]),
        chr(e[4]),
        chr(e[5]),
        chr(e[6]),
        chr(e[7]),
        chr(e[8]),
        chr(e[9]),
        chr(e[10]),
    );

    eprintln!(
        "  {} {} {} {}",
        chr(e[11]),
        chr(e[12]),
        chr(e[13]),
        chr(e[14]),
    );

    eprintln!(
        "  {} {} {} {}",
        chr(e[15]),
        chr(e[16]),
        chr(e[17]),
        chr(e[18]),
    );
}

#[derive(Debug, Clone)]
struct State {
    #[allow(dead_code)]
    parent: Option<Arc<Self>>,
    graph: MyGraph,
    cost: usize,
}

impl State {
    fn new(graph: MyGraph) -> Self {
        Self {
            parent: None,
            graph,
            cost: 0,
        }
    }

    fn is_complete(&self) -> bool {
        self.graph.node_weights().all(|n| match n {
            Node::Hallway(_) => true,
            Node::Intersection(_) => true,
            Node::Room(e, Some(a)) => e == a,
            Node::Room(_, None) => false,
        })
    }

    fn encode(&self) -> EncodedGraph {
        encode_graph(&self.graph)
    }
}

fn encode_graph(g: &MyGraph) -> EncodedGraph {
    g.node_weights()
        .map(|n| n.amphipod().copied())
        .collect::<Vec<_>>()
        .into_boxed_slice()
        .try_into()
        .expect("Wrong graph size")
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part1() {
        assert_eq!(12521, minimum_energy_to_organize(TEST_INPUT));
    }
}
