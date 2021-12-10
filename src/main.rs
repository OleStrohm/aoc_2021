#![allow(dead_code)]
#![allow(unused)]

use std::io::BufRead;

use itertools::Itertools;

mod previous_days;

fn main() {
    let s = include_str!("day10.txt");

    fn closing(p: char) -> char {
        match p {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => unreachable!(),
        }
    }

    enum ChunkError {
        Incomplete(String),
        Incorrect(u64),
    }

    fn parse_chunk(mut line: &str) -> (Result<&str, ChunkError>) {
        while let Some(p @ ('(' | '[' | '{' | '<')) = line.chars().next() {
            //println!("opening {}", p);
            line = match parse_chunk(&line[1..]) {
                Ok(l) => l,
                Err(e @ ChunkError::Incorrect(_)) => return Err(e),
                Err(ChunkError::Incomplete(bt)) => {
                    return Err(ChunkError::Incomplete(bt + &closing(p).to_string()))
                }
            };

            //println!("try closing {}", closing(p));
            line = match line.chars().next() {
                Some(c) if c == closing(p) => Ok(&line[1..]),
                Some(')') => Err(ChunkError::Incorrect(3)),
                Some(']') => Err(ChunkError::Incorrect(57)),
                Some('}') => Err(ChunkError::Incorrect(1197)),
                Some('>') => Err(ChunkError::Incorrect(25137)),
                None => Err(ChunkError::Incomplete(closing(p).to_string())),
                _ => unreachable!(),
            }?;
        }

        Ok(line)
    }

    let syntax_error = s
        .lines()
        .filter_map(|l| match parse_chunk(l) {
            Err(ChunkError::Incorrect(e)) => Some(e),
            _ => None,
        })
        .sum::<u64>();
    
    println!("part 1: {}", syntax_error);

    let autocomplete_error = s
        .lines()
        .filter_map(|l| match parse_chunk(l) {
            Err(ChunkError::Incomplete(bt)) => Some(dbg!(bt)),
            _ => None,
        })
        .map(|bt| {
            bt.chars().fold(0_u64, |s, c| {
                5 * s
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            })
        })
        .sorted()
        .collect_vec();

    println!("part 2: {}", autocomplete_error[autocomplete_error.len()/2]);
}
