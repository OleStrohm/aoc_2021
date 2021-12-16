pub fn day2() {
    let s = include_str!("day2.txt");

    enum Movement {
        Up(i64),
        Down(i64),
        Forward(i64),
    }

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
