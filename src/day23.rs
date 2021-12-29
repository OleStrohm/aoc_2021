use std::collections::VecDeque;
use std::fmt::Display;

use itertools::Itertools;
use rayon::iter::ParallelIterator;

pub fn day23() {
    let s = include_str!("input/day23.txt");
    let mut rooms: [_; 4] = s
        .lines()
        .nth(2)
        .unwrap()
        .chars()
        .zip(s.lines().nth(3).unwrap().chars())
        .filter(|(top, _)| ('A'..='D').contains(&top))
        .map(|(top, bottom)| vec![(bottom, false), (top, false)])
        .collect_vec()
        .try_into()
        .unwrap();

    const ROOM_SIZE: u64 = 4;

    rooms[0].insert(1, ('D', false));
    rooms[0].insert(2, ('D', false));
    rooms[1].insert(1, ('B', false));
    rooms[1].insert(2, ('C', false));
    rooms[2].insert(1, ('A', false));
    rooms[2].insert(2, ('B', false));
    rooms[3].insert(1, ('C', false));
    rooms[3].insert(2, ('A', false));

    let rooms = rooms;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct State {
        rooms: [Vec<(char, bool)>; 4],
        hallway: [Option<char>; 7],
        cost: u64,
        moves: Vec<Move>,
    }

    impl Display for State {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}\n", "#".repeat(13))?;
            write!(f, "#")?;
            for (pos, p) in self.hallway.iter().enumerate() {
                write!(f, "{}", p.unwrap_or('.'))?;
                if 1 <= pos && pos < 5 {
                    write!(f, ".")?;
                }
            }
            write!(f, "#\n")?;
            for l in (0..ROOM_SIZE).rev() {
                write!(f, "{0}{0}", if l == ROOM_SIZE - 1 { '#' } else { ' ' })?;
                for r in 0..4 {
                    write!(
                        f,
                        "#{}",
                        self.rooms[r]
                            .get(l as usize)
                            .map(|&(ch, _)| ch)
                            .unwrap_or('.')
                    )?;
                }
                write!(f, "#{0}{0}\n", if l == ROOM_SIZE - 1 { '#' } else { ' ' })?;
            }
            write!(f, "  {}\n", "#".repeat(9))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Move {
        IntoHallway(usize, usize),
        IntoRoom(usize, usize),
    }

    impl State {
        fn step(mut self, step: Move) -> State {
            self.moves.push(step);
            match step {
                Move::IntoHallway(room, hallway) => {
                    let real_hallway = match hallway {
                        0 => 0,
                        1 => 1,
                        2 => 3,
                        3 => 5,
                        4 => 7,
                        5 => 9,
                        6 => 10,
                        _ => unreachable!(),
                    };
                    let room_pos = 2 * room + 2;
                    let amphipod = self.rooms[room].pop().unwrap().0;
                    self.hallway[hallway] = Some(amphipod);
                    let dist = (real_hallway as isize - room_pos as isize).abs() as usize
                        + (ROOM_SIZE as usize - self.rooms[room].len());
                    self.cost += dist as u64 * 10_u64.pow(amphipod as u32 - 'A' as u32);
                }
                Move::IntoRoom(room, hallway) => {
                    let real_hallway = match hallway {
                        0 => 0,
                        1 => 1,
                        2 => 3,
                        3 => 5,
                        4 => 7,
                        5 => 9,
                        6 => 10,
                        _ => unreachable!(),
                    };
                    let room_pos = 2 * room + 2;
                    let dist = (real_hallway as isize - room_pos as isize).abs() as usize
                        + (ROOM_SIZE as usize - self.rooms[room].len());
                    let amphipod = self.hallway[hallway].take().unwrap();
                    self.rooms[room].push((amphipod, true));
                    self.cost += dist as u64 * 10_u64.pow(amphipod as u32 - 'A' as u32);
                }
            }
            self
        }

        fn find_moves(&self) -> Vec<Move> {
            let into_hallway = self
                .rooms
                .iter()
                .enumerate()
                .filter(|(pos, room)| room.len() > 0 && !room.last().unwrap().1)
                .flat_map(|(pos, _)| {
                    let room = pos + 1;
                    (0..7)
                        .filter(move |&hallway| {
                            let mut range = if hallway <= room {
                                hallway..=room
                            } else {
                                (room + 1)..=hallway
                            };
                            range.all(|p| self.hallway[p].is_none())
                        })
                        .map(move |hallway| Move::IntoHallway(room - 1, hallway))
                })
                .collect();

            let into_room = self
                .hallway
                .iter()
                .enumerate()
                .filter_map(|(hallway, &amphipod)| amphipod.map(|amphipod| (hallway, amphipod)))
                .filter(|&(hallway, amphipod)| {
                    let room = amphipod as usize - 'A' as usize;
                    let room_pos = 1 + room;
                    let is_movable_into = self.rooms[room]
                        .iter()
                        .all(|&(present, _)| present == amphipod);
                    let road_is_clear = if hallway <= room_pos {
                        (hallway + 1..=room_pos).all(|p| self.hallway[p].is_none())
                    } else {
                        ((room_pos + 1)..hallway).all(|p| self.hallway[p].is_none())
                    };
                    is_movable_into && road_is_clear
                })
                .map(|(hallway, amphipod)| {
                    let room = amphipod as usize - 'A' as usize;
                    Move::IntoRoom(room, hallway)
                })
                .collect_vec();

            if into_room.is_empty() {
                into_hallway
            } else {
                into_room
            }
        }

        fn finished(&self) -> bool {
            self.rooms
                .iter()
                .flat_map(|r| {
                    [
                        r.get(0).map(|&(ch, _)| ch).unwrap_or('.'),
                        r.get(1).map(|&(ch, _)| ch).unwrap_or('.'),
                        r.get(2).map(|&(ch, _)| ch).unwrap_or('.'),
                        r.get(3).map(|&(ch, _)| ch).unwrap_or('.'),
                    ]
                })
                .zip([
                    'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D',
                ])
                .all(|(l, r)| l == r)
        }

        fn min_moves_to_finish(&self) -> u64 {
            let mut rooms: [Vec<()>; 4] = {
                let mut cur_amphipod = 'A' as u8;
                self.rooms
                    .iter()
                    .map(|r| {
                        let v = r
                            .into_iter()
                            .take_while(|&&(amphipod, _)| amphipod == cur_amphipod as char)
                            .map(|_| ())
                            .collect();
                        cur_amphipod += 1;
                        v
                    })
                    .collect_vec()
                    .try_into()
                    .unwrap()
            };
            self.rooms
                .iter()
                .enumerate()
                .flat_map(|(room_id, room)| {
                    room.into_iter()
                        .enumerate()
                        .map(move |(l, &(amphipod, _))| (room_id, l as u64, amphipod))
                })
                .map(|(room, l, amphipod)| {
                    let real_room = amphipod as usize - 'A' as usize;
                    10_u64.pow(amphipod as u32 - 'A' as u32)
                        * if real_room == room {
                            if l == 0 || self.rooms[room].iter().all(|&(ch, _)| ch == amphipod) {
                                0
                            } else {
                                let cost =
                                    (ROOM_SIZE - l) + (ROOM_SIZE - rooms[room].len() as u64) + 2; // up left/right, down
                                rooms[room].push(());
                                cost as u64
                            }
                        } else {
                            let cost = (ROOM_SIZE - l)
                                + (ROOM_SIZE - rooms[real_room].len() as u64)
                                + 2 * (real_room as isize - room as isize).abs() as u64;
                            rooms[real_room].push(());
                            cost
                        }
                })
                .sum::<u64>()
                + self
                    .hallway
                    .iter()
                    .enumerate()
                    .filter_map(|(pos, p)| p.map(|ch| (pos, ch)))
                    .map(|(hallway, amphipod)| {
                        let real_hallway = match hallway {
                            0 => 0,
                            1 => 1,
                            2 => 3,
                            3 => 5,
                            4 => 7,
                            5 => 9,
                            6 => 9,
                            _ => unreachable!(),
                        };
                        let room = amphipod as usize - 'A' as usize;
                        let room_pos = 2 * room + 2;
                        let dist = (real_hallway as isize - room_pos as isize).abs() as usize
                            + (ROOM_SIZE as usize - rooms[room].len());
                        rooms[room].push(());
                        dist as u64 * 10_u64.pow(amphipod as u32 - 'A' as u32)
                    })
                    .sum::<u64>()
        }
    }

    let start = State {
        rooms,
        hallway: [None; 7],
        cost: 0,
        moves: vec![],
    };

    let mut states = VecDeque::new();
    states.push_back(start.clone());
    println!("{}", start);

    let mut th = 0;

    let min_cost = loop {
        let cur = states.pop_front().unwrap();
        println!("cur cost {}", cur.cost);
        println!("cur min cost {}", cur.min_moves_to_finish() + cur.cost);
        println!("cur state {}", cur);
        if cur.finished() {
            println!("backtrace");
            let mut bt = start.clone();
            println!("bt:\n{}", bt);
            for mo in cur.moves {
                bt = bt.step(mo);
                println!("bt:\n{}", bt);
            }
            break cur.cost;
        }
        let moves = cur.find_moves();
        //println!("moves {}", moves.len());
        for mo in moves {
            let new = cur.clone().step(mo);
            let (Ok(i) | Err(i)) = states
                .binary_search_by_key(&(new.min_moves_to_finish() + new.cost, new.cost), |s| {
                    (s.min_moves_to_finish() + s.cost, s.cost)
                });
            states.insert(i, new);
        }

        if cur.min_moves_to_finish() + cur.cost > th {
            states = states
                .into_iter()
                .dedup_by(|l, r| l.rooms == r.rooms && l.hallway == r.hallway)
                .collect();
            th += 100;
        }
    };
    println!("part 1: {}", min_cost);

    //let case1 = State {
    //    rooms: [
    //        vec![('D', false), ('D', false)],
    //        vec![('B', true), ('B', true), ('B', true), ('B', true)],
    //        vec![('C', true), ('C', true), ('C', true)],
    //        vec![('A', true), ('C', true), ('A', true), ('D', true)],
    //    ],
    //    hallway: [Some('A'), Some('D'), None, Some('A'), None, None, None],
    //    cost: 6804,
    //    moves: vec![],
    //};

    //println!("case1: {}", case1);
    //println!("case1 min: {}", case1.min_moves_to_finish() + case1.cost); // 38633
    //let moves = case1.find_moves();
    //println!("{:?}", moves);
    //let case2 = case1.step(moves[0]);
    //println!("case2: {}", case2);
    //println!("case2 min: {}", case2.min_moves_to_finish() + case2.cost);

    //   cur cost 10804
    // cur min cost 44437
    // cur state #############
    // #AD.D.A.....#
    // ###.#B#.#D###
    //   #.#B#C#A#
    //   #.#B#C#C#
    //   #D#B#C#A#
    //   #########
    //
    // cur cost 6804
    // cur min cost 45437
    // cur state #############
    // #AD...A.....#
    // ###.#B#.#D###
    //   #.#B#C#A#
    //   #D#B#C#C#
    //   #D#B#C#A#
    //   #########

    //println!("{}", case);
    //let moves = case.find_moves();
    //println!("{:?}", moves);
}
