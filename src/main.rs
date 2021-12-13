#![allow(dead_code)]
#![allow(unused)]

use itertools::Itertools;

mod previous_days;

fn main() {
    let s = include_str!("day13.txt");
    let mut lines = s.lines();
    let locations: Vec<(u32, u32)> = (&mut lines)
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();

    #[derive(Copy, Clone)]
    enum FoldAlong {
        X(u32),
        Y(u32),
    }

    let instructions = lines
        .map(|l| match l.split('=').collect_tuple() {
            Some(("fold along x", p)) => FoldAlong::X(p.parse().unwrap()),
            Some(("fold along y", p)) => FoldAlong::Y(p.parse().unwrap()),
            _ => unreachable!(),
        })
        .collect_vec();

    fn fold_along(dots: Vec<(u32, u32)>, fold: FoldAlong) -> Vec<(u32, u32)> {
        dots.iter()
            .map(|&(x, y)| match fold {
                FoldAlong::X(p) => (if x < p { x } else { 2 * p - x }, y),
                FoldAlong::Y(p) => (x, if y < p { y } else { 2 * p - y }),
            })
            .collect()
    }

    println!(
        "part 1: {}",
        fold_along(locations.clone(), instructions[0])
            .into_iter()
            .counts()
            .keys()
            .count()
    );

    let code = instructions.iter().fold(locations, |s, &instr| fold_along(s, instr));
    println!("part 2:");
    for y in 0..6 {
        for x in 0..40 {
            if code.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
