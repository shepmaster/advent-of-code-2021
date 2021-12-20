#![feature(array_windows)]
#![feature(int_abs_diff)]

use itertools::Itertools;
use petgraph::{algo::astar, graphmap::DiGraphMap};
use std::collections::BTreeSet;

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", n_unique_beacons(INPUT));
    println!("part2: {}", max_manhattan_distance_of_sensors(INPUT));
}

fn n_unique_beacons(s: &str) -> usize {
    let sensors = parse_sensors(s);
    let potential_connections = potential_connections(&sensors);
    let connections = valid_connections(&potential_connections, &sensors);
    let graph = connection_graph(&connections);
    let merged = merge(&sensors, &graph);

    merged.len()
}

fn max_manhattan_distance_of_sensors(s: &str) -> u32 {
    let sensors = parse_sensors(s);
    let potential_connections = potential_connections(&sensors);
    let connections = valid_connections(&potential_connections, &sensors);
    let graph = connection_graph(&connections);
    let merged = merge_sensors(&graph);

    merged
        .iter()
        .permutations(2)
        .map(|p| manhattan_distance(*p[0], *p[1]))
        .max()
        .expect("No Manhattan distance")
}

type Sensors = Vec<Beacons>;
type SensorIdx = usize;
type Beacons = Vec<Coord>;
type Coord = [i32; 3];
type Translation = [i32; 3];
type Rotation = fn(Coord) -> Coord;

type PotentialConnections = BTreeSet<(SensorIdx, SensorIdx)>;
type Connection = (SensorIdx, SensorIdx, Rotation, Coord, Coord);
type Connections = Vec<Connection>;
type ConnectionGraph = DiGraphMap<usize, (Rotation, Translation)>;

// https://www.reddit.com/r/adventofcode/comments/rk0fyk/
const ROTATIONS: &[Rotation] = &[
    // x is facing x
    |[x, y, z]| [x, y, z],
    |[x, y, z]| [x, -z, y],
    |[x, y, z]| [x, -y, -z],
    |[x, y, z]| [x, z, -y],
    // x is facing -x
    |[x, y, z]| [-x, -y, z],
    |[x, y, z]| [-x, -z, -y],
    |[x, y, z]| [-x, y, -z],
    |[x, y, z]| [-x, z, y],
    // x is facing y
    |[x, y, z]| [-z, x, -y],
    |[x, y, z]| [y, x, -z],
    |[x, y, z]| [z, x, y],
    |[x, y, z]| [-y, x, z],
    // x is facing -y
    |[x, y, z]| [z, -x, -y],
    |[x, y, z]| [y, -x, z],
    |[x, y, z]| [-z, -x, y],
    |[x, y, z]| [-y, -x, -z],
    // x is facing z
    |[x, y, z]| [-y, -z, x],
    |[x, y, z]| [z, -y, x],
    |[x, y, z]| [y, z, x],
    |[x, y, z]| [-z, y, x],
    // x is facing -z
    |[x, y, z]| [z, y, -x],
    |[x, y, z]| [-y, z, -x],
    |[x, y, z]| [-z, -y, -x],
    |[x, y, z]| [y, -z, -x],
];

fn parse_sensors(s: &str) -> Sensors {
    let mut lines = s.lines().peekable();

    let mut sensors = vec![];
    while lines.peek().is_some() {
        let beacons = lines
            .by_ref()
            .skip(1)
            .map(|l| l.trim())
            .take_while(|l| !l.is_empty())
            .map(|l| {
                l.split(',')
                    .map(|d| d.parse().expect("Invalid digit"))
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Must have 3 elements")
            })
            .collect();
        sensors.push(beacons);
    }
    sensors
}

// Use the distances between all the nodes to estimate if it's likely
// that two sensors overlap
fn potential_connections(sensors: &Sensors) -> BTreeSet<(SensorIdx, SensorIdx)> {
    // Do we need to worry about duplicate distances?
    let distances: Vec<_> = sensors.iter().map(|s| all_distances(s)).collect();

    let mut potential_connections = BTreeSet::new();
    for (ai, a) in distances.iter().enumerate() {
        for (bi, b) in distances.iter().enumerate().skip(ai) {
            if ai == bi {
                continue;
            }

            let overlap = a.intersection(b).count();
            if overlap >= 66 {
                // TODO: how'd I get 66? It just "looked good"
                potential_connections.insert((ai, bi));
                potential_connections.insert((bi, ai));
            }
        }
    }
    potential_connections
}

// Brute-force each potential sensor pair by translating each souce
// beacon to the origin, rotating the candidate beacons in all 24
// ways, and then translating each candidate beacon to the origin.
fn valid_connections(
    potential_connections: &PotentialConnections,
    sensors: &Sensors,
) -> Connections {
    let mut connections = vec![];
    'connection: for &(s1, s2) in potential_connections {
        let sensor1 = &sensors[s1];
        let sensor2 = &sensors[s2];

        for &pt1 in sensor1 {
            let sensor1_translated_by_pt1: BTreeSet<_> =
                translate_all(sensor1, negate(pt1)).collect();
            for &rotation in ROTATIONS {
                let rotated_sensor2: Vec<_> = rotate_all(sensor2, rotation).collect();

                for &pt2 in &rotated_sensor2 {
                    let rotated_sensor2_translated_by_pt2: BTreeSet<_> =
                        translate_all(&rotated_sensor2, negate(pt2)).collect();

                    let count = sensor1_translated_by_pt1
                        .intersection(&rotated_sensor2_translated_by_pt2)
                        .count();

                    if count >= 12 {
                        connections.push((s1, s2, rotation, pt1, pt2));
                        continue 'connection;
                    }
                }
            }
        }
    }
    connections
}

// Build a graph of rotation/translation transformations between the
// sensors.
fn connection_graph(connections: &Connections) -> ConnectionGraph {
    let mut graph = DiGraphMap::new();
    for &(from, to, rotation, a, b) in connections {
        let t = sub(a, b);
        graph.add_edge(to, from, (rotation, t));
    }
    graph
}

// Transform all beacons to sensor #0.
fn merge(sensors: &Sensors, graph: &ConnectionGraph) -> BTreeSet<Coord> {
    let mut all_beacons = BTreeSet::from_iter(sensors[0].iter().copied());
    for (idx, beacons) in sensors.iter().enumerate().skip(1) {
        let mut beacons = beacons.clone();

        let (_, path) = astar(&graph, idx, |n| n == 0, |_| 1, |_| 1).expect("No path");

        for &[a, b] in path.array_windows() {
            let &(rotation, translation) = graph.edge_weight(a, b).expect("edge missing");
            let rotated: Vec<_> = rotate_all(&beacons, rotation).collect();
            beacons = translate_all(&rotated, translation).collect();
        }

        all_beacons.extend(beacons);
    }
    all_beacons
}

// Transform all sensors to sensor #0.
fn merge_sensors(graph: &ConnectionGraph) -> BTreeSet<Coord> {
    let mut all_sensors = BTreeSet::new();

    for sensor in graph.nodes() {
        let (_, path) = astar(&graph, sensor, |n| n == 0, |_| 1, |_| 1).expect("No path");
        let mut sensor = [0, 0, 0];

        for &[a, b] in path.array_windows() {
            let &(rotation, translation) = graph.edge_weight(a, b).expect("edge missing");

            sensor = rotation(sensor);
            sensor = translate(sensor, translation)
        }

        all_sensors.insert(sensor);
    }
    all_sensors
}

fn all_distances(mut beacons: &[Coord]) -> BTreeSet<i32> {
    let mut distances = BTreeSet::new();
    while let Some((&h, t)) = beacons.split_first() {
        distances.extend(t.iter().map(|&c| distance_magnitude(h, c)));
        beacons = t;
    }
    distances
}

fn distance_magnitude([ax, ay, az]: Coord, [bx, by, bz]: Coord) -> i32 {
    (ax - bx).pow(2) + (ay - by).pow(2) + (az - bz).pow(2)
}

fn rotate_all(beacons: &Beacons, rotation: Rotation) -> impl Iterator<Item = Coord> + '_ {
    beacons.iter().copied().map(rotation)
}

fn translate_all(coords: &[Coord], by: Coord) -> impl Iterator<Item = Coord> + '_ {
    coords.iter().map(move |&c| translate(c, by))
}

fn translate([x, y, z]: Coord, [x0, y0, z0]: Coord) -> Coord {
    [x + x0, y + y0, z + z0]
}

fn sub([ax, ay, az]: Coord, [bx, by, bz]: Coord) -> Coord {
    [ax - bx, ay - by, az - bz]
}

fn negate([x, y, z]: Coord) -> Coord {
    [-x, -y, -z]
}

fn manhattan_distance([ax, ay, az]: Coord, [bx, by, bz]: Coord) -> u32 {
    ax.abs_diff(bx) + ay.abs_diff(by) + az.abs_diff(bz)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test-input");

    #[test]
    fn test_part1() {
        assert_eq!(79, n_unique_beacons(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3621, max_manhattan_distance_of_sensors(TEST_INPUT));
    }
}
