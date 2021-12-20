use std::io::BufRead;

use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Rot {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Orientation {
    PX(Rot),
    PY(Rot),
    PZ(Rot),
    NX(Rot),
    NY(Rot),
    NZ(Rot),
}

fn into_orient((tx, ty, tz): (i64, i64, i64), orient: Orientation) -> (i64, i64, i64) {
    fn rot_2d((p0, p1): (i64, i64), rot: Rot) -> (i64, i64) {
        match rot {
            Rot::Up => (p0, p1),
            Rot::Right => (-p1, p0),
            Rot::Down => (-p0, -p1),
            Rot::Left => (p1, -p0),
        }
    }

    match orient {
        Orientation::PX(r) => {
            let (y, z) = rot_2d((ty, tz), r);
            (tx, y, z)
        }
        Orientation::NX(r) => {
            let (y, z) = rot_2d((-ty, tz), r);
            (-tx, y, z)
        }
        Orientation::PY(r) => {
            let (x, z) = rot_2d((-ty, tz), r);
            (x, tx, z)
        }
        Orientation::NY(r) => {
            let (x, z) = rot_2d((ty, tz), r);
            (x, -tx, z)
        }
        Orientation::PZ(r) => {
            let (x, y) = rot_2d((ty, tz), r);
            (x, y, tx)
        }
        Orientation::NZ(r) => {
            let (x, y) = rot_2d((-ty, tz), r);
            (x, y, -tx)
        }
    }
}

fn most_likely_diff(
    reference: &[(i64, i64, i64)],
    target: &[(i64, i64, i64)],
) -> (((i64, i64, i64), Orientation), usize) {
    let all_orients = [
        Orientation::PX(Rot::Up),
        Orientation::PX(Rot::Right),
        Orientation::PX(Rot::Down),
        Orientation::PX(Rot::Left),
        Orientation::PY(Rot::Up),
        Orientation::PY(Rot::Right),
        Orientation::PY(Rot::Down),
        Orientation::PY(Rot::Left),
        Orientation::PZ(Rot::Up),
        Orientation::PZ(Rot::Right),
        Orientation::PZ(Rot::Down),
        Orientation::PZ(Rot::Left),
        Orientation::NX(Rot::Up),
        Orientation::NX(Rot::Right),
        Orientation::NX(Rot::Down),
        Orientation::NX(Rot::Left),
        Orientation::NY(Rot::Up),
        Orientation::NY(Rot::Right),
        Orientation::NY(Rot::Down),
        Orientation::NY(Rot::Left),
        Orientation::NZ(Rot::Up),
        Orientation::NZ(Rot::Right),
        Orientation::NZ(Rot::Down),
        Orientation::NZ(Rot::Left),
    ];

    reference
        .iter()
        .cartesian_product(target)
        .cartesian_product(&all_orients)
        .map(|((&p0, &p1), &orient)| (p0, into_orient(p1, orient), orient))
        .map(|((x0, y0, z0), (x1, y1, z1), orient)| ((x0 - x1, y0 - y1, z0 - z1), orient))
        .sorted()
        .group_by(|&id| id)
        .into_iter()
        .map(|(_, g)| g.collect_vec())
        .map(|g| (g[0], g.len()))
        .max_by_key(|&(_, len)| len)
        .unwrap()
}

fn from_a_to_b(ab: ((i64, i64, i64), Orientation), a: (i64, i64, i64)) -> (i64, i64, i64) {
    let d = into_orient(a, ab.1);
    let pa = ab.0;
    (pa.0 + d.0, pa.1 + d.1, pa.2 + d.2)
}

fn relative_to_zero(
    p: (i64, i64, i64),
    scanner: usize,
    conversions: &HashMap<(usize, usize), ((i64, i64, i64), Orientation)>,
) -> (i64, i64, i64) {
    fn relative_to_zero_inner(
        scanner: usize,
        conversions: &HashMap<(usize, usize), ((i64, i64, i64), Orientation)>,
        bt: &mut Vec<usize>,
    ) -> bool {
        if scanner == 0 {
            return true;
        }
        for (t, r) in conversions.keys() {
            if bt.contains(r) || *t != scanner {
                continue;
            }
            bt.push(scanner);
            if relative_to_zero_inner(*r, conversions, bt) {
                return true;
            }
            bt.pop();
        }

        return false;
    }

    if scanner == 0 {
        return p;
    }
    let mut bt = vec![];
    relative_to_zero_inner(scanner, conversions, &mut bt);
    bt.push(0);
    bt.reverse();

    let mut cur_ref = bt.pop().unwrap();
    let mut p = p;

    while let Some(next_ref) = bt.pop() {
        p = from_a_to_b(*conversions.get(&(cur_ref, next_ref)).unwrap(), p);
        cur_ref = next_ref;
    }

    p
}

pub fn day19() {
    let s = include_str!("input/day19.txt");
    let scanners: Vec<Vec<(i64, i64, i64)>> = s
        .lines()
        .group_by(|l| !l.is_empty())
        .into_iter()
        .filter_map(|(k, g)| k.then(|| g))
        .map(|g| {
            g.into_iter()
                .skip(1)
                .map(|l| {
                    l.split(',')
                        .map(|n| n.parse::<i64>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();

    let conversions = (0..scanners.len())
        .cartesian_product(0..scanners.len())
        .filter(|(r, t)| r != t)
        .map(|(r, t)| ((t, r), most_likely_diff(&scanners[r], &scanners[t])))
        .filter(|&(_, (_, confidence))| confidence >= 6)
        .map(|c @ (k, (v, _))| (k, v))
        .collect::<HashMap<_, _>>();

    let num_beacons = scanners
        .iter()
        .enumerate()
        .flat_map(|(scanner, beacons)| {
            let conversions = &conversions;
            beacons
                .iter()
                .map(move |&p| relative_to_zero(p, scanner, conversions))
        })
        .sorted()
        .dedup()
        .count();

    println!("part 1: {}", num_beacons);

    let scanner_positions = (0..scanners.len())
        .map(|scanner| relative_to_zero((0, 0, 0), scanner, &conversions))
        .collect_vec();

    let max_distance = scanner_positions
        .iter()
        .cartesian_product(scanner_positions.iter())
        .map(|((x0, y0, z0), (x1, y1, z1))| (x0 - x1).abs() + (y0 - y1).abs() + (z0 - z1).abs())
        .max()
        .unwrap();

    println!("part 2: {}", max_distance);
}
