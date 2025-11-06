#![feature(test)]

use advent_lib::{parsing::hex8, *};
use bitstream_io::{BigEndian, BitRead, BitReader};
use nom::{error::ErrorKind, multi::many1, Parser};
use nom_parse_trait::ParseFrom;
use std::io::Error as IOError;

#[derive(Debug, Clone)]
enum Packet {
    Op { v: u32, op: OpType, ps: Vec<Packet> },
    Lit { v: u32, val: u64 },
}

#[derive(Debug, Clone, Copy)]
enum OpType {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl<I, E> ParseFrom<I, E> for Packet
where
    E: nom::error::ParseError<I>,
    I: nom::Input + nom::AsBytes + nom::Offset,
    <I as nom::Input>::Item: nom::AsChar + Copy,
    <I as nom::Input>::Iter: Clone,
    I: for<'a> nom::Compare<&'a [u8]>,
    I: nom::Compare<&'static str>,
    for<'a> &'a str: nom::FindToken<<I as nom::Input>::Item>,
{
    fn parse(input: I) -> nom::IResult<I, Self, E> {
        let (rest, bs) = many1(hex8).parse(input.clone())?;
        let (packet, _) = Packet::read_from(&mut BitReader::endian(bs.as_slice(), BigEndian))
            .map_err(|_| nom::Err::Error(E::from_error_kind(input, ErrorKind::Switch)))?;
        Ok((rest, packet))
    }
}

impl Packet {
    fn read_from(bits: &mut BitReader<&[u8], BigEndian>) -> Result<(Packet, u64), IOError> {
        let v = bits.read::<3, u32>()?;
        let typ = bits.read::<3, u8>()?;
        let mut read_bits = 6;

        if typ == 4 {
            let mut value = 0;
            let mut read_more = true;
            while read_more {
                read_more = bits.read_bit()?;
                value <<= 4;
                value |= bits.read::<4, u64>()?;
                read_bits += 5;
            }
            Ok((Packet::Lit { v, val: value }, read_bits))
        } else {
            let op = match typ {
                0 => OpType::Sum,
                1 => OpType::Product,
                2 => OpType::Min,
                3 => OpType::Max,
                5 => OpType::GreaterThan,
                6 => OpType::LessThan,
                7 => OpType::EqualTo,
                _ => return Err(IOError::other("Unknown operation type")),
            };

            let mut ps = vec![];
            if bits.read_bit()? {
                let nr_of_packets = bits.read::<11, u64>()?;
                read_bits += 12;
                for _ in 0..nr_of_packets {
                    let (packet, rb) = Packet::read_from(bits)?;
                    ps.push(packet);
                    read_bits += rb;
                }
            } else {
                let mut nr_of_bits = bits.read::<15, u64>()?;
                read_bits += nr_of_bits + 16;
                while nr_of_bits > 0 {
                    let (packet, rb) = Packet::read_from(bits)?;
                    ps.push(packet);
                    nr_of_bits -= rb;
                }
            }

            Ok((Packet::Op { v, op, ps }, read_bits))
        }
    }

    fn vsum(&self) -> u32 {
        match self {
            Packet::Op { v, op: _, ps } => *v + ps.iter().map(|p| p.vsum()).sum::<u32>(),
            Packet::Lit { v, val: _ } => *v,
        }
    }

    fn value(&self) -> u64 {
        match self {
            Packet::Lit { v: _, val } => *val,
            Packet::Op {
                v: _,
                op: OpType::Sum,
                ps,
            } => ps.iter().map(|p| p.value()).sum(),
            Packet::Op {
                v: _,
                op: OpType::Product,
                ps,
            } => ps.iter().map(|p| p.value()).product(),
            Packet::Op {
                v: _,
                op: OpType::Min,
                ps,
            } => ps.iter().map(|p| p.value()).min().unwrap(),
            Packet::Op {
                v: _,
                op: OpType::Max,
                ps,
            } => ps.iter().map(|p| p.value()).max().unwrap(),
            Packet::Op {
                v: _,
                op: OpType::GreaterThan,
                ps,
            } => {
                if ps[0].value() > ps[1].value() {
                    1
                } else {
                    0
                }
            }
            Packet::Op {
                v: _,
                op: OpType::LessThan,
                ps,
            } => {
                if ps[0].value() < ps[1].value() {
                    1
                } else {
                    0
                }
            }
            Packet::Op {
                v: _,
                op: OpType::EqualTo,
                ps,
            } => {
                if ps[0].value() == ps[1].value() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn calculate_part1(packet: &Packet) -> u32 {
    packet.vsum()
}

fn calculate_part2(packet: &Packet) -> u64 {
    packet.value()
}

day_main!(Packet);

day_test!( 16 => 934, 912901337844 );

#[cfg(test)]
mod part1 {
    use crate::Packet;
    use nom_parse_trait::ParseFrom;

    fn test_packet(input: &str, exp: u32) {
        let (_, p): (_, Packet) = ParseFrom::<_, ()>::parse(input).unwrap();
        assert_eq!(exp, p.vsum(), "Expected {:?} for packet {:?}", exp, p,);
    }

    #[test]
    fn example1() {
        test_packet("8A004A801A8002F478", 16)
    }
    #[test]
    fn example2() {
        test_packet("620080001611562C8802118E34", 12)
    }
    #[test]
    fn example3() {
        test_packet("C0015000016115A2E0802F182340", 23)
    }
    #[test]
    fn example4() {
        test_packet("A0016C880162017C3686B18A3D4780", 31)
    }
}

#[cfg(test)]
mod part2 {
    use crate::Packet;
    use nom_parse_trait::ParseFrom;

    fn test_packet(input: &str, exp: u64) {
        let (_, p): (_, Packet) = ParseFrom::<_, ()>::parse(input).unwrap();
        assert_eq!(exp, p.value(), "Expected {:?} for packet {:?}", exp, p,);
    }

    #[test]
    fn example1() {
        test_packet("C200B40A82", 3)
    }
    #[test]
    fn example2() {
        test_packet("04005AC33890", 54)
    }
    #[test]
    fn example3() {
        test_packet("880086C3E88112", 7)
    }
    #[test]
    fn example4() {
        test_packet("CE00C43D881120", 9)
    }
    #[test]
    fn example5() {
        test_packet("D8005AC2A8F0", 1)
    }
    #[test]
    fn example6() {
        test_packet("F600BC2D8F", 0)
    }
    #[test]
    fn example7() {
        test_packet("9C005AC2F8F0", 0)
    }
    #[test]
    fn example8() {
        test_packet("9C0141080250320F1802104A08", 1)
    }
}
