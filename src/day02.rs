use std::cmp::min;
use std::collections::HashMap;

pub fn part1(input: String) {
    let mut position = (1u64, 1u64);
    println!(
        "{}",
        input
            .lines()
            .fold(String::new(), |mut code, l| {
                l.bytes().for_each(|b|
                    match b {
                        b'U' => position.1 = position.1.saturating_sub(1),
                        b'D' => position.1 = min(position.1 + 1, 2),
                        b'L' => position.0 = position.1.saturating_sub(1),
                        b'R' => position.0 = min(position.1 + 1, 2),
                        _ => ()
                    }
                );
                code.push(
                    match position {
                        (0, 0) => '1',
                        (1, 0) => '2',
                        (2, 0) => '3',
                        (0, 1) => '4',
                        (1, 1) => '5',
                        (2, 1) => '6',
                        (0, 2) => '7',
                        (1, 2) => '8',
                        _ => '9'
                    }
                );
                code
            })
    );
}

pub fn part2(input: String) {
    let mut keypad = HashMap::new();
    keypad.insert((2, 0), '1');
    keypad.insert((1, 1), '2');
    keypad.insert((2, 1), '3');
    keypad.insert((3, 1), '4');
    keypad.insert((0, 2), '5');
    keypad.insert((1, 2), '6');
    keypad.insert((2, 2), '7');
    keypad.insert((3, 2), '8');
    keypad.insert((4, 2), '9');
    keypad.insert((1, 3), 'A');
    keypad.insert((2, 3), 'B');
    keypad.insert((3, 3), 'C');
    keypad.insert((2, 4), 'D');

    let mut position = (0, 2);
    println!(
        "{}",
        input
            .lines()
            .fold(String::new(), |mut code, l| {
                l.bytes().for_each(|b| {
                    let mut new_position = position;
                    match b {
                        b'U' => new_position.1 -= 1,
                        b'D' => new_position.1 += 1,
                        b'L' => new_position.0 -= 1,
                        b'R' => new_position.0 += 1,
                        _ => ()
                    }
                    if keypad.contains_key(&new_position) {
                        position = new_position;
                    }
                });
                code.push(*keypad.get(&position).unwrap_or(&'X'));
                code
            })
    );
}
