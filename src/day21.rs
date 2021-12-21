use itertools::{fold, Itertools};

#[derive(Clone, Copy)]
enum Turn {
    P1,
    P2,
}

impl Turn {
    pub fn step(self) -> Self {
        match self {
            Turn::P1 => Turn::P2,
            Turn::P2 => Turn::P1,
        }
    }
}

struct DeterministicDice {
    next_number: u64,
    rolled: u64,
}

impl DeterministicDice {
    pub fn roll3(self) -> (u64, DeterministicDice) {
        let roll3 = self.next_number + 1;
        let next_number = (self.next_number + 1) % 100;
        let roll3 = roll3 + next_number + 1;
        let next_number = (next_number + 1) % 100;
        let roll3 = roll3 + next_number + 1;
        let next_number = (next_number + 1) % 100;
        (
            roll3,
            DeterministicDice {
                next_number,
                rolled: self.rolled + 3,
            },
        )
    }

    pub fn rolled(&self) -> u64 {
        self.rolled
    }
}

#[derive(Clone, Copy)]
struct Player {
    pos: u64,
    score: u64,
}

impl Player {
    pub fn step(self, roll: u64) -> Self {
        let pos = (self.pos + roll) % 10;
        let pos = if pos == 0 { 10 } else { pos };
        let score = self.score + pos;
        Player { pos, score }
    }

    pub fn score(&self) -> u64 {
        self.score
    }
}

pub fn day21() {
    let s = include_str!("input/day21.txt");
    let (p1, p2) = s
        .lines()
        .map(|l| {
            l.chars()
                .rev()
                .take_while(|ch| !ch.is_whitespace())
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .map(|pos| Player { pos, score: 0 })
        .collect_tuple()
        .unwrap();

    let dice = DeterministicDice {
        next_number: 0,
        rolled: 0,
    };

    let (final_p1, final_p2, turn, dice) = (0..)
        .fold_while((p1, p2, Turn::P1, dice), |(p1, p2, turn, dice), _| {
            let (roll, dice) = dice.roll3();
            let (p1, p2) = match turn {
                Turn::P1 => (p1.step(roll), p2),
                Turn::P2 => (p1, p2.step(roll)),
            };
            if p1.score().max(p2.score()) >= 1000 {
                itertools::FoldWhile::Done((p1, p2, turn, dice))
            } else {
                itertools::FoldWhile::Continue((p1, p2, turn.step(), dice))
            }
        })
        .into_inner();

    let part1 = match turn {
        Turn::P1 => final_p2.score() * dice.rolled(),
        Turn::P2 => final_p1.score() * dice.rolled(),
    };
    println!("part 1: {}", part1);

    fn dirac_roll() -> impl Iterator<Item = (u64, u64)> {
        [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)].into_iter()
    }

    fn how_many_wins(p1: Player, p2: Player, turn: Turn, universes: u64) -> (u64, u64) {
        dirac_roll().fold((0, 0), |(p1_wins, p2_wins), (roll, universe_multiplier)| {
            let universes = universes * universe_multiplier;
            //println!(
            //    "universes {} | roll {} | score p1 {} | score p2 {}",
            //    universes,
            //    roll,
            //    p1.score(),
            //    p2.score()
            //);
            let (p1, p2) = match turn {
                Turn::P1 => (p1.step(roll), p2),
                Turn::P2 => (p1, p2.step(roll)),
            };
            let (new_p1_wins, new_p2_wins) = if p1.score().max(p2.score()) >= 21 {
                match turn {
                    Turn::P1 => (universes, 0),
                    Turn::P2 => (0, universes),
                }
            } else {
                how_many_wins(p1, p2, turn.step(), universes)
            };
            (p1_wins + new_p1_wins, p2_wins + new_p2_wins)
        })
    }

    println!("part 2: {:?}", how_many_wins(p1, p2, Turn::P1, 1));
}
