use itertools::Itertools;

pub fn day16() {
    let s = include_str!("day16.txt");
    let mut bits = s.trim_end().chars().flat_map(|ch| {
        let digit = ch.to_digit(16).unwrap() as u8;
        [
            (digit >> 3) & 1,
            (digit >> 2) & 1,
            (digit >> 1) & 1,
            digit & 1,
        ]
    });
    let bits = &mut bits;

    #[derive(Debug, Clone)]
    enum Packet {
        Literal(u64, u8),
        Op(u8, Vec<Packet>, u8),
    }

    fn parse_literal(ver: u8, bits: &mut impl Iterator<Item = u8>) -> Packet {
        let mut literal = 0;
        while let Some(bit) = bits.next() {
            literal = bits.take(4).fold(literal, |s, b| 2 * s + b as u64);
            if bit == 0 {
                break;
            }
        }
        Packet::Literal(literal, ver)
    }

    fn parse_op(ver: u8, id: u8, bits: &mut impl Iterator<Item = u8>) -> Packet {
        let len_type = bits.next().unwrap();

        let packets = if len_type == 0 {
            let len_of_packets = bits.take(15).fold(0, |s, b| 2 * s + b as u64);
            let inner = bits.take(len_of_packets as usize).collect_vec();
            let mut inner = inner.into_iter().peekable();
            let mut packets = Vec::new();
            while let Some(_) = inner.peek() {
                packets.push(parse_packet(&mut inner));
            }
            packets
        } else {
            let num_packets = bits.take(11).fold(0, |s, b| 2 * s + b as u64);
            (0..num_packets).map(|_| parse_packet(bits)).collect_vec()
        };

        Packet::Op(id, packets, ver)
    }

    fn parse_packet(bits: &mut impl Iterator<Item = u8>) -> Packet {
        let ver = bits.take(3).fold(0, |s, b| 2 * s + b);
        let id = bits.take(3).fold(0, |s, b| 2 * s + b);

        match id {
            4 => parse_literal(ver, bits),
            _ => parse_op(ver, id, bits),
        }
    }

    fn sum_versions(p: &Packet) -> u64 {
        match p {
            Packet::Literal(_, ver) => *ver as u64,
            Packet::Op(_, packets, ver) => {
                *ver as u64 + packets.iter().map(sum_versions).sum::<u64>()
            }
        }
    }

    fn eval_packet(p: &Packet) -> u64 {
        match p {
            Packet::Literal(lit, _) => *lit,
            Packet::Op(0, packets, _) => packets.iter().map(eval_packet).sum(),
            Packet::Op(1, packets, _) => packets.iter().map(eval_packet).product(),
            Packet::Op(2, packets, _) => packets.iter().map(eval_packet).min().unwrap(),
            Packet::Op(3, packets, _) => packets.iter().map(eval_packet).max().unwrap(),
            Packet::Op(5, packets, _) => packets
                .iter()
                .map(eval_packet)
                .collect_tuple()
                .map(|(fst, snd)| (fst > snd) as u64)
                .unwrap(),
            Packet::Op(6, packets, _) => packets
                .iter()
                .map(eval_packet)
                .collect_tuple()
                .map(|(fst, snd)| (fst < snd) as u64)
                .unwrap(),
            Packet::Op(7, packets, _) => packets
                .iter()
                .map(eval_packet)
                .collect_tuple()
                .map(|(fst, snd)| (fst == snd) as u64)
                .unwrap(),
            _ => unreachable!(),
        }
    }

    let packet = parse_packet(bits);
    println!("part 1: {}", sum_versions(&packet));
    println!("part 2: {}", eval_packet(&packet));
}
