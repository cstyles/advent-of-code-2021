use std::num::ParseIntError;

use bitvec::{field::BitField, order::Msb0, prelude::BitVec, slice::BitSlice, view::BitView};

fn main() {
    let input = include_str!("../input.txt").trim_end();
    println!("part1 = {}", part1(input));
}

fn part1(input: &str) -> u64 {
    let digits = parse_hex(input).unwrap();
    let bits = digits.as_slice().view_bits::<Msb0>();
    // dbg!(bits);

    let (version, _bits_read) = parse_packet(bits);
    version
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
}
