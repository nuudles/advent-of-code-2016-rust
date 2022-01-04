use std::collections::HashSet;

pub fn part1(input: String) {
    let mut position = (0i64, 0i64);
    let mut dir_index = 0usize;
    let directions = vec![
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1)
    ];
    input
        .split(", ")
        .for_each(|i| {
            if i.starts_with("L") {
                dir_index = dir_index.checked_sub(1).unwrap_or(directions.len() - 1);
            } else {
                dir_index = (dir_index + 1) % directions.len();
            }
            let count = i[1..].parse().unwrap_or(0);
            position.0 += directions[dir_index].0 * count;
            position.1 += directions[dir_index].1 * count;
        });
    println!("{:?}", position.0.abs() + position.1.abs());
}

pub fn part2(input: String) {
    let mut position = (0i64, 0i64);
    let mut dir_index = 0usize;
    let directions = vec![
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1)
    ];
    let mut seen = HashSet::new();
    seen.insert(position);
    for i in input.split(", ") {
        if i.starts_with("L") {
            dir_index = dir_index.checked_sub(1).unwrap_or(directions.len() - 1);
        } else {
            dir_index = (dir_index + 1) % directions.len();
        }
        for _ in 0..i[1..].parse().unwrap_or(0) {
            position.0 += directions[dir_index].0;
            position.1 += directions[dir_index].1;
            if seen.contains(&position) {
                println!("{:?}", position.0.abs() + position.1.abs());
                return;
            }
            seen.insert(position);
        }
    }
}
