use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    input
        .lines()
        .map(|l|
            l.split(' ')
                .filter(|s| !s.is_empty())
                .filter_map(|s| s.parse::<u64>().ok())
                .collect::<Vec<_>>()
        )
        .filter(|v| v[0] + v[1] > v[2] && v[0] + v[2] > v[1] && v[1] + v[2] > v[0])
        .count()
        .print();
}

pub fn part2(input: String) {
    input
        .lines()
        .map(|l|
            l.split(' ')
                .filter(|s| !s.is_empty())
                .filter_map(|s| s.parse::<u64>().ok())
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>()
        .chunks(3)
        .flat_map(|w| 
            vec![
                vec![w[0][0], w[1][0], w[2][0]],
                vec![w[0][1], w[1][1], w[2][1]],
                vec![w[0][2], w[1][2], w[2][2]]
            ]
        )
        .filter(|v| v[0] + v[1] > v[2] && v[0] + v[2] > v[1] && v[1] + v[2] > v[0])
        .count()
        .print();
}
