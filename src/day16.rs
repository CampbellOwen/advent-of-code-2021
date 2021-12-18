use std::{fs::read_to_string, io::BufRead};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Header {
    version: u8,
    id: u8,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operator {
    Sum,
    Product,
    Min,
    Max,
    GT,
    LT,
    EQ,
}

#[derive(Debug, PartialEq, Clone)]
enum PacketType {
    Literal(i64),
    Operator(Operator, Vec<Packet>),
}
#[derive(Debug, PartialEq, Clone)]
struct Packet {
    header: Header,
    body: PacketType,
}

fn parse_number<I>(stream: &mut I, num_bits: usize) -> i64
where
    I: Iterator<Item = char>,
{
    let mut num_string = String::new();

    for _ in 0..num_bits {
        num_string.push(stream.next().expect("Not enough bits in stream"));
    }

    i64::from_str_radix(&num_string, 2).expect("input stream malformed")
}

fn parse_literal<I>(stream: &mut I) -> (usize, i64)
where
    I: Iterator<Item = char>,
{
    let mut literal = 0;
    let mut num_bits = 0;

    let mut keep_parsing = true;
    while keep_parsing {
        keep_parsing = if stream.next().expect("We should keep parsing????") == '1' {
            true
        } else {
            false
        };
        num_bits += 1;

        // Parse group ( 4 bits )
        literal = (literal << 4) | parse_number(stream, 4);

        num_bits += 4;
    }

    (num_bits, literal)
}

fn parse_header<I>(stream: &mut I) -> (usize, Header)
where
    I: Iterator<Item = char>,
{
    let version = parse_number(stream, 3) as u8;
    let packet_type = parse_number(stream, 3) as u8;

    (
        6,
        Header {
            version,
            id: packet_type,
        },
    )
}

enum LengthType {
    NumBits(usize),
    NumPackets(usize),
}

fn parse_operator<I>(stream: &mut I, type_id: u8) -> (usize, PacketType)
where
    I: Iterator<Item = char>,
{
    let mut num_bits = 0;

    let length_type = stream.next().expect("Packet body malformed");
    num_bits += 1;

    let length = match length_type {
        '0' => {
            num_bits += 15;
            LengthType::NumBits(parse_number(stream, 15) as usize)
        }
        '1' => {
            num_bits += 11;
            LengthType::NumPackets(parse_number(stream, 11) as usize)
        }
        _ => unreachable!(),
    };

    let mut sub_packets = Vec::new();

    match length {
        LengthType::NumBits(bits) => {
            let mut bits = bits;
            while bits > 0 {
                let (parsed, sub) = parse_packet(stream);
                sub_packets.push(sub);
                num_bits += parsed;
                bits -= parsed;
            }
        }
        LengthType::NumPackets(packets) => {
            for _ in 0..packets {
                let (parsed, sub) = parse_packet(stream);
                sub_packets.push(sub);
                num_bits += parsed;
            }
        }
    }

    let operator = match type_id {
        0 => Operator::Sum,
        1 => Operator::Product,
        2 => Operator::Min,
        3 => Operator::Max,
        5 => Operator::GT,
        6 => Operator::LT,
        7 => Operator::EQ,
        _ => unreachable!(),
    };

    (num_bits, PacketType::Operator(operator, sub_packets))
}

fn parse_packet<I>(stream: &mut I) -> (usize, Packet)
where
    I: Iterator<Item = char>,
{
    let (header_bits, header) = parse_header(stream);
    let (body_bits, body) = match header.id {
        4 => {
            let (num_bits, literal) = parse_literal(stream);
            (num_bits, PacketType::Literal(literal))
        }
        _ => parse_operator(stream, header.id),
    };

    (header_bits + body_bits, Packet { header, body })
}

fn hex_to_binary_string(s: &str) -> String {
    let binary_strings = s
        .chars()
        .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
        .map(|num| format!("{:04b}", num))
        .collect::<Vec<String>>();
    binary_strings.iter().map(|s| s.chars()).flatten().collect()
}

fn sum_versions(packet: &Packet) -> usize {
    match &packet.body {
        PacketType::Literal(_) => packet.header.version as usize,
        PacketType::Operator(_, sub_packets) => {
            packet.header.version as usize + sub_packets.iter().map(sum_versions).sum::<usize>()
        }
    }
}

fn eval_packet(p: &Packet) -> i64 {
    match &p.body {
        PacketType::Literal(i) => *i,
        PacketType::Operator(op, sub_packets) => {
            let mut it = sub_packets.iter();
            let first = it.next().expect("At least 1 sub packet");
            let first = eval_packet(first);
            it.fold(first, |res, p| match op {
                Operator::Sum => res + eval_packet(p),
                Operator::Product => res * eval_packet(p),
                Operator::Min => res.min(eval_packet(p)),
                Operator::Max => res.max(eval_packet(p)),
                Operator::GT => {
                    if res > eval_packet(p) {
                        1
                    } else {
                        0
                    }
                }
                Operator::LT => {
                    if res < eval_packet(p) {
                        1
                    } else {
                        0
                    }
                }
                Operator::EQ => {
                    if res == eval_packet(p) {
                        1
                    } else {
                        0
                    }
                }
            })
        }
    }
}

pub fn part1() {
    let input = read_to_string("input/day16.txt").expect("Input file should exist");
    let binary = hex_to_binary_string(&input);
    let mut stream = binary.chars();

    let (_, packet) = parse_packet(&mut stream);

    let version_sum = sum_versions(&packet);
    println!("Version sum: {}", version_sum);
}
pub fn part2() {
    let input = read_to_string("input/day16.txt").expect("Input file should exist");
    let binary = hex_to_binary_string(&input);
    let mut stream = binary.chars();

    let (_, packet) = parse_packet(&mut stream);

    let result = eval_packet(&packet);

    println!("Packet expresion is {}", result);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn convert_string() {
        let input = "D2FE28";
        let binary = hex_to_binary_string(input);

        assert_eq!(binary, "110100101111111000101000");

        assert_eq!(
            hex_to_binary_string("38006F45291200"),
            "00111000000000000110111101000101001010010001001000000000"
        );
    }

    #[test]
    fn parse_header_test() {
        let input = "D2FE28";
        let binary = hex_to_binary_string(input);
        let mut binary = binary.chars();

        let (header_bits, header) = parse_header(&mut binary);

        assert_eq!(header, Header { version: 6, id: 4 });
        assert_eq!(header_bits, 6);
    }

    #[test]
    fn parse_literal_test() {
        let input = "D2FE28";
        let binary = hex_to_binary_string(input);
        let mut binary = binary.chars();

        let _ = parse_header(&mut binary);
        let (bits, number) = parse_literal(&mut binary);
        assert_eq!(number, 2021);
        assert_eq!(bits, 15);
    }

    #[test]
    fn parse_operator_test() {
        let input = "38006F45291200";
        let binary = hex_to_binary_string(input);
        let mut binary = binary.chars();

        let (bits, header) = parse_header(&mut binary);
        assert_eq!(bits, 6);
        assert_eq!(header.id, 6);

        let (bits, operator) = parse_operator(&mut binary, header.id);
        assert_eq!(bits, 43);

        if let PacketType::Operator(Operator::LT, packets) = operator {
            assert_eq!(packets.len(), 2);
            assert_eq!(packets[0].body, PacketType::Literal(10));
            assert_eq!(packets[1].body, PacketType::Literal(20));
        } else {
            assert!(false);
        }

        let input = "EE00D40C823060";
        let binary = hex_to_binary_string(input);
        let mut binary = binary.chars();

        let (bits, header) = parse_header(&mut binary);
        assert_eq!(bits, 6);
        assert_eq!(header.id, 3);

        let (bits, operator) = parse_operator(&mut binary, header.id);
        assert_eq!(bits, 45);

        if let PacketType::Operator(Operator::Max, packets) = operator {
            assert_eq!(packets.len(), 3);
            assert_eq!(packets[0].body, PacketType::Literal(1));
            assert_eq!(packets[1].body, PacketType::Literal(2));
            assert_eq!(packets[2].body, PacketType::Literal(3));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn part1_examples() {
        let input = "8A004A801A8002F478";
        let binary = hex_to_binary_string(input);
        let mut binary = binary.chars();
        let (_, packet) = parse_packet(&mut binary);
        let version_sum = sum_versions(&packet);

        assert_eq!(version_sum, 16);

        let input = "620080001611562C8802118E34";
        let binary = hex_to_binary_string(input);
        let mut binary = binary.chars();
        let (_, packet) = parse_packet(&mut binary);
        let version_sum = sum_versions(&packet);

        assert_eq!(version_sum, 12);

        let input = "C0015000016115A2E0802F182340";
        let binary = hex_to_binary_string(input);
        let mut binary = binary.chars();
        let (_, packet) = parse_packet(&mut binary);
        let version_sum = sum_versions(&packet);

        assert_eq!(version_sum, 23);

        let input = "A0016C880162017C3686B18A3D4780";
        let binary = hex_to_binary_string(input);
        let mut binary = binary.chars();
        let (_, packet) = parse_packet(&mut binary);
        let version_sum = sum_versions(&packet);

        assert_eq!(version_sum, 31);
    }

    #[test]
    fn part2_examples() {
        let input = "C200B40A82";
        let binary = hex_to_binary_string(input);
        let mut binary = binary.chars();
        let (_, packet) = parse_packet(&mut binary);
        let result = eval_packet(&packet);

        assert_eq!(result, 3);

        let input = "04005AC33890";
        let binary = hex_to_binary_string(input);
        let mut binary = binary.chars();
        let (_, packet) = parse_packet(&mut binary);
        let result = eval_packet(&packet);

        assert_eq!(result, 54);

        let input = "880086C3E88112";
        let binary = hex_to_binary_string(input);
        let mut binary = binary.chars();
        let (_, packet) = parse_packet(&mut binary);
        let result = eval_packet(&packet);

        assert_eq!(result, 7);

        let input = "9C0141080250320F1802104A08";
        let binary = hex_to_binary_string(input);
        let mut binary = binary.chars();
        let (_, packet) = parse_packet(&mut binary);
        let result = eval_packet(&packet);

        assert_eq!(result, 1);
    }
}
