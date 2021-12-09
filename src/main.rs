#![allow(dead_code)]
#![allow(unused)]

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

mod previous_days;

fn main() {
    let s = include_str!("day9.txt");
    let grid = s
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let low_points = grid
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            let grid = &grid;
            r.iter().enumerate().filter_map(move |(x, n)| {
                let up = y
                    .checked_sub(1)
                    .and_then(|y| grid.get(y))
                    .and_then(|r| r.get(x));
                let down = grid.get(y + 1).and_then(|r| r.get(x));
                let left = grid
                    .get(y)
                    .and_then(|r| x.checked_sub(1).and_then(|x| r.get(x)));
                let right = grid.get(y).and_then(|r| r.get(x + 1));

                up.into_iter()
                    .chain(down)
                    .chain(left)
                    .chain(right)
                    .min()
                    .and_then(|min| (n < min).then(|| (x, y, n)))
            })
        })
        .collect_vec();
    let risk_level = low_points.iter().map(|(_, _, &low)| low + 1).sum::<u32>();
    println!("part 1: {}", risk_level);

    let height = s.lines().count();
    let width = s.lines().next().unwrap().len();

    fn flood_fill(
        g: &mut Vec<Vec<(usize, usize, u32)>>,
        basin: (usize, usize),
        (x, y): (usize, usize),
    ) {
        match g[y][x] {
            (_, _, 9) => return,
            (bx, by, _) if (bx, by) == basin => return,
            _ => {
                g[y][x] = (basin.0, basin.1, g[y][x].2);
                if let Some(x) = x.checked_sub(1) {
                    flood_fill(g, basin, (x, y));
                }
                if let Some(y) = y.checked_sub(1) {
                    flood_fill(g, basin, (x, y));
                }
                if x + 1 < g[0].len() {
                    flood_fill(g, basin, (x + 1, y));
                }
                if y + 1 < g.len() {
                    flood_fill(g, basin, (x, y + 1));
                }
            }
        }
    }

    fn not_separated_by_9(
        grid: &Vec<Vec<u32>>,
        (sx, sy): (usize, usize),
        (ex, ey): (usize, usize),
    ) -> bool {
        let xmin = sx.min(ex);
        let xmax = sx.max(ex);
        let ymin = sy.min(ey);
        let ymax = sy.max(ey);

        let separated = (xmin..=xmax).all(|x| grid[ymin][x] < 9)
            && (ymin..=ymax).all(|y| grid[y][xmax] < 9)
            || (xmin..=xmax).all(|x| grid[ymax][x] < 9) && (ymin..=ymax).all(|y| grid[y][xmin] < 9);
        println!(
            "sx {} | sy {} | ex {} | ey {} | not_sep {}",
            sx, sy, ex, ey, separated
        );
        separated
    }

    let mut grid = grid
        .iter()
        .enumerate()
        .map(|(y, r)| r.iter().enumerate().map(|(x, &v)| (x, y, v)).collect_vec())
        .collect_vec();

    let basin = low_points
        .iter()
        .map(|&(x, y, _)| {
            let basin = (x, y);
            if let Some(x) = x.checked_sub(1) {
                flood_fill(&mut grid, basin, (x, y));
            }
            if let Some(y) = y.checked_sub(1) {
                flood_fill(&mut grid, basin, (x, y));
            }
            if x + 1 < grid[0].len() {
                flood_fill(&mut grid, basin, (x + 1, y));
            }
            if y + 1 < grid.len() {
                flood_fill(&mut grid, basin, (x, y + 1));
            }

            grid.iter()
                .flat_map(|r| r.iter().filter(|(bx, by, _)| (*bx, *by) == basin))
                .count() as u64
        })
        .sorted()
        .rev()
        .take(3)
        .product::<u64>();

    println!("part 2: {}", basin);
}
