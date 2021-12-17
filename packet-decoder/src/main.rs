use std::io::BufRead;

use bitvec::prelude::*;

fn main() {
    let mut packet_data = dbg!(load_packet(std::io::stdin().lock().lines().map(Result::unwrap).flat_map(|l| l.chars().collect::<Vec<_>>())));
    let packet = parse_packet(&mut packet_data);
    println!("{:?}", packet);
    dbg!(packet.version_sum());
    dbg!(packet.value());
}

fn load_packet<I>(iter: I) -> BitVec where I: Iterator<Item = char> {
    let mut packet_data: BitVec = BitVec::new();
    for c in iter {
        let i = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'A' => 10,
            'B' => 11,
            'C' => 12,
            'D' => 13,
            'E' => 14,
            'F' => 15,
            _ => unreachable!(),
        };
        let bits = BITS[i];
        for bit in bits {
            packet_data.push(bit);
        }
    }
    packet_data
}

fn parse_packet(data: &mut BitVec) -> BITSPacket {
    let version = rbits_u8(data, 3);
    println!("{}", version);
    let packet_type = rbits_u8(data, 3);
    let data = match packet_type {
        4 => parse_literal_packet(data),
        // _ => unimplemented!(),
        ty => parse_operator_packet(data, ty),
    };
    return BITSPacket {
        version,
        data,
    }
}

fn parse_literal_packet(data: &mut BitVec) -> BITSPacketData {
    let mut v = 0;

    while !data.is_empty() {
        let cont = data.remove(0);
        let a = rbits_u8(data, 4) as u64;
        v <<= 4;
        v |= a;
        if !cont { break; }
    }

    // println!("{:#018b}", v);

    BITSPacketData::Literal(v)
}

fn parse_operator_packet(data: &mut BitVec, ty: u8) -> BITSPacketData {
    use BITSPacketData::*;
    let length_type_id = data.remove(0);
    // println!("{}", length_type_id);

    let mut children = Vec::new();

    if !length_type_id {
        // 15 bits - total length in bits of inner packets
        let children_length = rbits_u16(data, 15) as usize;
        let mut read = 0;
        while read < children_length {
            let initial_len = data.len();
            children.push(parse_packet(data));
            read += initial_len - data.len();
        }
    } else {
        // 11 bits - number of sub packets
        let packet_count = rbits_u16(data, 11);
        for _ in 0..packet_count {
            children.push(parse_packet(data));
        }
    }

    match ty {
        0 => Sum(children),
        1 => Product(children),
        2 => Minimum(children),
        3 => Maximum(children),
        5 => GreaterThan(children),
        6 => LessThan(children),
        7 => EqualTo(children),
        _ => unreachable!(),
    }
}

fn rbits_u8(data: &mut BitVec, count: u8) -> u8 {
    assert!(count <= 8);
    let mut v = 0;
    for i in 0..count {
        if data.remove(0) {
            v |= 1 << (count - 1 - i);
        }
    }
    v
}

fn rbits_u16(data: &mut BitVec, count: u8) -> u16 {
    assert!(count <= 16);
    let mut v = 0;
    for i in 0..count {
        if data.remove(0) {
            v |= 1 << (count - 1 - i);
        }
    }
    v
}

#[derive(Debug)]
struct BITSPacket {
    pub version: u8,
    pub data: BITSPacketData,
}

impl BITSPacket {
    fn version_sum(&self) -> u64 {
        dbg!(self.version);
        self.version as u64 + self.data.version_sum()
    }

    fn value(&self) -> u64 {
        self.data.value()
    }
}

#[derive(Debug)]
enum BITSPacketData {
    Literal(u64),
    Sum(Vec<BITSPacket>),
    Product(Vec<BITSPacket>),
    Minimum(Vec<BITSPacket>),
    Maximum(Vec<BITSPacket>),
    GreaterThan(Vec<BITSPacket>),
    LessThan(Vec<BITSPacket>),
    EqualTo(Vec<BITSPacket>),
}

impl BITSPacketData {
    fn version_sum(&self) -> u64 {
        match self {
            BITSPacketData::Literal(_) => 0,
            BITSPacketData::Sum(children)
            | BITSPacketData::Product(children)
            | BITSPacketData::Minimum(children)
            | BITSPacketData::Maximum(children)
            | BITSPacketData::GreaterThan(children)
            | BITSPacketData::LessThan(children)
            | BITSPacketData::EqualTo(children)
                => children.iter().map(|child| child.version_sum()).sum::<u64>(),
        }
    }

    fn value(&self) -> u64 {
        use BITSPacketData::*;
        match self {
            Literal(value) => *value,
            Sum(children) => children.iter().map(BITSPacket::value).sum(),
            Product(children) => {
                let mut result = 1;
                for child in children {
                    result *= child.value();
                }
                result
            }
            Minimum(children) => children.iter().map(BITSPacket::value).min().unwrap(),
            Maximum(children) => children.iter().map(BITSPacket::value).max().unwrap(),
            GreaterThan(children) => {
                assert!(children.len() == 2);
                let value_a = children.first().unwrap().value();
                let value_b = children.last().unwrap().value();
                if value_a > value_b { 1 } else { 0 }
            }
            LessThan(children) => {
                assert!(children.len() == 2);
                let value_a = children.first().unwrap().value();
                let value_b = children.last().unwrap().value();
                if value_a < value_b { 1 } else { 0 }
            }
            EqualTo(children) => {
                assert!(children.len() == 2);
                let value_a = children.first().unwrap().value();
                let value_b = children.last().unwrap().value();
                if value_a == value_b { 1 } else { 0 }
            }
        }
    }
}

const BITS: [[bool; 4]; 16] = [
    [false, false, false, false  ],
    [false, false, false, true,  ],
    [false, false, true,  false, ],
    [false, false, true,  true,  ],
    [false, true,  false, false, ],
    [false, true,  false, true,  ],
    [false, true,  true,  false, ],
    [false, true,  true,  true,  ],
    [true,  false, false, false  ],
    [true,  false, false, true,  ],
    [true,  false, true,  false, ],
    [true,  false, true,  true,  ],
    [true,  true,  false, false, ],
    [true,  true,  false, true,  ],
    [true,  true,  true,  false, ],
    [true,  true,  true,  true,  ],
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_literal_packet() {
        let packet = "D2FE28";
        let mut data = load_packet(packet.chars());
        let packet = parse_packet(&mut data);
        assert_eq!(packet.version, 6);
    }

    #[test]
    fn test_operator_length_type_id_1() {
        let packet = "EE00D40C823060";
        let mut data = load_packet(packet.chars());
        let packet = parse_packet(&mut data);
        println!("{:?}", packet);
        assert_eq!(packet.version, 7);
        if let BITSPacketData::Maximum(children) = packet.data {
            println!("id=1 {:?}", children);
        } else {
            panic!("Did not parse as operator packet");
        }
        assert_eq!(data.len(), 5);
    }

    #[test]
    fn test_operator_length_type_id_0() {
        let packet = "38006F45291200";
        let mut data = load_packet(packet.chars());
        let packet = parse_packet(&mut data);
        println!("{:?}", packet);
        assert_eq!(packet.version, 1);
        if let BITSPacketData::LessThan(children) = packet.data {
            println!("id=0 {:?}", children);
        } else {
            panic!("Did not parse as less than packet");
        }
        assert_eq!(data.len(), 7);
    }

    macro_rules! assert_packet_sum {
        ($packet:expr, $type:ident, $sum:expr, $root_child_len:expr) => {
            let mut data = load_packet($packet.chars());
            let packet = parse_packet(&mut data);
            println!("{:?}", packet);
            if let $crate::BITSPacketData::$type(children) = &packet.data {
                assert_eq!(children.len(), $root_child_len);
            } else {
                panic!("Not an operator packet!");
            }
            assert_eq!(packet.version_sum(), $sum);
        };
    }

    #[test]
    fn test_packet_sum() {
        assert_packet_sum!("8A004A801A8002F478", Minimum, 16, 1);
        assert_packet_sum!("620080001611562C8802118E34", Sum, 12, 2);
        assert_packet_sum!("C0015000016115A2E0802F182340", Sum, 23, 2);
        assert_packet_sum!("A0016C880162017C3686B18A3D4780", Sum, 31, 1);
    }
}
