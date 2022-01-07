use itertools::Itertools;

pub fn part1(input: String) {
    // Figured out a pattern
    let mut max = 1;
    let mut counter = 1;
    for _ in 1..input.parse::<usize>().unwrap_or(0) {
        counter += 2;
        if counter > max {
            counter = 1;
            max = max * 2 + 1;
        }
    }
    println!("{}", counter);
}

pub fn part2(input: String) {
    // Couldn't quickly figure out the pattern, so brute forced it. 
    // Takes about 4 minutes to run.
    let mut elves = (1..=input.parse::<usize>().unwrap_or(0)).collect_vec();
    let mut turn = 0usize;
    while elves.len() > 1 {
        if turn + elves.len() / 2 >= elves.len() {
            elves.remove((turn + elves.len() / 2) % elves.len());
            turn -= 1;
        } else {
            elves.remove(turn + elves.len() / 2);
        }
        turn = if turn + 1 >= elves.len() { 0 } else { turn + 1 };
    }
    println!("{}", elves[0]);
}
