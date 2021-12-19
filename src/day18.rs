use std::fmt::Debug;
use std::iter::Peekable;
use std::ops::Deref;

use itertools::Itertools;

#[derive(Debug, Clone)]
enum Pair {
    Pair(Box<Pair>, Box<Pair>),
    Num(u64),
}

fn parse_pair(it: &mut Peekable<impl Iterator<Item = char>>) -> Pair {
    match it.peek() {
        Some('[') => {
            assert_eq!(it.next(), Some('[')); // opening bracket
            let left = parse_pair(it);
            assert_eq!(it.next(), Some(',')); // comma
            let right = parse_pair(it);
            assert_eq!(it.next(), Some(']')); // closing bracket

            Pair::Pair(Box::new(left), Box::new(right))
        }
        _ => Pair::Num(
            it.peeking_take_while(|ch| ch.is_numeric())
                .collect::<String>()
                .parse::<u64>()
                .unwrap(),
        ),
    }
}

fn explode(pair: &mut Pair, level: u32) -> Option<(Option<u64>, Option<u64>)> {
    match pair {
        Pair::Pair(left, right) => match (&**left, &**right) {
            (Pair::Num(left), Pair::Num(right)) if level >= 4 => {
                let explosion = Some((Some(*left), Some(*right)));
                *pair = Pair::Num(0);
                explosion
            }
            _ => match explode(left, level + 1) {
                None => match explode(right, level + 1) {
                    Some((Some(l_exp), r_exp)) => match catch_explosion(left, l_exp, false) {
                        Some(()) => Some((None, r_exp)),
                        None => Some((Some(l_exp), r_exp)),
                    },
                    e => e,
                },
                Some((l_exp, Some(r_exp))) => match catch_explosion(right, r_exp, true) {
                    Some(()) => Some((l_exp, None)),
                    None => Some((l_exp, Some(r_exp))),
                },
                e => e,
            },
        },
        Pair::Num(_) => None,
    }
}

fn catch_explosion(pair: &mut Pair, exp: u64, going_right: bool) -> Option<()> {
    match pair {
        Pair::Pair(left, right) if going_right => catch_explosion(left, exp, going_right)
            .or_else(|| catch_explosion(right, exp, going_right)),
        Pair::Pair(left, right) => catch_explosion(right, exp, going_right)
            .or_else(|| catch_explosion(left, exp, going_right)),
        Pair::Num(n) => {
            *pair = Pair::Num(*n + exp);
            Some(())
        }
    }
}

fn split(pair: &mut Pair) -> Option<()> {
    match pair {
        Pair::Pair(left, right) => split(left).or_else(|| split(right)),
        Pair::Num(n) if *n >= 10 => {
            *pair = Pair::Pair(
                Box::new(Pair::Num(*n / 2)),
                Box::new(Pair::Num(*n - *n / 2)),
            );
            Some(())
        }
        _ => None,
    }
}

fn add(left: Pair, right: Pair) -> Pair {
    fn reduce(pair: &mut Pair) {
        while let Some(_) = explode(pair, 0).map_or_else(|| split(pair), |_| Some(())) {}
    }

    let mut pair = Pair::Pair(Box::new(left), Box::new(right));
    reduce(&mut pair);
    pair
}

fn mag(pair: &Pair) -> u64 {
    match pair {
        Pair::Pair(left, right) => 3 * mag(left) + 2 * mag(right),
        Pair::Num(n) => *n,
    }
}

pub fn day18() {
    let s = include_str!("input/day18.txt");

    let mut sum = s
        .lines()
        .map(|l| parse_pair(&mut l.chars().peekable()))
        .fold1(|s, p| add(s, p))
        .unwrap();
    println!("{:?}", sum);

    println!("part 1: {}", mag(&sum));

    let pairs = s
        .lines()
        .map(|l| parse_pair(&mut l.chars().peekable()))
        .collect_vec();

    let max = (0..pairs.len())
        .cartesian_product(0..pairs.len())
        .filter(|(li, ri)| li != ri)
        .map(|(li, ri)| mag(&add(pairs[li].clone(), pairs[ri].clone())))
        .max()
        .unwrap();

    println!("part 2: {}", max);
}
