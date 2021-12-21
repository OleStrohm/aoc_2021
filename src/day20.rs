use itertools::Itertools;

pub fn day20() {
    let s = include_str!("input/day20.txt");
    let substitutions = s
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(|ch| (ch == '#') as u8)
        .collect_vec();

    let mut image = s
        .lines()
        .skip(2)
        .map(|l| l.chars().map(|ch| (ch == '#') as u8).collect_vec())
        .collect_vec();

    fn enhance(mut image: Vec<Vec<u8>>, boundary: u8, substitutions: &[u8]) -> Vec<Vec<u8>> {
        let mut width = image[0].len();
        let mut height = image.len();
        if image.first().unwrap().iter().any(|&p| p != boundary) {
            image.insert(0, vec![boundary; width]);
            height += 1;
        }
        if image.last().unwrap().iter().any(|&p| p != boundary) {
            image.push(vec![boundary; width]);
            height += 1;
        }
        if image.iter().any(|l| *l.first().unwrap() != boundary) {
            image.iter_mut().for_each(|l| l.insert(0, boundary));
            width += 1;
        }
        if image.iter().any(|l| *l.last().unwrap() != boundary) {
            image.iter_mut().for_each(|l| l.push(boundary));
            width += 1;
        }

        let image = (0..height)
            .map(|y| {
                let map = &image;
                let substitutions = &substitutions;
                (0..width)
                    .map(move |x| {
                        let index = [
                            (x.checked_sub(1), y.checked_sub(1)),
                            (Some(x), y.checked_sub(1)),
                            (x.checked_add(1), y.checked_sub(1)),
                            (x.checked_sub(1), Some(y)),
                            (Some(x), Some(y)),
                            (x.checked_add(1), Some(y)),
                            (x.checked_sub(1), y.checked_add(1)),
                            (Some(x), y.checked_add(1)),
                            (x.checked_add(1), y.checked_add(1)),
                        ]
                        .into_iter()
                        .map(|(x, y)| map.get(y?)?.get(x?))
                        .map(|v| v.copied().unwrap_or(boundary))
                        .fold(0, |s, b| 2 * s + b as usize);

                        substitutions[index]
                    })
                    .collect_vec()
            })
            .collect_vec();

        for row in &image {
            for &p in row {
                print!("{}", if p == 1 { '#' } else { '.' });
            }
            println!();
        }
        println!();

        image
    }

    println!(
        "part 1: {}",
        (0..2)
            .fold((image.clone(), 0), |(image, boundary), i| {
                (
                    enhance(image, boundary, &substitutions),
                    if boundary == 0 {
                        *substitutions.first().unwrap()
                    } else {
                        *substitutions.last().unwrap()
                    },
                )
            })
            .0
            .iter()
            .flat_map(|r| r.iter())
            .filter(|&&p| p == 1)
            .count()
    );

    println!(
        "part 2: {}",
        (0..50)
            .fold((image, 0), |(image, boundary), i| {
                (
                    enhance(image, boundary, &substitutions),
                    if boundary == 0 {
                        *substitutions.first().unwrap()
                    } else {
                        *substitutions.last().unwrap()
                    },
                )
            })
            .0
            .iter()
            .flat_map(|r| r.iter())
            .filter(|&&p| p == 1)
            .count()
    );
}
