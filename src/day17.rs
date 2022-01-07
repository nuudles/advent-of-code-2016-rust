use pathfinding::prelude::dijkstra;

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    let path = dijkstra(
        &(String::new(), (0, 0)),
        |(path, pos)| {
            let mut string = input.clone();
            string.push_str(path.as_str());
            let hash = format!("{:x}", md5::compute(string));
            hash.chars().take(4).enumerate().fold(Vec::new(), |mut result, (i, c)| {
                if ['b', 'c', 'd', 'e', 'f'].contains(&c) {
                    let new_pos = match i {
                        0 => (pos.0, pos.1 - 1),
                        1 => (pos.0, pos.1 + 1),
                        2 => (pos.0 - 1, pos.1),
                        _ => (pos.0 + 1, pos.1)
                    };
                    if (0..4).contains(&new_pos.0) && (0..4).contains(&new_pos.1) {
                        let mut new_path = path.clone();
                        new_path.push(
                            match i {
                                0 => 'U',
                                1 => 'D',
                                2 => 'L',
                                _ => 'R'
                            }
                        );
                        result.push(((new_path, new_pos), 1));
                    }
                }
                result
            })
        },
        |(_, pos)| pos == &(3, 3)
    );
    if let Some(path) = path {
        let last_result = path.0.last().map(|l| &l.0).unwrap();
        println!("{:?}", last_result);
    }
}

fn max_steps(input: &String, path: String, pos: (i64, i64)) -> u64 {
    if pos == (3, 3) {
        return 0;
    }

    let mut string = input.clone();
    string.push_str(path.as_str());
    let hash = format!("{:x}", md5::compute(string));
    hash
        .chars()
        .take(4)
        .enumerate()
        .flat_map(|(i, c)| {
            if ['b', 'c', 'd', 'e', 'f'].contains(&c) {
                let new_pos = match i {
                    0 => (pos.0, pos.1 - 1),
                    1 => (pos.0, pos.1 + 1),
                    2 => (pos.0 - 1, pos.1),
                    _ => (pos.0 + 1, pos.1)
                };
                if (0..4).contains(&new_pos.0) && (0..4).contains(&new_pos.1) {
                    let mut new_path = path.clone();
                    new_path.push(
                        match i {
                            0 => 'U',
                            1 => 'D',
                            2 => 'L',
                            _ => 'R'
                        }
                    );
                    return Some(max_steps(input, new_path, new_pos) + 1);
                }
            }
            None
        })
        .max()
        .unwrap_or(0)
}

pub fn part2(input: String) {
    max_steps(&input, String::new(), (0, 0)).print();
}
