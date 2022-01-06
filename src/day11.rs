use std::{collections::BTreeMap, cmp::{min, max}};

use pathfinding::prelude::astar;
use itertools::Itertools;
use regex::Regex;

use crate::selfprint::SelfPrint;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Item<'a> {
    Elevator,
    Generator(&'a str),
    Microchip(&'a str)
}

fn starting_floors<'a>(input: &'a String) -> BTreeMap<Item<'a>, usize> {
    let re = Regex::new("(?:([a-z]+) generator)|(?:([a-z]+)-compatible microchip)")
        .expect("Could not compile Regex");

    input
        .lines()
        .take(3)
        .enumerate()
        .fold(BTreeMap::new(), |mut map, (i, line)| {
            for capture in re.captures_iter(line) {
                if let Some(name) = capture.get(1).map(|g| g.as_str()) {
                    map.insert(Item::Generator(name), i + 1);
                } else if let Some(name) = capture.get(2).map(|g| g.as_str()) {
                    map.insert(Item::Microchip(name), i + 1);
                }
            }
            map
        })
}

fn can_move_to_floor(target: Item, state: &BTreeMap<Item, usize>, floor: usize) -> bool {
    match target {
        Item::Generator(name) => {
            let microchip_floor = state.get(&Item::Microchip(name)).unwrap_or(&0);
            if *microchip_floor == floor {
                return true;
            }
            !state.iter().any(|(i, f)| f == microchip_floor && matches!(i, Item::Generator(_)))
        },
        Item::Microchip(name) => {
            let generator_floor = state.get(&Item::Generator(name)).unwrap_or(&0);
            if *generator_floor == floor {
                return true;
            }
            !state.iter().any(|(i, f)| f == &floor && matches!(i, Item::Generator(_)))
        },
        Item::Elevator => true
    }
}

fn shortest_path(input: String, with_additional_items: bool) -> Option<usize> {
    let mut floors = starting_floors(&input);
    floors.insert(Item::Elevator, 1);
    if with_additional_items {
        floors.insert(Item::Generator("elerium"), 1);
        floors.insert(Item::Microchip("elerium"), 1);
        floors.insert(Item::Generator("dilithium"), 1);
        floors.insert(Item::Microchip("dilithium"), 1);
    }
    let path = astar(
        &floors,
        |state| {
            // println!("{:?}", state);
            let mut next_states = vec![];
            let current_floor = state.get(&Item::Elevator).unwrap_or(&0);
            let targets = state.iter().filter(|(_, f)| f == &current_floor).map(|t| t.0);
            for (target1, target2) in targets.tuple_combinations() {
                let prev_floor = max(current_floor - 1, 1);
                let next_floor = min(current_floor + 1, 4);
                match (target1, target2) {
                    (Item::Generator(generator_name), Item::Microchip(chip_name)) | 
                        (Item::Microchip(chip_name), Item::Generator(generator_name)) => {
                            if generator_name == chip_name {
                                if prev_floor != *current_floor {
                                    let mut new_state = state.clone();
                                    new_state.insert(*target1, prev_floor);
                                    new_state.insert(*target2, prev_floor);
                                    new_state.insert(Item::Elevator, prev_floor);
                                    next_states.push((new_state, 1));
                                }
                                if next_floor != *current_floor {
                                    let mut new_state = state.clone();
                                    new_state.insert(*target1, next_floor);
                                    new_state.insert(*target2, next_floor);
                                    new_state.insert(Item::Elevator, next_floor);
                                    next_states.push((new_state, 1));
                                }
                            }
                            continue;
                    },
                    _ => ()
                }
                if prev_floor != *current_floor &&
                    (matches!(target1, Item::Elevator) || matches!(target2, Item::Elevator)) &&
                    can_move_to_floor(*target1, state, prev_floor) && 
                    can_move_to_floor(*target2, state, prev_floor) {
                        let mut new_state = state.clone();
                        new_state.insert(*target1, prev_floor);
                        new_state.insert(*target2, prev_floor);
                        new_state.insert(Item::Elevator, prev_floor);
                        next_states.push((new_state, 1));
                }
                if next_floor != *current_floor && 
                    can_move_to_floor(*target1, state, next_floor) && 
                    can_move_to_floor(*target2, state, next_floor) {
                        let mut new_state = state.clone();
                        new_state.insert(*target1, next_floor);
                        new_state.insert(*target2, next_floor);
                        new_state.insert(Item::Elevator, next_floor);
                        next_states.push((new_state, 1));
                }
            }
            next_states
        },
        |state| state.values().map(|f| 4 - f).sum(),
        |state| state.values().all(|f| *f == 4)
    );
    path.map(|(_, c)| c)
}

pub fn part1(input: String) {
    shortest_path(input, false).unwrap_or(0).print();
}

pub fn part2(input: String) {
    // Took 16 minutes, but it got the right answer...
    shortest_path(input, true).unwrap_or(0).print();
}

#[cfg(test)]
mod tests {
    use super::shortest_path;

    #[test]
    fn test_shortest_path() {
        assert_eq!(
            shortest_path(
                r"The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
                The second floor contains a hydrogen generator.
                The third floor contains a lithium generator.
                The fourth floor contains nothing relevant.
                ".to_string(),
                false
            ),
            Some(11)
        )
    }
}

