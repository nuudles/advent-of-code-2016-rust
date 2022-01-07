use std::collections::HashSet;

use itertools::iproduct;

use crate::selfprint::SelfPrint;

fn safe_tile_count(input: &String, rows: usize) -> usize {
    let x_max = input.len();
    let mut traps = input
        .chars()
        .enumerate()
        .fold(HashSet::new(), |mut result, (x, c)| {
            if c == '^' {
                result.insert((x, 0usize));
            }
            result
        });
    let mut trap_count = traps.len();
    for y in 1..rows {
        let mut new_traps = HashSet::new();
        for x in 0..x_max {
            let is_left_trap = x.checked_sub(1).map(|x| traps.contains(&(x, y - 1))).unwrap_or(false);
            let is_center_trap = traps.contains(&(x, y - 1));
            let is_right_trap = traps.contains(&(x + 1, y - 1));
            if (is_left_trap && is_center_trap && !is_right_trap) || 
                (!is_left_trap && is_center_trap && is_right_trap) ||
                (is_left_trap && !is_center_trap && !is_right_trap) ||
                (!is_left_trap && !is_center_trap && is_right_trap) {
                    new_traps.insert((x, y));
            }
        }
        trap_count += new_traps.len();
        traps = new_traps;
    }
    x_max * rows - trap_count
}
/*
    let mut new_traps = HashSet::new();
    for (y, x) in iproduct!(1..rows, 0..x_max) {
        let is_left_trap = x.checked_sub(1).map(|x| traps.contains(&(x, y - 1))).unwrap_or(false);
        let is_center_trap = traps.contains(&(x, y - 1));
        let is_right_trap = traps.contains(&(x + 1, y - 1));
        if (is_left_trap && is_center_trap && !is_right_trap) || 
            (!is_left_trap && is_center_trap && is_right_trap) ||
            (is_left_trap && !is_center_trap && !is_right_trap) ||
            (!is_left_trap && !is_center_trap && is_right_trap) {
                new_traps.insert((x, y));
        }
        traps = new_traps;
    }
     */

pub fn part1(input: String) {
    safe_tile_count(&input, 40).print();
}

pub fn part2(input: String) {
    safe_tile_count(&input, 400000).print();
}
