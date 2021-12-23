use std::io::BufRead;
use std::ops::Sub;

use itertools::{iproduct, Itertools};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    pub fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cube {
    min: Point,
    max: Point,
    v: bool,
}

impl Cube {
    fn split(self, splitter: &Cube) -> Vec<Cube> {
        if splitter.contains(&self) {
            return vec![];
        }
        if !self.intersects(splitter) {
            return vec![self];
        }

        [self]
            .into_iter()
            .flat_map(|c| c.split_face(Some((splitter.min.x, splitter.max.x)), None, None))
            .flat_map(|c| c.split_face(None, Some((splitter.min.y, splitter.max.y)), None))
            .flat_map(|c| c.split_face(None, None, Some((splitter.min.z, splitter.max.z))))
            .filter(|c| !c.intersects(splitter))
            .collect_vec()
    }

    fn split_face(
        self,
        x: Option<(i64, i64)>,
        y: Option<(i64, i64)>,
        z: Option<(i64, i64)>,
    ) -> Vec<Cube> {
        if let Some((lx, rx)) = x {
            if rx < self.min.x || self.max.x < lx {
                return vec![self];
            }

            let mut left = self.clone();
            left.max.x = lx - 1;
            let mut right = self.clone();
            right.min.x = rx + 1;
            let mut mid = self.clone();
            mid.min.x = lx.max(self.min.x);
            mid.max.x = rx.min(self.max.x);
            let left = (lx - 1 >= self.min.x).then(|| left);
            let right = (rx + 1 <= self.max.x).then(|| right);
            [left, Some(mid), right]
                .into_iter()
                .filter_map(|id| id)
                .collect_vec()
        } else if let Some((ly, ry)) = y {
            if ry < self.min.y || self.max.y < ly {
                return vec![self];
            }

            let mut left = self.clone();
            left.max.y = ly - 1;
            let mut right = self.clone();
            right.min.y = ry + 1;
            let mut mid = self.clone();
            mid.min.y = ly.max(self.min.y);
            mid.max.y = ry.min(self.max.y);

            let left = (ly - 1 >= self.min.y).then(|| left);
            let right = (ry + 1 <= self.max.y).then(|| right);
            [left, Some(mid), right]
                .into_iter()
                .filter_map(|id| id)
                .collect_vec()
        } else if let Some((lz, rz)) = z {
            if rz < self.min.z || self.max.z < lz {
                return vec![self];
            }

            let mut left = self.clone();
            left.max.z = lz - 1;
            let mut right = self.clone();
            right.min.z = rz + 1;
            let mut mid = self.clone();
            mid.min.z = lz.max(self.min.z);
            mid.max.z = rz.min(self.max.z);

            let left = (lz - 1 >= self.min.z).then(|| left);
            let right = (rz + 1 <= self.max.z).then(|| right);
            [left, Some(mid), right]
                .into_iter()
                .filter_map(|id| id)
                .collect_vec()
        } else {
            unreachable!()
        }
    }

    fn intersects(&self, other: &Cube) -> bool {
        fn not_intersects_1d(pmin: i64, pmax: i64, min: i64, max: i64) -> bool {
            pmax < min || pmin > max
        }

        !(not_intersects_1d(self.min.x, self.max.x, other.min.x, other.max.x)
            || not_intersects_1d(self.min.y, self.max.y, other.min.y, other.max.y)
            || not_intersects_1d(self.min.z, self.max.z, other.min.z, other.max.z))
    }

    fn contains(&self, other: &Cube) -> bool {
        self.min.x <= other.min.x
            && self.min.y <= other.min.y
            && self.min.z <= other.min.z
            && other.max.x <= self.max.x
            && other.max.y <= self.max.y
            && other.max.z <= self.max.z
    }
}

pub fn day22() {
    let s = include_str!("input/day22.txt");
    let steps = s
        .lines()
        .map(|l| {
            let (v, bounds) = l.split(' ').collect_tuple().unwrap();
            let v = v == "on";
            let (xmin, xmax, ymin, ymax, zmin, zmax) = bounds
                .split(',')
                .flat_map(|l| l[2..].split("..").map(|n| n.parse::<i64>().unwrap()))
                .collect_tuple()
                .unwrap();
            (
                Point::new(xmin, ymin, zmin),
                Point::new(xmax, ymax, zmax),
                v,
            )
        })
        .collect_vec();

    let mut cubes: Vec<Cube> = vec![];

    for &(min, max, v) in &steps {
        let cube = Cube { min, max, v };
        cubes = cubes
            .into_iter()
            .flat_map(|c| c.split(&cube))
            .chain([cube])
            .collect();
    }

    let count_on = cubes
        .iter()
        .filter(|c| c.v)
        .fold(0, |s, Cube { min, max, .. }| {
            s + (1 + max.x - min.x) * (1 + max.y - min.y) * (1 + max.z - min.z)
        });

    println!("count {}", count_on);
}
