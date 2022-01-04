use counter::Counter;

pub fn part1(input: String) {
    let mut counters = (0..8).map(|_| Counter::<char, u64>::new()).collect::<Vec<_>>();
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            counters[i][&c] += 1;
        }
    }
    let chars = counters.iter().map(|c| c.most_common().first().unwrap().0).collect::<String>();
    println!("{}", chars);
}

pub fn part2(input: String) {
    let mut counters = (0..8).map(|_| Counter::<char, u64>::new()).collect::<Vec<_>>();
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            counters[i][&c] += 1;
        }
    }
    let chars = counters.iter().map(|c| c.most_common().last().unwrap().0).collect::<String>();
    println!("{}", chars);
}
