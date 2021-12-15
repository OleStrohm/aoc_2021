#![allow(dead_code)]
#![allow(unused)]

use std::collections::{HashMap, VecDeque};
use std::slice::SliceIndex;

use itertools::Itertools;

mod previous_days;

fn main() {
    let s = include_str!("day15.txt");

    let grid = s
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let height = grid.len();
    let width = grid[0].len();

    let grid = &grid;

    let mut grid = (0..5 * height)
        .map(|y| {
            (0..5 * width)
                .map(move |x| {
                    (
                        (grid[y % height][x % width] + (x / width + y / height) as u32 - 1) % 9
                            + 1,
                        None,
                    )
                })
                .collect_vec()
        })
        .collect_vec();

    let height = grid.len();
    let width = grid[0].len();

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));

    let risk = loop {
        let (x, y, tot_risk) = queue.pop_front().unwrap();
        if grid[y][x].1.is_some() {
            continue;
        }

        if (x, y) == (width - 1, height - 1) {
            break tot_risk;
        }

        grid[y][x].1 = Some(tot_risk);

        for new_node in [(-1, 0), (0, -1), (1, 0), (0, 1)]
            .into_iter()
            .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
            .filter(|&(x, y)| 0 <= x && x < width as isize && 0 <= y && y < height as isize)
            .map(|(x, y)| {
                (
                    x as usize,
                    y as usize,
                    tot_risk + grid[y as usize][x as usize].0,
                )
            })
        {
            let (Err(i) | Ok(i)) = queue.binary_search_by_key(&new_node.2, |&(_, _, risk)| risk);
            queue.insert(i, new_node);
        }
    };

    println!("part 2: {}", risk);
}
