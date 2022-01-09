use std::collections::{HashSet, HashMap};

use itertools::Itertools;
use pathfinding::prelude::{astar, absdiff, dijkstra};

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    let mut walls = HashSet::new();
    let mut nodes = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                nodes.insert(c, (x, y));
            } else if c == '#' {
                walls.insert((x, y));
            }
        }
    }

    let edges = nodes
        .iter()
        .tuple_combinations()
        .map(|((n1, p1), (n2, p2))| {
            let path = astar(
                p1,
                |pos| {
                    [
                        (pos.0 - 1, pos.1),
                        (pos.0 + 1, pos.1),
                        (pos.0, pos.1 - 1),
                        (pos.0, pos.1 + 1)
                    ]
                    .iter()
                    .flat_map(|(x, y)|
                        if !walls.contains(&(*x, *y)) {
                            Some(((*x, *y), 1))
                        } else {
                            None
                        }
                    )
                    .collect_vec()
                },
                |pos| absdiff(pos.0, p2.0) + absdiff(pos.1, p2.1),
                |pos| pos == p2
            );
            let weight = path.map(|x| x.1).unwrap_or_default();
            ((n1, n2), weight)
        })
        .collect::<HashMap<_, _>>();

    println!("===Part 1===");
    dijkstra(
        &vec!['0'],
        |visited| {
            let last = visited.last().unwrap_or(&'0');
            nodes
                .keys()
                .filter(|n| !visited.contains(n))
                .map(|n| {
                    let mut new_visited = visited.clone();
                    new_visited.push(*n);
                    let weight = edges.get(&(last, n)).or(edges.get(&(n, last))).unwrap_or(&0);
                    (new_visited, *weight)
                })
                .collect_vec()
        },
        |visited| visited.len() == nodes.len()
    )
    .expect("No min found")
    .1
    .print();

    println!("===Part 2===");
    dijkstra(
        &vec!['0'],
        |visited| {
            let last = visited.last().unwrap_or(&'0');
            nodes
                .keys()
                .filter(|n| !visited.contains(n) || visited.len() == nodes.len() && n == &&'0')
                .map(|n| {
                    let mut new_visited = visited.clone();
                    new_visited.push(*n);
                    let weight = edges.get(&(last, n)).or(edges.get(&(n, last))).unwrap_or(&0);
                    (new_visited, *weight)
                })
                .collect_vec()
        },
        |visited| visited.len() == nodes.len() + 1
    )
    .expect("No min found")
    .1
    .print();
}
