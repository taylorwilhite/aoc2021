use hex;
#[derive(PartialEq, Clone, Copy, Debug)]
enum PacketType {
    Literal,
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Less,
    Equal,
}
#[derive(Clone)]
struct Packet {
    packet_type: PacketType,
    version: usize,
    sub_packets: Option<Box<[Packet]>>,
    literal: Option<usize>,
}

pub fn main() {
    let string = include_str!("../input.txt");
    let bits = get_bits(string);
    let part1 = part_one(&bits);
    println!("Part 1: {}", part1);
    let part2 = part_two(&bits);
    println!("Part 2: {}", part2);
}

fn get_bits(string: &str) -> String {
    let mut result = String::new();
    let bytes = hex::decode(string).unwrap();
    for byte in bytes {
        result.push_str(&format!("{:08b}", byte));
    }
    return result;
}

fn part_one(bits: &str) -> usize {
    let (packet, _) = parse_packet(bits);
    let sum = get_packet_sum(packet);
    return sum;
}

fn part_two(bits: &str) -> usize {
    let (packet, _) = parse_packet(bits);
    let result = get_packet_result(packet);
    return result;
}

fn get_packet_sum(packet: Packet) -> usize {
    let sum = packet.version;
    let additional = match packet.sub_packets {
        None => 0,
        Some(b) => {
            let packets = b.into_vec();
            packets
                .iter()
                .fold(0, |acc, p| acc + get_packet_sum(p.clone()))
        }
    };
    sum + additional
}

fn get_packet_result(packet: Packet) -> usize {
    use PacketType::*;
    let result = match packet.packet_type {
        Literal => packet.literal.unwrap(),
        Sum => packet
            .sub_packets
            .unwrap()
            .into_iter()
            .fold(0, |acc, x| acc + get_packet_result(x.clone())),
        Product => packet
            .sub_packets
            .unwrap()
            .into_iter()
            .fold(1, |acc, x| acc * get_packet_result(x.clone())),
        Minimum => packet
            .sub_packets
            .unwrap()
            .into_iter()
            .fold(usize::MAX, |acc, x| {
                let packet_result = get_packet_result(x.clone());
                if acc > packet_result {
                    packet_result
                } else {
                    acc
                }
            }),
        Maximum => packet.sub_packets.unwrap().into_iter().fold(0, |acc, x| {
            let packet_result = get_packet_result(x.clone());
            if acc < packet_result {
                packet_result
            } else {
                acc
            }
        }),
        Greater => {
            let comparison = packet.sub_packets.unwrap().into_vec();
            let value1 = get_packet_result(comparison[0].clone());
            let value2 = get_packet_result(comparison[1].clone());
            if value1 > value2 {
                1
            } else {
                0
            }
        }
        Less => {
            let comparison = packet.sub_packets.unwrap().into_vec();
            let value1 = get_packet_result(comparison[0].clone());
            let value2 = get_packet_result(comparison[1].clone());
            if value1 < value2 {
                1
            } else {
                0
            }
        }
        Equal => {
            let comparison = packet.sub_packets.unwrap().into_vec();
            let value1 = get_packet_result(comparison[0].clone());
            let value2 = get_packet_result(comparison[1].clone());
            if value1 == value2 {
                1
            } else {
                0
            }
        }
    };
    return result;
}

fn parse_packet(bits: &str) -> (Packet, usize) {
    let version = usize::from_str_radix(bits.get(0..3).unwrap(), 2).unwrap();

    let packet_type = match usize::from_str_radix(bits.get(3..6).unwrap(), 2).unwrap() {
        4 => PacketType::Literal,
        0 => PacketType::Sum,
        1 => PacketType::Product,
        2 => PacketType::Minimum,
        3 => PacketType::Maximum,
        5 => PacketType::Greater,
        6 => PacketType::Less,
        7 => PacketType::Equal,
        _ => unreachable!(),
    };

    let rest = bits.get(6..).unwrap();

    if packet_type == PacketType::Literal {
        let (value, used) = parse_literal(rest);
        let literal_packet = Packet {
            packet_type: packet_type,
            version: version,
            sub_packets: None,
            literal: Some(value),
        };
        return (literal_packet, used + 6);
    } else {
        let length_id = rest.get(0..1).unwrap();
        if length_id == "0" {
            let bitcount = usize::from_str_radix(rest.get(1..16).unwrap(), 2).unwrap();
            let mut bits_used = 16;
            let mut sub_packets: Vec<Packet> = Vec::new();
            while bits_used < bitcount + 16 {
                let (packet, used) = parse_packet(rest.get(bits_used..).unwrap());
                sub_packets.push(packet);
                bits_used += used;
            }
            let packet = Packet {
                version: version,
                packet_type: packet_type,
                sub_packets: Some(sub_packets.into_boxed_slice()),
                literal: None,
            };
            return (packet, bits_used + 6);
        } else {
            let packet_number = usize::from_str_radix(rest.get(1..12).unwrap(), 2).unwrap();
            let mut bits_used = 12;
            let mut sub_packets: Vec<Packet> = Vec::new();
            for _ in 0..packet_number {
                let (packet, used) = parse_packet(rest.get(bits_used..).unwrap());
                sub_packets.push(packet);
                bits_used += used;
            }
            let packet = Packet {
                version: version,
                packet_type: packet_type,
                sub_packets: Some(sub_packets.into_boxed_slice()),
                literal: None,
            };
            return (packet, bits_used + 6);
        }
    }
}

fn parse_literal(string: &str) -> (usize, usize) {
    let mut result = String::new();
    let mut break_point = 0;
    for num in (0..string.len()).step_by(5) {
        let slice = string.get(num + 1..num + 5).unwrap();
        result.push_str(slice);
        if string.chars().nth(num).unwrap() == '0' {
            break_point = num + 5;
            break;
        }
    }
    (usize::from_str_radix(&result, 2).unwrap(), break_point)
}

#[test]
fn string_rep() {
    let bits = get_bits("D2FE28");
    assert_eq!(bits, "110100101111111000101000");
}
#[test]
fn example_1() {
    let bits = get_bits("8A004A801A8002F478");
    let result = part_one(&bits);

    assert_eq!(result, 16);
}
#[test]
fn example_2() {
    let bits = get_bits("620080001611562C8802118E34");
    let result = part_one(&bits);

    assert_eq!(result, 12);
}
#[test]
fn example_3() {
    let bits = get_bits("C0015000016115A2E0802F182340");
    let result = part_one(&bits);

    assert_eq!(result, 23);
}
#[test]
fn example_4() {
    let bits = get_bits("A0016C880162017C3686B18A3D4780");
    let result = part_one(&bits);

    assert_eq!(result, 31);
}
#[test]
fn p2_example_1() {
    let bits = get_bits("C200B40A82");
    let result = part_two(&bits);
    assert_eq!(result, 3);
}
#[test]
fn p2_example_2() {
    let bits = get_bits("04005AC33890");
    let result = part_two(&bits);
    assert_eq!(result, 54);
}
#[test]
fn p2_example_3() {
    let bits = get_bits("880086C3E88112");
    let result = part_two(&bits);
    assert_eq!(result, 7);
}
#[test]
fn p2_example_4() {
    let bits = get_bits("CE00C43D881120");
    let result = part_two(&bits);
    assert_eq!(result, 9);
}
#[test]
fn p2_example_5() {
    let bits = get_bits("D8005AC2A8F0");
    let result = part_two(&bits);
    assert_eq!(result, 1);
}
#[test]
fn p2_example_6() {
    let bits = get_bits("F600BC2D8F");
    let result = part_two(&bits);
    assert_eq!(result, 0);
}
#[test]
fn p2_example_7() {
    let bits = get_bits("9C005AC2F8F0");
    let result = part_two(&bits);
    assert_eq!(result, 0);
}
#[test]
fn p2_example_8() {
    let bits = get_bits("9C0141080250320F1802104A08");
    let result = part_two(&bits);
    assert_eq!(result, 1);
}
