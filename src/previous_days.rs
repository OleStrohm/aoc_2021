use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

fn step(state: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let new = state.get(0).map_or(0, |&(_, k)| k) + state.get(2).map_or(0, |&(a, _)| a);
    state
        .split_at(1)
        .1
        .iter()
        .copied()
        .chain(std::iter::once((new, new)))
        .collect()
}

fn day6() {
    let s = include_str!("day6.txt");
    let numbers = s
        .trim_end()
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .sorted()
        .group_by(|&n| n)
        .into_iter()
        .map(|(key, group)| (key, group.count() as _))
        .collect::<HashMap<_, _>>();
    let state = itertools::repeat_n((0, 0), 2)
        .chain((0..7).map(|d| (numbers.get(&d).copied().unwrap_or(0), 0)))
        .collect_vec();

    println!("Initial state: {:?}", state);

    let part1 = (0..80).fold(state.clone(), |state, _| step(&state));

    println!(
        "part 1: {}",
        part1.iter().skip(2).map(|(a, _)| a).sum::<u64>()
            + part1.iter().map(|(_, k)| k).sum::<u64>()
    );

    let part2 = (0..256).fold(state, |state, _| step(&state));

    println!(
        "part 1: {}",
        part2.iter().skip(2).map(|(a, _)| a).sum::<u64>()
            + part2.iter().map(|(_, k)| k).sum::<u64>()
    );
}

type Point = (u32, u32);
type Line = (Point, Point);

fn line_contains_point(((x1, y1), (x2, y2)): Line, (x, y): Point) -> bool {
    let between = |p, b1: u32, b2| b1.min(b2) <= p && p <= b1.max(b2);
    x1 == x2 && x1 == x && between(y, y1, y2) || y1 == y2 && y1 == y && between(x, x1, x2)
}

fn line_contains_point_diag(((x1, y1), (x2, y2)): Line, (x, y): Point) -> bool {
    let between = |p, b1: u32, b2| b1.min(b2) <= p && p <= b1.max(b2);
    let between_diag = |(x, y), (x1, y1): Point, (x2, y2): Point| {
        x1 + y2 == x2 + y1 && x + y1 == x1 + y || x1 + y1 == x2 + y2 && x + y == x1 + y1
    };
    between(x, x1, x2)
        && between(y, y1, y2)
        && (x1 == x2 || y1 == y2 || between_diag((x, y), (x1, y1), (x2, y2)))
}

fn day5() {
    let s = include_str!("day5.txt");
    let lines: Vec<Line> = s
        .lines()
        .map(|l| {
            let [x1, y1]: [u32; 2] = l
                .split(&[',', ' '] as &[_])
                .take(2)
                .map(|n| n.parse().unwrap())
                .collect_vec()
                .try_into()
                .unwrap();
            let [y2, x2]: [u32; 2] = l
                .rsplit(&[',', ' '] as &[_])
                .take(2)
                .map(|n| n.parse().unwrap())
                .collect_vec()
                .try_into()
                .unwrap();

            ((x1, y1), (x2, y2))
        })
        .collect_vec();

    let bounds = lines
        .iter()
        .flat_map(|&((x1, y1), (x2, y2))| vec![(x1, y1), (x2, y2)])
        .collect_vec();

    let (xmin, xmax) = bounds
        .iter()
        .map(|&(x, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (ymin, ymax) = bounds
        .iter()
        .map(|&(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();

    let danger_points = (ymin..=ymax)
        .into_par_iter()
        .flat_map_iter(|y| {
            let lines = &lines;
            (xmin..=xmax).filter(move |&x| {
                lines
                    .par_iter()
                    .filter(|&&l| line_contains_point(l, (x, y)))
                    .count()
                    >= 2
            })
        })
        .count();

    println!("part 1: {}", danger_points);

    let danger_points = (ymin..=ymax)
        .into_par_iter()
        .flat_map_iter(|y| {
            let lines = &lines;
            (xmin..=xmax).filter(move |&x| {
                lines
                    .par_iter()
                    .filter(|&&l| line_contains_point_diag(l, (x, y)))
                    .count()
                    >= 2
            })
        })
        .count();

    println!("part 2: {}", danger_points);
}

type Board = [[u32; 5]; 5];

fn is_bingo(board: &Board, numbers: &HashSet<u32>) -> bool {
    let row_bingo = (0..5).any(|row| (0..5).all(|col| numbers.contains(&board[row][col])));
    let col_bingo = (0..5).any(|col| (0..5).all(|row| numbers.contains(&board[row][col])));

    row_bingo || col_bingo
}

fn board_score(board: &Board, numbers: &HashSet<u32>, last_num: u32) -> u32 {
    (0..5)
        .flat_map(|row| (0..5).map(move |col| (row, col)))
        .map(|(r, c)| board[r][c])
        .filter(|b| !numbers.contains(b))
        .sum::<u32>()
        * last_num
}

fn day4() {
    let s = include_str!("day4.txt");
    let numbers = s
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u32>().unwrap());

    let boards: Vec<Board> = s
        .lines()
        .skip(1)
        .filter(|l| !l.is_empty())
        .chunks(5)
        .into_iter()
        .map(|lines| {
            lines
                .map(|l| {
                    l.split(" ")
                        .filter_map(|n| n.parse::<u32>().ok())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();

    let (score, _) = numbers
        .clone()
        .fold((None, HashSet::new()), |(score, ns), n| match score {
            Some(s) => (Some(s), ns),
            _ => {
                let ns = ns
                    .into_iter()
                    .chain(std::iter::once(n))
                    .collect::<HashSet<_>>();
                let score = boards
                    .iter()
                    .filter(|b| is_bingo(b, &ns))
                    .map(|b| board_score(b, &ns, n))
                    .max();
                (score, ns)
            }
        });

    println!("part 1: {}", score.unwrap());

    let (score, _, _) = numbers.fold((None, boards, HashSet::new()), |(score, bs, ns), n| {
        let ns = ns.into_iter().chain(std::iter::once(n)).collect::<_>();
        let score = bs.get(0).map(|b| board_score(b, &ns, n)).or(score);
        let next_boards = bs.into_iter().filter(|b| !is_bingo(b, &ns)).collect();

        (score, next_boards, ns)
    });

    println!("part 2: {:?}", score.unwrap());
}

fn day3() {
    let s = include_str!("day3.txt");
    let line_count = s.lines().count() as u64;
    let line_len = s.lines().next().unwrap().len() as u64;

    let bits: Vec<Vec<u64>> = s
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect();

    let most_common = bits
        .iter()
        .fold(Vec::new(), |s, n| {
            n.iter()
                .zip(s.into_iter().chain(std::iter::repeat(0)))
                .map(|(a, b)| a + b)
                .collect()
        })
        .into_iter()
        .map(|b| if 2 * b >= line_count { 1 } else { 0 })
        .collect::<Vec<_>>();

    let gamma = most_common.iter().fold(0, |s, b| 2 * s + b);
    let epsilon = most_common.iter().fold(0, |s, b| 2 * s + (1 - b));

    println!("part 1: {}", gamma * epsilon);

    let numbers: Vec<u64> = s
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .fold(0, |s, b| 2 * s + b)
        })
        .collect();

    let oxygen = (0..line_len).rev().fold(numbers.clone(), |left, i| {
        let common = left.iter().fold(0, |s, n| s + ((n >> i) & 1));
        let common = if 2 * common >= left.len().try_into().unwrap() {
            1
        } else {
            0
        };

        if left.len() == 1 {
            left
        } else {
            left.into_iter()
                .filter(|&n| ((n >> i) & 1) == common)
                .collect()
        }
    })[0];
    let co2 = (0..line_len).rev().fold(numbers.clone(), |left, i| {
        let common = left.iter().fold(0, |s, n| s + ((n >> i) & 1));
        let uncommon = if 2 * common >= left.len().try_into().unwrap() {
            0
        } else {
            1
        };

        if left.len() == 1 {
            left
        } else {
            left.into_iter()
                .filter(|&n| ((n >> i) & 1) == uncommon)
                .collect()
        }
    })[0];

    println!("oxygen: {}", oxygen);
    println!("co2: {}", co2);
    println!("part 2: {}", oxygen * co2);
}

enum Movement {
    Up(i64),
    Down(i64),
    Forward(i64),
}

fn day2() {
    let s = include_str!("day2.txt");
    let movements = s
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split(" ").collect();
            let dist = parts[1].parse().unwrap();
            match parts[0] {
                "up" => Movement::Up(dist),
                "down" => Movement::Down(dist),
                "forward" => Movement::Forward(dist),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();

    let (horizontal, depth) = movements.iter().fold((0, 0), |(h, d), m| match m {
        Movement::Up(dist) => (h, d - dist),
        Movement::Down(dist) => (h, d + dist),
        Movement::Forward(dist) => (h + dist, d),
    });

    println!("part 1: {}", horizontal * depth);

    let (_, horizontal, depth) = movements.iter().fold((0, 0, 0), |(aim, h, d), m| match m {
        Movement::Up(dist) => (aim - dist, h, d),
        Movement::Down(dist) => (aim + dist, h, d),
        Movement::Forward(dist) => (aim, h + dist, d + aim * dist),
    });

    println!("part 2: {}", horizontal * depth);
}

fn day1() {
    let s = include_str!("day1.txt");
    let ns = s
        .lines()
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    // part 1
    let increases = ns.windows(2).filter(|vals| vals[0] < vals[1]).count();
    println!("part 1: {}", increases);

    // part 2
    let windowed_ns = ns
        .windows(3)
        .map(|ns| ns.iter().sum())
        .collect::<Vec<u64>>();
    let increases = windowed_ns
        .windows(2)
        .filter(|vals| vals[0] < vals[1])
        .count();
    println!("part 2: {}", increases);
}
