use std::io;

#[derive(Debug, Clone)]
struct Range {
    from: u64,
    to: u64,
}

impl Range {
    fn new(from: u64, to: u64) -> Self {
        assert_eq!(from <= to, true, "from must be less or equal to to");
        Self { from, to }
    }
}
#[derive(Debug)]
struct Input {
    ranges: Vec<Range>,
    ids: Vec<u64>,
}

fn main() {
    let input = parse_input();
    println!("input: {input:?}");

    let part1_res = calc_part1(&input);
    println!("part1 res {part1_res:?}");

    let part2_res = calc_part2(&input);
    println!("part2 res {part2_res:?}");
}

fn parse_input() -> Input {
    let input_str = io::read_to_string(io::stdin()).unwrap();
    println!("input_str: {input_str:?}");
    let (ranges_str, ids_str) = input_str.split_once("\r\n\r\n").unwrap();

    let ranges = ranges_str
        .lines()
        .map(|line| {
            let (from, to) = line.split_once("-").unwrap();
            Range::new(from.parse::<u64>().unwrap(), to.parse::<u64>().unwrap())
        })
        .collect::<Vec<Range>>();

    let ids = ids_str
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    Input { ranges, ids }
}

fn calc_part1(input: &Input) -> u64 {
    let mut fresh_qty = 0;

    input.ids.iter().for_each(|id| {
        let in_range = input
            .ranges
            .iter()
            .find(|range| range.from <= *id && *id <= range.to);

        if in_range.is_some() {
            fresh_qty += 1;
        }
    });

    fresh_qty
}

fn calc_part2(input: &Input) -> u64 {
    let mut ranges = input.ranges.clone();
    ranges.sort_by_key(|r| (r.from, r.to));

    let mut merged = Vec::with_capacity(ranges.len());
    let mut cur = ranges[0].clone();
    for r in ranges.into_iter().skip(1) {
        if r.from <= cur.to + 1 {
            cur.to = cur.to.max(r.to);
        } else {
            merged.push(cur);
            cur = r;
        }
    }
    merged.push(cur);
    merged.into_iter().map(|r| r.to - r.from + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::{Input, Range, calc_part2};

    #[test]
    fn part2_holes_two_side() {
        let input: Input = Input {
            ranges: vec![
                Range { from: 1, to: 3 },
                Range { from: 5, to: 7 },
                Range { from: 9, to: 12 },
                Range { from: 4, to: 8 },
            ],
            ids: vec![],
        };
        assert_eq!(calc_part2(&input), 12);
    }

    #[test]
    fn part2_left_merge_in_the_middle() {
        let input: Input = Input {
            ranges: vec![
                Range { from: 1, to: 3 },
                Range { from: 5, to: 7 },
                Range { from: 10, to: 12 },
                Range { from: 6, to: 8 },
            ],
            ids: vec![],
        };
        assert_eq!(calc_part2(&input), 10);
    }

    #[test]
    fn part2_left_merge_left() {
        let input: Input = Input {
            ranges: vec![
                Range { from: 1, to: 3 },
                Range { from: 10, to: 12 },
                Range { from: 2, to: 4 },
            ],
            ids: vec![],
        };
        assert_eq!(calc_part2(&input), 7);
    }

    #[test]
    fn part2_right_merge_in_the_middle() {
        let input: Input = Input {
            ranges: vec![
                Range { from: 1, to: 3 },
                Range { from: 5, to: 6 },
                Range { from: 10, to: 12 },
                Range { from: 9, to: 11 },
            ],
            ids: vec![],
        };
        assert_eq!(calc_part2(&input), 9);
    }

    #[test]
    fn part2_right_merge_right() {
        let input: Input = Input {
            ranges: vec![
                Range { from: 1, to: 3 },
                Range { from: 10, to: 12 },
                Range { from: 9, to: 11 },
            ],
            ids: vec![],
        };
        assert_eq!(calc_part2(&input), 7);
    }

    #[test]
    fn part2_new_left() {
        let input: Input = Input {
            ranges: vec![Range { from: 5, to: 7 }, Range { from: 1, to: 3 }],
            ids: vec![],
        };
        assert_eq!(calc_part2(&input), 6);
    }

    #[test]
    fn part2_new_right() {
        let input: Input = Input {
            ranges: vec![Range { from: 5, to: 7 }, Range { from: 10, to: 12 }],
            ids: vec![],
        };
        assert_eq!(calc_part2(&input), 6);
    }

    #[test]
    fn part2_contains_left() {
        let input: Input = Input {
            ranges: vec![
                Range { from: 1, to: 3 },
                Range { from: 5, to: 7 },
                Range { from: 2, to: 2 },
            ],
            ids: vec![],
        };
        assert_eq!(calc_part2(&input), 6);
    }

    #[test]
    fn part2_contains_right() {
        let input: Input = Input {
            ranges: vec![
                Range { from: 1, to: 3 },
                Range { from: 5, to: 7 },
                Range { from: 6, to: 6 },
            ],
            ids: vec![],
        };
        assert_eq!(calc_part2(&input), 6);
    }

    #[test]
    fn part2_merge_right_bug_1() {
        let input: Input = Input {
            ranges: vec![Range { from: 1, to: 100 }, Range { from: 50, to: 60 }],
            ids: vec![],
        };
        assert_eq!(calc_part2(&input), 100);
    }

    #[test]
    fn part2_merge_left_bug_1() {
        let input: Input = Input {
            ranges: vec![Range { from: 50, to: 60 }, Range { from: 1, to: 100 }],
            ids: vec![],
        };
        assert_eq!(calc_part2(&input), 100);
    }

    #[test]
    fn part2_merge_left_bug_2() {
        let input: Input = Input {
            ranges: vec![
                Range { from: 5, to: 5 },
                Range { from: 100, to: 100 },
                Range { from: 1, to: 2 },
            ],
            ids: vec![],
        };
        assert_eq!(calc_part2(&input), 4);
    }

    #[test]
    fn part2_cross_overlap_missed_left_merge() {
        // Correct union is [1, 20] → 20 IDs
        let input = Input {
            ranges: vec![
                Range { from: 1, to: 10 },
                Range { from: 8, to: 20 },
                Range { from: 9, to: 12 },
            ],
            ids: vec![],
        };
        assert_eq!(calc_part2(&input), 20);
    }

    #[test]
    fn part2_example_from_input1() {
        let input: Input = Input {
            ranges: vec![
                Range { from: 3, to: 5 },
                Range { from: 10, to: 14 },
                Range { from: 16, to: 20 },
                Range { from: 12, to: 18 },
            ],
            ids: vec![],
        };
        assert_eq!(calc_part2(&input), 14);
    }
}
