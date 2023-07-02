use std::fs::File;
use std::io::Read;

use nom::bytes::complete::take_while_m_n;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::IResult;
use nom::{error::ErrorKind, Err};

#[derive(Debug)]
pub struct Header {
    pub version: u8,
    pub type_id: u8,
}

#[derive(Debug)]
pub struct Packet {
    pub header: Header,
    pub content: PacketContent,
}

impl Packet {
    fn size(&self) -> usize {
        6 + self.content.size()
    }

    pub fn sum_of_packet_versions(&self) -> usize {
        self.header.version as usize + self.content.sum_of_packet_versions()
    }

    pub fn value(&self) -> u64 {
        self.content.value(self.header.type_id)
    }
}

#[derive(Debug)]
pub enum PacketContent {
    Literal(Vec<u8>),
    Operator(Operator),
}

impl PacketContent {
    fn size(&self) -> usize {
        match self {
            PacketContent::Literal(numbers) => 5 * numbers.len(),
            PacketContent::Operator(op) => 1 + op.size(),
        }
    }

    fn sum_of_packet_versions(&self) -> usize {
        match self {
            PacketContent::Literal(_) => 0,
            PacketContent::Operator(op) => op.sum_of_packet_versions(),
        }
    }

    fn value(&self, type_id: u8) -> u64 {
        match self {
            PacketContent::Literal(numbers) => literal_value(numbers),
            PacketContent::Operator(op) => op.value(type_id),
        }
    }
}

fn literal_value(nibbles: &[u8]) -> u64 {
    let mut result = 0;
    for &v in nibbles {
        result *= 16;
        result += v as u64;
    }
    result
}

#[derive(Debug)]
pub enum Operator {
    NumBits(u16, Vec<Packet>),
    NumPackets(u16, Vec<Packet>),
}

impl Operator {
    fn size(&self) -> usize {
        match self {
            Operator::NumBits(num_bits, _) => 15 + *num_bits as usize,
            Operator::NumPackets(_, packets) => {
                11 + packets.iter().map(|p| p.size()).sum::<usize>()
            }
        }
    }

    fn sum_of_packet_versions(&self) -> usize {
        match self {
            Operator::NumBits(_, packets) => packets
                .iter()
                .map(|packet| packet.sum_of_packet_versions())
                .sum(),
            Operator::NumPackets(_, packets) => packets
                .iter()
                .map(|packet| packet.sum_of_packet_versions())
                .sum(),
        }
    }

    fn value(&self, op: u8) -> u64 {
        let packets = match self {
            Operator::NumBits(_, packets) => packets,
            Operator::NumPackets(_, packets) => packets,
        };

        let values = packets.iter().map(|p| p.value());
        match op {
            0 => values.sum(),          // sum
            1 => values.product(),      // product
            2 => values.min().unwrap(), // minimum
            3 => values.max().unwrap(), // maximum
            5 => {
                let v: Vec<_> = values.take(2).collect();
                u64::from(v[0] > v[1])
            } // greater than
            6 => {
                let v: Vec<_> = values.take(2).collect();
                u64::from(v[0] < v[1])
            } // less than
            7 => {
                let v: Vec<_> = values.take(2).collect();
                u64::from(v[0] == v[1])
            } // equal to
            _ => unreachable!("op = {}", op),
        }
    }
}

pub fn packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let (input, header) = header(input)?;
    let (input, content) = match header.type_id {
        4 => literal(input),
        _ => operator(input),
    }?;
    let packet = Packet { header, content };
    Ok((input, packet))
}

fn header(input: (&[u8], usize)) -> IResult<(&[u8], usize), Header> {
    let (input, version): ((&[u8], usize), u8) = nom::bits::complete::take(3u8)(input)?;
    let (input, type_id): ((&[u8], usize), u8) = nom::bits::complete::take(3u8)(input)?;
    let header = Header { version, type_id };
    // dbg!(&header);
    Ok((input, header))
}

fn literal(input: (&[u8], usize)) -> IResult<(&[u8], usize), PacketContent> {
    let mut result = Vec::new();
    let mut i = input;
    loop {
        let (next_input, (cont, value)) = literal_number(i)?;
        i = next_input;
        // dbg!(cont, value);
        result.push(value);
        if !cont {
            break;
        }
    }
    Ok((i, PacketContent::Literal(result)))
}

fn literal_number(input: (&[u8], usize)) -> IResult<(&[u8], usize), (bool, u8)> {
    let (input, cont): ((&[u8], usize), u8) = nom::bits::complete::take(1u8)(input)?;
    let (input, value): ((&[u8], usize), u8) = nom::bits::complete::take(4u8)(input)?;

    Ok((input, (cont != 0, value)))
}

fn operator(input: (&[u8], usize)) -> IResult<(&[u8], usize), PacketContent> {
    let (input, length_type_id): ((&[u8], usize), u8) = nom::bits::complete::take(1u8)(input)?;
    // dbg!(length_type_id);
    let (input, op) = if length_type_id == 0 {
        num_bits(input)
    } else {
        num_packets(input)
    }?;
    Ok((input, PacketContent::Operator(op)))
}

fn num_bits(input: (&[u8], usize)) -> IResult<(&[u8], usize), Operator> {
    let (input, num_bits): ((&[u8], usize), u16) = nom::bits::complete::take(15u8)(input)?;
    let mut result = Vec::new();
    let mut i = input;
    let mut packet_bits = 0;
    // dbg!(num_bits);
    loop {
        let (next_input, packet) = packet(i)?;
        i = next_input;
        packet_bits += packet.size() as u16;
        // dbg!(packet.size());
        result.push(packet);
        if packet_bits >= num_bits {
            if packet_bits > num_bits {
                return Err(Err::Error(nom::error::Error::new(i, ErrorKind::Count)));
            }
            break;
        }
    }
    Ok((i, Operator::NumBits(num_bits, result)))
}

fn num_packets(input: (&[u8], usize)) -> IResult<(&[u8], usize), Operator> {
    let (input, num_packets): ((&[u8], usize), u16) = nom::bits::complete::take(11u8)(input)?;
    let mut result = Vec::new();
    let mut i = input;
    // dbg!(num_packets);
    for _ in 0..num_packets {
        let (next_input, packet) = packet(i)?;
        i = next_input;
        result.push(packet);
    }
    Ok((i, Operator::NumPackets(num_packets, result)))
}

type InputData = Vec<u8>;

pub fn input_data(input: &str) -> IResult<&str, InputData> {
    many1(hex_primary)(input)
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

pub fn read_file(name: &str) -> anyhow::Result<String> {
    let mut f = File::open(name)?;
    let mut result = String::new();
    f.read_to_string(&mut result)?;
    Ok(result)
}
