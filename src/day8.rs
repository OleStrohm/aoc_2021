use itertools::Itertools;

pub fn day8() {
    let s = include_str!("day8.txt");
    let (input, output): (Vec<_>, Vec<_>) = s
        .lines()
        .map(|l| l.split(" | ").collect_tuple::<(&str, &str)>().unwrap())
        .unzip();

    let num_1478 = output
        .iter()
        .flat_map(|l| l.split(" "))
        .filter(|n| matches!(n.len(), 2 | 3 | 4 | 7))
        .count();
    println!("part 1: {}", num_1478);

    let sum = input
        .into_iter()
        .zip(output.into_iter())
        .fold(0, |s, (i, o)| {
            let input = i.split(' ').collect_vec();

            fn fits_in(inner: &str, outer: &str) -> bool {
                inner.chars().all(|c| outer.contains(c))
            }
            fn equals(a: &str, b: &str) -> bool {
                fits_in(a, b) && fits_in(b, a)
            }
            fn extract<'a>(collection: &'_ [&'a str], pred: impl FnMut(&&&str) -> bool) -> &'a str {
                collection.into_iter().filter(pred).next().unwrap()
            }

            let one = extract(&input, |n| n.len() == 2);
            let seven = extract(&input, |n| n.len() == 3);
            let four = extract(&input, |n| n.len() == 4);
            let eight = extract(&input, |n| n.len() == 7);

            let one_of_235 = input.iter().filter(|n| n.len() == 5).copied().collect_vec();
            let one_of_069 = input.iter().filter(|n| n.len() == 6).copied().collect_vec();

            let (zero, six, nine) = one_of_069
                .into_iter()
                .sorted_by_key(|n| one_of_235.iter().filter(|i| fits_in(i, n)).count())
                .collect_tuple()
                .unwrap();

            let five = extract(&one_of_235, |n| fits_in(n, six));
            let three = extract(&one_of_235, |n| fits_in(one, n));

            let mapping = vec![
                (zero, 0),
                (one, 1),
                (three, 3),
                (four, 4),
                (five, 5),
                (six, 6),
                (seven, 7),
                (eight, 8),
                (nine, 9),
            ];

            let n = o
                .split(' ')
                .rev()
                .enumerate()
                .map(|(i, n)| {
                    10_u64.pow(i as _)
                        * mapping
                            .iter()
                            .find(|(c, _)| equals(n, c))
                            .map_or(2, |(_, v)| *v)
                })
                .sum::<u64>();

            s + n
        });

    println!("part 2: {}", sum);
}
