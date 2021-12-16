use itertools::Itertools;
use std::ops::Sub;

pub fn day7() {
    let s = include_str!("day7.txt");
    let positions = s
        .trim_end()
        .split(',')
        .map(|n| dbg!(n).parse::<i64>().unwrap())
        .sorted()
        .collect_vec();
    let (&min, &max) = positions.iter().minmax().into_option().unwrap();
    let least_fuel = (min..=max)
        .map(|p| positions.iter().map(|cp| cp.sub(p).abs()).sum::<i64>())
        .min()
        .unwrap();
    println!("part 1: {:?}", least_fuel,);

    fn sum_1_to_n(n: i64) -> i64 {
        n * (n + 1) / 2
    }

    let least_fuel = (min..=max)
        .map(|p| {
            positions
                .iter()
                .map(|cp| sum_1_to_n(cp.sub(p).abs()))
                .sum::<i64>()
        })
        .min()
        .unwrap();
    println!("part 2: {:?}", least_fuel,);
}
