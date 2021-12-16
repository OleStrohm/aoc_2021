use itertools::Itertools;
use std::collections::HashSet;

pub fn day4() {
    let s = include_str!("day4.txt");
    let numbers = s
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u32>().unwrap());

    type Board = [[u32; 5]; 5];

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
