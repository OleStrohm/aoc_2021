pub fn day3() {
    let s = include_str!("day3.txt");
    let line_count = s.lines().count() as u64;
    let line_len = s.lines().next().unwrap().len() as u64;

    let bits: Vec<Vec<u64>> = s
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
        .collect();

    let most_common = bits
        .iter()
        .fold(Vec::new(), |s, n| {
            n.iter()
                .zip(s.into_iter().chain(std::iter::repeat(0)))
                .map(|(a, b)| a + b)
                .collect()
        })
        .into_iter()
        .map(|b| if 2 * b >= line_count { 1 } else { 0 })
        .collect::<Vec<_>>();

    let gamma = most_common.iter().fold(0, |s, b| 2 * s + b);
    let epsilon = most_common.iter().fold(0, |s, b| 2 * s + (1 - b));

    println!("part 1: {}", gamma * epsilon);

    let numbers: Vec<u64> = s
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .fold(0, |s, b| 2 * s + b)
        })
        .collect();

    let oxygen = (0..line_len).rev().fold(numbers.clone(), |left, i| {
        let common = left.iter().fold(0, |s, n| s + ((n >> i) & 1));
        let common = if 2 * common >= left.len().try_into().unwrap() {
            1
        } else {
            0
        };

        if left.len() == 1 {
            left
        } else {
            left.into_iter()
                .filter(|&n| ((n >> i) & 1) == common)
                .collect()
        }
    })[0];
    let co2 = (0..line_len).rev().fold(numbers.clone(), |left, i| {
        let common = left.iter().fold(0, |s, n| s + ((n >> i) & 1));
        let uncommon = if 2 * common >= left.len().try_into().unwrap() {
            0
        } else {
            1
        };

        if left.len() == 1 {
            left
        } else {
            left.into_iter()
                .filter(|&n| ((n >> i) & 1) == uncommon)
                .collect()
        }
    })[0];

    println!("oxygen: {}", oxygen);
    println!("co2: {}", co2);
    println!("part 2: {}", oxygen * co2);
}
