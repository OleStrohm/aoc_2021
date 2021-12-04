#![allow(dead_code)]

use std::collections::HashSet;

fn main() {
    day4();
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
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|lines| {
            lines
                .iter()
                .map(|l| {
                    l.split(" ")
                        .filter(|n| !n.is_empty())
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<[u32; 5]>>()
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
        let ns = ns
            .into_iter()
            .chain(std::iter::once(n))
            .collect::<HashSet<_>>();
        let score = bs.get(0).map(|b| board_score(b, &ns, n)).or(score);

        let next_boards: Vec<_> = bs.into_iter().filter(|b| !is_bingo(b, &ns)).collect();

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
