#![feature(map_first_last)]
#![feature(array_chunks)]
#![feature(slice_as_chunks)]

use petgraph::graph::{NodeIndex, UnGraph};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
    sync::Arc,
};

const INPUT_0: &str = include_str!("../input-0");
const INPUT_1: &str = include_str!("../input-1");

fn main() {
    println!("part1: {}", minimum_energy_to_organize::<Folded>(INPUT_0));
    println!("part2: {}", minimum_energy_to_organize::<Unfolded>(INPUT_1));
}

fn minimum_energy_to_organize<K>(s: &str) -> usize
where
    K: Kind,
{
    let g = K::parse_graph(s);
    let state = find_minimum_cost::<K>(g).expect("No minimum cost found");
    state.cost
}

trait Kind {
    const N_AMPHIPODS: usize;
    type EncodedGraph: PartialOrd + Ord + PartialEq + Eq;

    fn parse_graph(s: &str) -> MyGraph;

    fn encode_graph(g: &MyGraph) -> Self::EncodedGraph;

    fn dump_graph(g: &MyGraph);
}

struct Folded;

impl Kind for Folded {
    const N_AMPHIPODS: usize = 8;
    type EncodedGraph = Box<[Option<Amphipod>; 19]>;

    fn parse_graph(s: &str) -> MyGraph {
        parse_graph::<8>(s)
    }

    fn encode_graph(g: &MyGraph) -> Self::EncodedGraph {
        encode_graph::<19>(g)
    }

    fn dump_graph(g: &MyGraph) {
        dump_graph(&*Self::encode_graph(g))
    }
}

struct Unfolded;

impl Kind for Unfolded {
    const N_AMPHIPODS: usize = 16;
    type EncodedGraph = Box<[Option<Amphipod>; 27]>;

    fn parse_graph(s: &str) -> MyGraph {
        parse_graph::<16>(s)
    }

    fn encode_graph(g: &MyGraph) -> Self::EncodedGraph {
        encode_graph::<27>(g)
    }

    fn dump_graph(g: &MyGraph) {
        dump_graph(&*Self::encode_graph(g))
    }
}

fn parse_graph<const N: usize>(s: &str) -> MyGraph {
    let starts = extract_amphipods::<N>(s);

    let mut g = MyGraph::new_undirected();

    let mut prev = add_hallway_and_intersections(&mut g);

    for amphipods in starts.array_chunks() {
        prev = add_room_level(&mut g, amphipods, prev);
    }

    g
}

fn extract_amphipods<const N: usize>(s: &str) -> Box<[Amphipod; N]> {
    use Amphipod::*;

    s.lines()
        .flat_map(|l| l.trim().chars())
        .flat_map(|c| match c {
            'A' => Some(Amber),
            'B' => Some(Bronze),
            'C' => Some(Copper),
            'D' => Some(Desert),
            _ => None,
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
        .try_into()
        .expect("incorrect number of starting amphipods")
}

fn add_hallway_and_intersections(g: &mut MyGraph) -> [NodeIndex; 4] {
    use Node::*;

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

    [i1, i2, i3, i4]
}

fn add_room_level(
    g: &mut MyGraph,
    amphipods: &[Amphipod; 4],
    prev: [NodeIndex; 4],
) -> [NodeIndex; 4] {
    use {Amphipod::*, Node::*};

    let ra = g.add_node(Room(Amber, Some(amphipods[0])));
    let rb = g.add_node(Room(Bronze, Some(amphipods[1])));
    let rc = g.add_node(Room(Copper, Some(amphipods[2])));
    let rd = g.add_node(Room(Desert, Some(amphipods[3])));

    g.add_edge(prev[0], ra, ());
    g.add_edge(prev[1], rb, ());
    g.add_edge(prev[2], rc, ());
    g.add_edge(prev[3], rd, ());

    [ra, rb, rc, rd]
}

fn find_minimum_cost<K>(g: MyGraph) -> Option<State>
where
    K: Kind,
{
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
                let encoded = next_state.encode::<K>();
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

fn dump_graph(encoded: &[Option<Amphipod>]) {
    use Amphipod::*;

    let (hallway, rooms) = encoded.split_at(11);
    let hallway: &[_; 11] = hallway.try_into().unwrap();
    let (rooms, _) = rooms.as_chunks::<4>();

    let chr = |v| match v {
        Some(Amber) => 'A',
        Some(Bronze) => 'B',
        Some(Copper) => 'C',
        Some(Desert) => 'D',
        None => '.',
    };

    eprintln!(
        "{}{}{}{}{}{}{}{}{}{}{}",
        chr(hallway[0]),
        chr(hallway[1]),
        chr(hallway[2]),
        chr(hallway[3]),
        chr(hallway[4]),
        chr(hallway[5]),
        chr(hallway[6]),
        chr(hallway[7]),
        chr(hallway[8]),
        chr(hallway[9]),
        chr(hallway[10]),
    );

    for room_row in rooms {
        eprintln!(
            "  {} {} {} {}",
            chr(room_row[0]),
            chr(room_row[1]),
            chr(room_row[2]),
            chr(room_row[3]),
        );
    }
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

    fn encode<K>(&self) -> K::EncodedGraph
    where
        K: Kind,
    {
        K::encode_graph(&self.graph)
    }
}

fn encode_graph<const N: usize>(g: &MyGraph) -> Box<[Option<Amphipod>; N]> {
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

    const TEST_INPUT_0: &str = include_str!("../test-input-0");
    const TEST_INPUT_1: &str = include_str!("../test-input-1");

    #[test]
    fn test_part1() {
        assert_eq!(12521, minimum_energy_to_organize::<Folded>(TEST_INPUT_0));
    }

    #[test]
    fn test_part2() {
        assert_eq!(44169, minimum_energy_to_organize::<Unfolded>(TEST_INPUT_1));
    }
}
