use itertools::{FoldWhile, Itertools};
use rayon::prelude::*;

pub fn day25() {
    let s = include_str!("input/day25.txt");
    let mut grid = s.lines().map(|l| l.chars().collect_vec()).collect_vec();

    grid.iter()
        .for_each(|r| println!("{}", r.iter().collect::<String>()));
    println!();

    fn half_step(grid: Vec<Vec<char>>, ty: char) -> Vec<Vec<char>> {
        let height = grid.len();
        let width = grid[0].len();
        let dx = (ty == '>') as usize;
        let dy = (ty == 'v') as usize;

        grid.iter()
            .enumerate()
            .map(|(y, r)| {
                r.into_iter()
                    .enumerate()
                    .map(|(x, v)| {
                        let x_orig = (x as isize - dx as isize).rem_euclid(width as isize) as usize;
                        let y_orig =
                            (y as isize - dy as isize).rem_euclid(height as isize) as usize;
                        let x_dest = (x + dx).rem_euclid(width);
                        let y_dest = (y + dy).rem_euclid(height);

                        if grid[y][x] == '.' && grid[y_orig][x_orig] == ty {
                            ty
                        } else if grid[y][x] == ty && grid[y_dest][x_dest] == '.' {
                            '.'
                        } else {
                            *v
                        }
                    })
                    .collect_vec()
            })
            .collect_vec()
    }

    let mut moved = false;
    let (grid, steps) = (1..)
        .fold_while((grid, 0), |(grid, _), i| {
            let prev = grid.clone();
            let mut grid = half_step(grid, '>');
            grid = half_step(grid, 'v');
            if prev != grid {
                FoldWhile::Continue((grid, i))
            } else {
                FoldWhile::Done((grid, i))
            }
        })
        .into_inner();

    grid.iter()
        .for_each(|r| println!("{}", r.iter().collect::<String>()));
    println!();
    println!("part 1: {}", steps);
}
