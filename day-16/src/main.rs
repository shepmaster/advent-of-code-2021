use std::{fmt, num::ParseIntError};

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", version_sum(INPUT));
    // WRONG: 2289229686 (too low) -- Shifted values off the left and lost them
    println!("part2: {}", evaluate(INPUT));
}

fn version_sum(hex: &str) -> u64 {
    let bits = bit_stream(hex);
    let packet = parse(&mut &*bits);
    packet.sum_of_versions()
}

fn evaluate(hex: &str) -> u64 {
    let bits = bit_stream(hex);
    let mut cursor = &*bits;
    let packet = parse(&mut cursor);
    assert!(cursor.chars().all(|c| c == '0'));
    packet.eval()
}

fn bit_stream(hex: &str) -> String {
    hex.trim()
        .chars()
        .flat_map(|c| match c {
            '0' => b"0000",
            '1' => b"0001",
            '2' => b"0010",
            '3' => b"0011",
            '4' => b"0100",
            '5' => b"0101",
            '6' => b"0110",
            '7' => b"0111",
            '8' => b"1000",
            '9' => b"1001",
            'A' => b"1010",
            'B' => b"1011",
            'C' => b"1100",
            'D' => b"1101",
            'E' => b"1110",
            'F' => b"1111",
            o => panic!("bad hex {o}"),
        })
        .map(|&c| c as char)
        .collect()
}

#[derive(Debug)]
enum Packet {
    Sum(u64, Vec<Packet>),
    Product(u64, Vec<Packet>),
    Minimum(u64, Vec<Packet>),
    Maximum(u64, Vec<Packet>),
    GreaterThan(u64, Vec<Packet>),
    LessThan(u64, Vec<Packet>),
    EqualTo(u64, Vec<Packet>),
    Literal(u64, u64),
}

impl Packet {
    fn sum_of_versions(&self) -> u64 {
        use Packet::*;

        match self {
            Sum(v, c)
            | Product(v, c)
            | Minimum(v, c)
            | Maximum(v, c)
            | GreaterThan(v, c)
            | LessThan(v, c)
            | EqualTo(v, c) => c.iter().map(Packet::sum_of_versions).sum::<u64>() + v,
            Literal(v, _) => *v,
        }
    }

    fn eval(&self) -> u64 {
        use Packet::*;

        match self {
            Sum(_, c) => c.iter().map(Packet::eval).sum(),
            Product(_, c) => c.iter().map(Packet::eval).product(),
            Minimum(_, c) => c.iter().map(Packet::eval).min().expect("min of zero items"),
            Maximum(_, c) => c.iter().map(Packet::eval).max().expect("max of zero items"),
            GreaterThan(_, c) => (c[0].eval() > c[1].eval()) as u64,
            LessThan(_, c) => (c[0].eval() < c[1].eval()) as u64,
            EqualTo(_, c) => (c[0].eval() == c[1].eval()) as u64,
            &Literal(_, v) => v,
        }
    }

    #[cfg(test)]
    fn children(&self) -> &[Packet] {
        use Packet::*;

        match self {
            Sum(_, c)
            | Product(_, c)
            | Minimum(_, c)
            | Maximum(_, c)
            | GreaterThan(_, c)
            | LessThan(_, c)
            | EqualTo(_, c) => c,
            Literal(..) => &[],
        }
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Packet::*;

        let mut children = |op, c| {
            write!(f, "({op}")?;
            for c in c {
                write!(f, " {c}")?;
            }
            write!(f, ")")
        };

        match self {
            Sum(_, c) => children("+", c),
            Product(_, c) => children("*", c),
            Minimum(_, c) => children("min", c),
            Maximum(_, c) => children("max", c),
            GreaterThan(_, c) => children(">", c),
            LessThan(_, c) => children("<", c),
            EqualTo(_, c) => children("=", c),
            Literal(_, v) => v.fmt(f),
        }
    }
}

fn parse(bits: &mut &str) -> Packet {
    use Packet::*;

    let version = bits.parse_bits(3).expect("invalid version");
    let ty = bits.parse_bits(3).expect("invalid type");

    match ty {
        0 => Sum(version, parse_operator_children(bits)),
        1 => Product(version, parse_operator_children(bits)),
        2 => Minimum(version, parse_operator_children(bits)),
        3 => Maximum(version, parse_operator_children(bits)),
        4 => Literal(version, parse_literal_value(bits)),
        5 => GreaterThan(version, parse_operator_children(bits)),
        6 => LessThan(version, parse_operator_children(bits)),
        7 => EqualTo(version, parse_operator_children(bits)),

        o => panic!("undefined operator {o}"),
    }
}

fn parse_literal_value(bits: &mut &str) -> u64 {
    let mut value = 0;
    loop {
        let keep_going = bits.parse_bits(1).expect("invalid keep going") == 1;
        let v = bits.parse_bits(4).expect("invalid literal value");

        value = value << 4 | v;
        if !keep_going {
            return value;
        }
    }
}

fn parse_operator_children(bits: &mut &str) -> Vec<Packet> {
    let length_type_id = bits.parse_bits(1).expect("invalid length type id");
    match length_type_id {
        0 => {
            let n_bits = bits.parse_bits(15).expect("invalid length bits");
            let n_bits = usize::try_from(n_bits).expect("value doesn't fit in usize");
            let (mut child_bits, rest) = bits.split_at(n_bits);

            let mut children = vec![];
            while !child_bits.is_empty() {
                children.push(parse(&mut child_bits));
            }
            *bits = rest;

            children
        }
        1 => {
            let n_packets = bits.parse_bits(11).expect("invalid length packets");
            (0..n_packets).map(|_| parse(bits)).collect()
        }
        o => panic!("undefined length type id {o}"),
    }
}

trait Parsing {
    fn parse_bits(&mut self, n_bits: usize) -> Result<u64, ParseIntError>;
}

impl Parsing for &str {
    fn parse_bits(&mut self, n_bits: usize) -> Result<u64, ParseIntError> {
        let (val, bits) = self.split_at(n_bits);
        let val = u64::from_str_radix(val, 2)?;
        *self = bits;
        Ok(val)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(16, version_sum("8A004A801A8002F478"));
        assert_eq!(12, version_sum("620080001611562C8802118E34"));
        assert_eq!(23, version_sum("C0015000016115A2E0802F182340"));
        assert_eq!(31, version_sum("A0016C880162017C3686B18A3D4780"));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(3, evaluate("C200B40A82"));
        assert_eq!(54, evaluate("04005AC33890"));
        assert_eq!(7, evaluate("880086C3E88112"));
        assert_eq!(9, evaluate("CE00C43D881120"));
        assert_eq!(1, evaluate("D8005AC2A8F0"));
        assert_eq!(0, evaluate("F600BC2D8F"));
        assert_eq!(0, evaluate("9C005AC2F8F0"));
        assert_eq!(1, evaluate("9C0141080250320F1802104A08"));
    }

    #[test]
    fn test_n_children() {
        let bits = bit_stream("38006F45291200");
        let packet = parse(&mut &*bits);
        assert_eq!(2, packet.children().len());

        let bits = bit_stream("EE00D40C823060");
        let packet = parse(&mut &*bits);
        assert_eq!(3, packet.children().len());
    }
}
