use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;

use itertools::Itertools;

pub fn day24() {
    let _s = include_str!("input/day24.txt");

    // Manually parsed from the input
    let divz: [i64; 14] = [1, 1, 1, 1, 26, 1, 1, 26, 1, 26, 26, 26, 26, 26];
    let popz = divz.map(|d| d == 26);
    let addx: [i64; 14] = [12, 12, 13, 12, -3, 10, 14, -16, 12, -8, -12, -7, -6, -11];
    let addy: [i64; 14] = [7, 8, 2, 11, 6, 12, 14, 13, 15, 10, 6, 10, 8, 5];

    let mut possibilites = vec![vec![false], vec![true]];
    for i in 1..14 {
        possibilites = possibilites
            .into_iter()
            .flat_map(|mut v| {
                let mut vnew = v.clone();
                v.push(true);
                vnew.push(false);
                [v, vnew]
            })
            .collect();
    }

    #[derive(Debug, Clone, Copy)]
    enum Diff {
        Free,
        RelTo(usize, i64),
    }
    let constraints = possibilites
        .clone()
        .into_iter()
        .filter_map(|pushed| {
            let mut bt = Vec::new();
            let mut diffs = HashMap::new();
            for i in 0..14 {
                let last = bt.last().copied();
                if popz[i] {
                    bt.pop();
                }
                if pushed[i] {
                    bt.push(i);
                } else {
                    let last_i = last?;
                    diffs.insert((i, last_i), addy[last_i] + addx[i]);
                }
            }
            (bt.len() == 0).then(|| diffs)
        })
        .filter(|diffs| diffs.values().all(|d| d.abs() <= 8))
        .collect_vec();

    let constraints = constraints[0].clone();

    let mut max_num = [0; 14];
    for (&(r, l), &v) in &constraints {
        if v >= 0 {
            max_num[r] = 9;
            max_num[l] = 9 - v;
        } else {
            max_num[r] = 9 + v;
            max_num[l] = 9;
        }
    }

    print!("part 1: ");
    max_num.iter().for_each(|d| print!("{}", d));
    println!();

    let mut min_num = [0; 14];
    for (&(r, l), &v) in &constraints {
        if v >= 0 {
            max_num[r] = 1 + v;
            max_num[l] = 1;
        } else {
            max_num[r] = 1;
            max_num[l] = 1 - v;
        }
    }

    print!("part 2: ");
    max_num.iter().for_each(|d| print!("{}", d));
}
