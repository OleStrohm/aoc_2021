use itertools::{FoldWhile, Itertools};

fn y_after_steps(yvel: i64, steps: i64) -> i64 {
    yvel * steps - steps * (steps - 1) / 2
}

fn x_after_steps(xvel: i64, steps: i64) -> i64 {
    xvel * (xvel + 1) / 2
        - if steps > xvel {
            0
        } else {
            (xvel - steps) * (xvel - steps + 1) / 2
        }
}

fn hits_target(xvel: i64, yvel: i64, xmin: i64, xmax: i64, ymin: i64, ymax: i64) -> Option<i64> {
    (0..)
        .fold_while(((0, 0), None), |((x, y), _), s| {
            if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
                FoldWhile::Done(((x, y), Some(s)))
            } else if x > xmax || y < ymin {
                FoldWhile::Done(((x, y), None))
            } else {
                FoldWhile::Continue(((x_after_steps(xvel, s), y_after_steps(yvel, s)), None))
            }
        })
        .into_inner()
        .1
}

fn hits_target_2(xvel: i64, yvel: i64, xmin: i64, xmax: i64, ymin: i64, ymax: i64) -> Option<i64> {
    let s_zero = 0.max(1 + 2 * yvel);
    let s_min = (yvel as f64 + 0.5 + ((0.5 + yvel as f64).powi(2) - 2.0 * ymax as f64).sqrt())
        .floor() as i64;
    let s_max = (yvel as f64 + 0.5 + ((0.5 + yvel as f64).powi(2) - 2.0 * ymin as f64).sqrt())
        .ceil() as i64;

    (s_min..=s_max).find(|&s| {
        let x = x_after_steps(xvel, s);
        let y = y_after_steps(yvel, s);
        x >= xmin && x <= xmax && y >= ymin && y <= ymax
    })
}

pub fn day17() {
    let s = include_str!("input/day17.txt");
    let (xmin, xmax) = s[s.find("x=").unwrap() + 2..s.find(", y=").unwrap()]
        .split("..")
        .collect_tuple()
        .map(|(min, max)| (min.parse::<i64>().unwrap(), max.parse::<i64>().unwrap()))
        .unwrap();
    let (ymin, ymax) = s[s.find("y=").unwrap() + 2..]
        .trim_end()
        .split("..")
        .collect_tuple()
        .map(|(min, max)| (min.parse::<i64>().unwrap(), max.parse::<i64>().unwrap()))
        .unwrap();

    println!(
        "part 1 {:?}",
        (-90..=90)
            .flat_map(|yvel| {
                (0..=251).filter_map(move |xvel| {
                    hits_target_2(xvel, yvel, xmin, xmax, ymin, ymax).map(|s| {
                        (0..s)
                            .map(|s| (xvel, yvel, x_after_steps(xvel, s), y_after_steps(yvel, s)))
                            .take_while(|&(_, _, x, y)| y >= ymin && x <= xmax)
                            .max_by_key(|&(_, _, _, y)| y)
                            .unwrap()
                    })
                })
            })
            .max_by_key(|&(_, _, _, y)| y).unwrap().3
    );

    let velocities = (-90..=90)
        .flat_map(|yvel| {
            (0..=251)
                .filter(move |&xvel| hits_target_2(xvel, yvel, xmin, xmax, ymin, ymax).is_some())
        })
        .count();

    println!("part 2: {}", velocities);
}
