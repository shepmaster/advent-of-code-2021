use std::num::ParseIntError;

const INPUT: &str = include_str!("../input");

fn main() {
    println!("part1: {}", version_sum(INPUT));
}

fn version_sum(hex: &str) -> u16 {
    let bits: String = hex
        .trim()
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
        .collect();

    let packet = parse(&mut &*bits);

    packet.sum_of_versions()
}

#[derive(Debug)]
enum Packet {
    Operator(u16, Vec<Packet>),
    Literal(u16, u16),
}

impl Packet {
    fn sum_of_versions(&self) -> u16 {
        match self {
            Packet::Operator(v, c) => c.iter().map(Packet::sum_of_versions).sum::<u16>() + v,
            Packet::Literal(v, _) => *v,
        }
    }
}

fn parse(bits: &mut &str) -> Packet {
    let version = bits.parse_bits(3).expect("invalid version");
    let ty = bits.parse_bits(3).expect("invalid type");

    match ty {
        4 => {
            let mut value = 0;
            loop {
                let keep_going = bits.parse_bits(1).expect("invalid keep going") == 1;
                let v = bits.parse_bits(4).expect("invalid literal value");

                value = value << 4 | v;
                if !keep_going {
                    break;
                }
            }
            Packet::Literal(version, value)
        }
        _ => {
            let length_type_id = bits.parse_bits(1).expect("invalid length type id");

            let children = match length_type_id {
                0 => {
                    let n_bits = bits.parse_bits(15).expect("invalid length bits");
                    let n_bits = usize::from(n_bits);
                    let (mut child_bits, rest) = bits.split_at(n_bits);

                    let mut children = vec![];
                    while !child_bits.is_empty() {
                        children.push(parse(&mut child_bits));
                    }
                    *bits = rest;

                    children
                }
                _ => {
                    let n_packets = bits.parse_bits(11).expect("invalid length packets");
                    (0..n_packets).map(|_| parse(bits)).collect()
                }
            };

            Packet::Operator(version, children)
        }
    }
}

trait Parsing {
    fn parse_bits(&mut self, n_bits: usize) -> Result<u16, ParseIntError>;
}

impl Parsing for &str {
    fn parse_bits(&mut self, n_bits: usize) -> Result<u16, ParseIntError> {
        let (val, bits) = self.split_at(n_bits);
        let val = u16::from_str_radix(val, 2)?;
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
}
