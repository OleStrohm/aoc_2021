pub fn day1() {
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
