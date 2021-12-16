use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub fn day11() {
    let s = include_str!("day11.txt");

    let mut grid: Vec<Vec<u32>> = s
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    fn step(grid: Vec<Vec<u32>>, old_grid: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
        fn offset((x, y): (usize, usize), (dx, dy): (isize, isize)) -> Option<(usize, usize)> {
            Some((
                (x as isize + dx).try_into().ok()?,
                (y as isize + dy).try_into().ok()?,
            ))
        }

        let new_grid = grid
            .iter()
            .enumerate()
            .map(|(y, col)| {
                col.iter()
                    .enumerate()
                    .map(|(x, v)| {
                        let boost = [
                            (-1, -1),
                            (0, -1),
                            (1, -1),
                            (-1, 0),
                            (1, 0),
                            (-1, 1),
                            (0, 1),
                            (1, 1),
                        ]
                        .map(|off| offset((x, y), off))
                        .into_iter()
                        .flatten()
                        .filter_map(|(x, y)| {
                            Some((*old_grid.get(y)?.get(x)?, *grid.get(y)?.get(x)?))
                        })
                        .filter(|&(old, new)| old < 10 && new >= 10)
                        .count();

                        ((v + boost as u32).min(10))
                    })
                    .collect_vec()
            })
            .collect_vec();

        if new_grid != grid {
            step(new_grid, grid)
        } else {
            new_grid
        }
    }

    let (_, flashes) = (0..100).fold((grid.clone(), 0), |(grid, flashes), _| {
        let new_grid = grid
            .iter()
            .map(|r| r.iter().map(|v| (v + 1).min(10)).collect_vec())
            .collect_vec();
        let grid = step(new_grid, grid);
        let flashes = flashes + grid.iter().flatten().filter(|&&v| v == 10).count();
        let grid = grid
            .iter()
            .map(|r| r.iter().map(|v| v % 10).collect_vec())
            .collect_vec();

        (grid, flashes)
    });

    println!("part 1: {}", flashes);

    let (_, steps_to_all_flashing) = (1..)
        .fold_while((grid, 0), |(grid, time), s| {
            let new_grid = grid
                .iter()
                .map(|r| r.iter().map(|v| (v + 1).min(10)).collect_vec())
                .collect_vec();
            let grid = step(new_grid, grid);
            if grid.iter().flatten().all(|&v| v == 10) {
                Done((grid, s))
            } else {
                let grid = grid
                    .iter()
                    .map(|r| r.iter().map(|v| v % 10).collect_vec())
                    .collect_vec();

                Continue((grid, 0))
            }
        })
        .into_inner();

    println!("part 2: {}", steps_to_all_flashing);
}
