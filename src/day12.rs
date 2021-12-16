use itertools::Itertools;

pub fn day12() {
    type Link = (String, String);

    let s = include_str!("day12.txt");

    let links: Vec<Link> = s
        .lines()
        .map(|l| l.split('-').map(|s| s.to_string()).collect_tuple().unwrap())
        .collect_vec();

    fn find_paths(
        path: &mut Vec<String>,
        links: &Vec<Link>,
        valid_path: impl Fn(&String, &[String]) -> bool + Copy,
    ) -> u64 {
        let mut paths = 0;
        for (l, r) in links {
            paths += match path.last() {
                Some(c) if c == l || c == r => {
                    let (cur, next) = if c == l { (l, r) } else { (r, l) };
                    if next == "end" {
                        1
                    } else if next == "start" {
                        0
                    } else if valid_path(next, path) {
                        path.push(next.clone());
                        let paths = find_paths(path, links, valid_path);
                        path.pop();
                        paths
                    } else {
                        0
                    }
                }
                _ => 0,
            }
        }

        paths
    }

    let mut v = vec![String::from("start")];
    println!(
        "part 1: {}",
        find_paths(&mut v, &links, |s, p| {
            s.chars().any(|c| c.is_ascii_uppercase()) || !p.contains(s)
        })
    );
    let mut v = vec![String::from("start")];
    println!(
        "part 2: {}",
        find_paths(&mut v, &links, |s, p| {
            s.chars().any(|c| c.is_ascii_uppercase())
                || p.iter()
                    .filter(|c| !c.chars().any(|ch| ch.is_ascii_uppercase()))
                    .counts()
                    .into_iter()
                    .all(|(_, count)| count == 1)
                || !p.contains(s)
        })
    );
}
