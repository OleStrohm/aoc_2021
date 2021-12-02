#![allow(dead_code)]

fn main() {
    day2();
}

enum Movement {
    Up(i64),
    Down(i64),
    Forward(i64),
}

fn day2() {
    let s = include_str!("day2.txt");
    let movements = s
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split(" ").collect();
            let dist = parts[1].parse().unwrap();
            match parts[0] {
                "up" => Movement::Up(dist),
                "down" => Movement::Down(dist),
                "forward" => Movement::Forward(dist),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();

    let (horizontal, depth) = movements.iter().fold((0, 0), |(h, d), m| match m {
        Movement::Up(dist) => (h, d - dist),
        Movement::Down(dist) => (h, d + dist),
        Movement::Forward(dist) => (h + dist, d),
    });

    println!("part 1: {}", horizontal * depth);

    let (_, horizontal, depth) = movements.iter().fold((0, 0, 0), |(aim, h, d), m| match m {
        Movement::Up(dist) => (aim - dist, h, d),
        Movement::Down(dist) => (aim + dist, h, d),
        Movement::Forward(dist) => (aim, h + dist, d + aim * dist),
    });

    println!("part 2: {}", horizontal * depth);
}

fn day1() {
    let s = include_str!("day1.txt");
    let ns = s
        .lines()
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    // part 1
    let increases = ns.windows(2).filter(|vals| vals[0] < vals[1]).count();
    println!("part 1: {}", increases);

    // part 2
    let windowed_ns = ns
        .windows(3)
        .map(|ns| ns.iter().sum())
        .collect::<Vec<u64>>();
    let increases = windowed_ns
        .windows(2)
        .filter(|vals| vals[0] < vals[1])
        .count();
    println!("part 2: {}", increases);
}
