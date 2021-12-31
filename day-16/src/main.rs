use std::num::ParseIntError;

use bitvec::{field::BitField, order::Msb0, prelude::BitVec, slice::BitSlice, view::BitView};

fn main() {
    let input = include_str!("../input.txt").trim_end();
    println!("part1 = {}", part1(input));
    println!("part2 = {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let digits = parse_hex(input).unwrap();
    let bits = digits.as_slice().view_bits::<Msb0>();
    // dbg!(bits);

    let (version, _bits_read) = parse_packet(bits);
    version
}

fn part2(input: &str) -> usize {
    // TODO: dedup
    let digits = parse_hex(input).unwrap();
    let bits = digits.as_slice().view_bits::<Msb0>();
    let (value, _bits_read) = parse_packet2(bits);
    value
}

fn parse_hex(string: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..string.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&string[i..i + 2], 16))
        .collect()
}

/// Return value = (recursive version number sum, bits read)
fn parse_packet(bits: &BitSlice<Msb0, u8>) -> (u64, usize) {
    let mut version: u64 = bits[0..3].load_be();
    let type_id: u8 = bits[3..6].load_be();
    let mut offset = 6;

    if type_id == 4 {
        let (_value, bits_read) = read_literal(&bits[6..]);
        // dbg!(value);
        (version, bits_read + offset)
    } else {
        let length_type_id = bits[6];
        offset += 1;

        if length_type_id {
            // next 11 bits are number of sub-packets
            let number_of_subpackets: usize = bits[7..18].load_be();
            offset += 11;
            let mut bits_read = 0;

            for _ in 0..number_of_subpackets {
                let (v, br) = parse_packet(&bits[bits_read + offset..]);
                bits_read += br;
                version += v;
            }

            (version, bits_read + offset)
        } else {
            // next 15 bits are length of sub-packets (in bits)
            let length_of_subpackets: usize = bits[7..22].load_be();
            offset += 15;
            let mut bits_read = 0;

            while bits_read < length_of_subpackets {
                let (v, br) = parse_packet(&bits[bits_read + offset..]);
                bits_read += br;
                version += v;
            }

            (version, bits_read + offset)
        }
    }
}

/// Return value = (calculated value, bits read)
fn parse_packet2(bits: &BitSlice<Msb0, u8>) -> (usize, usize) {
    let _version: usize = bits[0..3].load_be();
    let type_id: u8 = bits[3..6].load_be();
    let offset = 6;

    if type_id == 4 {
        let (value, bits_read) = read_literal(&bits[6..]);
        (value, bits_read + offset)
    } else {
        let (subpackets, bits_read) = parse_subpackets(&bits[6..]);
        let value = match type_id {
            0 => subpackets.into_iter().sum(),
            1 => subpackets.into_iter().product(),
            2 => subpackets.into_iter().min().unwrap(),
            3 => subpackets.into_iter().max().unwrap(),
            5 => {
                if subpackets[0] > subpackets[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if subpackets[0] < subpackets[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if subpackets[0] == subpackets[1] {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!("bad type_id: {}", type_id),
        };

        (value, bits_read + offset)
    }
}

fn parse_subpackets(bits: &BitSlice<Msb0, u8>) -> (Vec<usize>, usize) {
    let length_type_id = bits[0];
    let mut offset = 1;
    let mut values = Vec::with_capacity(2);

    if length_type_id {
        // next 11 bits are number of sub-packets
        let number_of_subpackets: usize = bits[1..12].load_be();
        offset += 11;
        let mut bits_read = 0;

        for _ in 0..number_of_subpackets {
            let (v, br) = parse_packet2(&bits[bits_read + offset..]);
            bits_read += br;
            values.push(v);
        }

        (values, bits_read + offset) // TODO
    } else {
        // next 15 bits are length of sub-packets (in bits)
        let length_of_subpackets: usize = bits[1..16].load_be();
        offset += 15;
        let mut bits_read = 0;

        while bits_read < length_of_subpackets {
            let (v, br) = parse_packet2(&bits[bits_read + offset..]);
            bits_read += br;
            values.push(v);
        }

        (values, bits_read + offset) // TODO
    }
}

/// Return value = (value, bits read)
fn read_literal(bits: &BitSlice<Msb0, u8>) -> (usize, usize) {
    let mut start = 0;
    let mut store: BitVec = Default::default();

    while bits[start] {
        store.extend_from_bitslice(&bits[start + 1..start + 5]);
        start += 5;
    }

    store.extend_from_bitslice(&bits[start + 1..start + 5]);

    store.reverse(); // TODO: hacky
    (store.load_be(), start + 5)
}

mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn part1_tests() {
        assert_eq!(16, part1("8A004A801A8002F478"));
        assert_eq!(12, part1("620080001611562C8802118E34"));
        assert_eq!(23, part1("C0015000016115A2E0802F182340"));
        assert_eq!(31, part1("A0016C880162017C3686B18A3D4780"));
    }

    #[test]
    fn part2_tests() {
        assert_eq!(3, part2("C200B40A82"));
        assert_eq!(54, part2("04005AC33890"));
        assert_eq!(7, part2("880086C3E88112"));
        assert_eq!(9, part2("CE00C43D881120"));
        assert_eq!(1, part2("D8005AC2A8F0"));
        assert_eq!(0, part2("F600BC2D8F"));
        assert_eq!(0, part2("9C005AC2F8F0"));
        assert_eq!(1, part2("9C0141080250320F1802104A08"));
    }
}
