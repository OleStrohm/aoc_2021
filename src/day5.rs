use itertools::Itertools;
use rayon::prelude::*;

pub fn day5() {
    let s = include_str!("day5.txt");

    type Point = (u32, u32);
    type Line = (Point, Point);

    fn line_contains_point(((x1, y1), (x2, y2)): Line, (x, y): Point) -> bool {
        let between = |p, b1: u32, b2| b1.min(b2) <= p && p <= b1.max(b2);
        x1 == x2 && x1 == x && between(y, y1, y2) || y1 == y2 && y1 == y && between(x, x1, x2)
    }

    fn line_contains_point_diag(((x1, y1), (x2, y2)): Line, (x, y): Point) -> bool {
        let between = |p, b1: u32, b2| b1.min(b2) <= p && p <= b1.max(b2);
        let between_diag = |(x, y), (x1, y1): Point, (x2, y2): Point| {
            x1 + y2 == x2 + y1 && x + y1 == x1 + y || x1 + y1 == x2 + y2 && x + y == x1 + y1
        };
        between(x, x1, x2)
            && between(y, y1, y2)
            && (x1 == x2 || y1 == y2 || between_diag((x, y), (x1, y1), (x2, y2)))
    }

    let lines: Vec<Line> = s
        .lines()
        .map(|l| {
            let [x1, y1]: [u32; 2] = l
                .split(&[',', ' '] as &[_])
                .take(2)
                .map(|n| n.parse().unwrap())
                .collect_vec()
                .try_into()
                .unwrap();
            let [y2, x2]: [u32; 2] = l
                .rsplit(&[',', ' '] as &[_])
                .take(2)
                .map(|n| n.parse().unwrap())
                .collect_vec()
                .try_into()
                .unwrap();

            ((x1, y1), (x2, y2))
        })
        .collect_vec();

    let bounds = lines
        .iter()
        .flat_map(|&((x1, y1), (x2, y2))| vec![(x1, y1), (x2, y2)])
        .collect_vec();

    let (xmin, xmax) = bounds
        .iter()
        .map(|&(x, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (ymin, ymax) = bounds
        .iter()
        .map(|&(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();

    let danger_points = (ymin..=ymax)
        .into_par_iter()
        .flat_map_iter(|y| {
            let lines = &lines;
            (xmin..=xmax).filter(move |&x| {
                lines
                    .par_iter()
                    .filter(|&&l| line_contains_point(l, (x, y)))
                    .count()
                    >= 2
            })
        })
        .count();

    println!("part 1: {}", danger_points);

    let danger_points = (ymin..=ymax)
        .into_par_iter()
        .flat_map_iter(|y| {
            let lines = &lines;
            (xmin..=xmax).filter(move |&x| {
                lines
                    .par_iter()
                    .filter(|&&l| line_contains_point_diag(l, (x, y)))
                    .count()
                    >= 2
            })
        })
        .count();

    println!("part 2: {}", danger_points);
}
