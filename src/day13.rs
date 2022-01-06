use std::collections::HashSet;

use pathfinding::prelude::{astar, absdiff};

use crate::selfprint::SelfPrint;

fn is_wall(p: (u64, u64), fav_num: u64) -> bool {
    let mut num = p.0 * p.0 + 3 * p.0 + 2 * p.0 * p.1 + p.1 + p.1 * p.1 + fav_num;
    let mut one_count = 0;
    while num > 0 {
        if num & 1 == 1 {
            one_count += 1;
        }
        num = num >> 1;
    }
    one_count % 2 == 1
}

pub fn part1(input: String) {
    let fav_num = input.parse::<u64>().unwrap_or(0);
    let destination = (31u64, 39u64);
    let path = astar(
        &(1u64, 1u64),
        |point| {
            let mut neighbors = vec![];
            for (x, y) in [
                (point.0.checked_sub(1), Some(point.1)),
                (Some(point.0), point.1.checked_sub(1)),
                (Some(point.0 + 1), Some(point.1)),
                (Some(point.0), Some(point.1 + 1))
            ] {
                if let (Some(x), Some(y)) = (x, y) {
                    if !is_wall((x, y), fav_num) {
                        neighbors.push(((x, y), 1));
                    }
                }
            }
            neighbors
        },
        |point| absdiff(destination.0, point.0) + absdiff(destination.1, point.1),
        |point| point == &destination
    );
    path.expect("No min path found").1.print();
}

pub fn part2(input: String) {
    let fav_num = input.parse::<u64>().unwrap_or(0);
    let mut seen = HashSet::<(u64, u64)>::new();
    seen.insert((1, 1));

    let mut next_points = seen.clone();
    let mut steps = 1;
    while steps <= 50 {
        let clone = next_points.clone();
        next_points.clear();
        for point in clone {
            for (x, y) in [
                (point.0.checked_sub(1), Some(point.1)),
                (Some(point.0), point.1.checked_sub(1)),
                (Some(point.0 + 1), Some(point.1)),
                (Some(point.0), Some(point.1 + 1))
            ] {
                if let (Some(x), Some(y)) = (x, y) {
                    if !seen.contains(&(x, y)) && !is_wall((x, y), fav_num) {
                        seen.insert((x, y));
                        next_points.insert((x, y));
                    }
                }
            }
        }
        steps += 1;
    }

    seen.len().print();
}
