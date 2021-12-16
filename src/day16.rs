use std::cmp::{min, max};
use std::ops::{Add, Mul};


enum PackData{
    Literal(i64)
    ,Operator(Vec<Packet>)
}

struct Packet{
    version: u8
    ,type_id: u8
    ,data: PackData
    ,bit_length: i64
}

// An iterator for bits from a hex string
struct BitStream<'a>{
    input: &'a [u8]
    ,c_pos: usize
    ,c_last: u8
    ,b_pos: usize
    ,len: usize
}

// Takes an ascii character from the set 0123456789ABCDEF, returns a number between 0 and 15
// (returns an arbitrary value for other characters, which I don't care about :P)
fn hexit_parse(c: u8) -> u8{
    let c = c-48;
    if c <= 9 {c}
    else {c - ((65-48)-10)}
}

// Bit stream implementation, takes an ascii string of hex digits, is an iterator over bits from that string from left to right
impl BitStream<'_>{
    fn new(input: &[u8]) -> BitStream{
        BitStream{
            input:input
            ,c_pos:0
            ,c_last:hexit_parse(input[0])
            ,b_pos:0
            ,len:input.len()
        }
    }
}

// The iterator logic for the bistream
impl Iterator for BitStream<'_>{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item>{
        match self.b_pos{
            4 => {
                self.b_pos = 1;
                self.c_pos += 1;
                if self.c_pos >= self.len {
                    return None
                }
                self.c_last = hexit_parse(self.input[self.c_pos]);
            }
            _ => {
                self.b_pos += 1;
            }
        }
        Some((self.c_last >> (3-(self.b_pos-1))) & 1)
    }
}

// takes the next n bits from a bitstream and parses them as a number
fn take_bits(bits: &mut BitStream, n: i64) -> i64{
    let mut result: i64 = 0;
    for _ in 0..n{
        result = (result << 1) | (bits.next().unwrap()) as i64;
    }
    result
}

// parses the literal section of a data packet, return the data and the bit length
fn parse_literal(bits: &mut BitStream) -> (PackData, i64){
    let mut val: i64 = 0;
    let mut n_bits = 0;
    // encoding is [a][bcde] where [a] is the not-stop bit, and [bcde] are the next four bits of the number
    while {
        let not_last = bits.next().unwrap() == 1;
        val = val << 4 | take_bits(bits, 4);
        n_bits += 5;
        not_last
    }{}
    (PackData::Literal(val), n_bits)
}

// parse an operator packet payload, return the data and the bit length
fn parse_operator(bits: &mut BitStream) -> (PackData, i64){
    let mut subpacks: Vec<Packet>;
    let mut n_bits = 1;
    // First bit represents how the length is encoded. 0 for number of bits, 1 for number of packets
    let len_type = bits.next().unwrap();
    // Parse the list of subpackets accordingly
    match len_type{
        0 => {
            let subpack_bit_len = take_bits(bits, 15);
            n_bits += subpack_bit_len + 15;
            subpacks = Vec::new();
            let mut cur_sub_len = 0;
            while cur_sub_len < subpack_bit_len{
                let next_pack = parse_packet(bits);
                cur_sub_len += next_pack.bit_length;
                subpacks.push(next_pack);
            }
        }
        ,1 => {
            let n_packs = take_bits(bits, 11);
            subpacks = Vec::with_capacity(n_packs as usize);
            n_bits += 11;
            for _ in 0..n_packs{
                let next_pack = parse_packet(bits);
                n_bits += next_pack.bit_length;
                subpacks.push(next_pack);
            }
        }
        ,_=> unreachable!()
    }
    (PackData::Operator(subpacks), n_bits)
}

// recursively parse a packet and its payload
fn parse_packet(bits: &mut BitStream) -> Packet{
    let version = take_bits(bits, 3);
    let type_id = take_bits(bits, 3);
    let (data, length) = match type_id{
        4 => parse_literal(bits)
        ,_ => parse_operator(bits)
    };

    Packet{
        version: version as u8
        ,type_id: type_id as u8
        ,data: data
        ,bit_length: length + 6
    }
}

// parse all of the packets in a given input
fn parse_all(input: &[u8]) -> Packet{
    let mut bits = BitStream::new(input);
    parse_packet(&mut bits)
}

// sum all of the version numbers of the packets, recursively
fn version_sum(p: &Packet) -> i64 {
    (p.version as i64) + match &p.data{
        PackData::Operator(subpackets) => subpackets.iter().map(version_sum).sum()
        ,_=>0
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let p = parse_all(input);
    version_sum(&p)
}

// Determine the function that an operator packet applies (n input version)
fn operator(type_id: u8) -> fn(i64, i64) -> i64{
    match type_id{
        0 => i64::add
        ,1 => i64::mul
        ,2 => min
        ,3 => max
        ,_ => unreachable!()
    }
}

// Determine the function that an operator packet applies (two input version)
fn comparison(type_id: u8) -> fn(&i64, &i64) -> bool {
    match type_id{
        5 => i64::gt
        ,6 => i64::lt
        ,7 => i64::eq
        ,_ => unreachable!()
    }
}

// Evaluate a packet recursively
fn eval(p: &Packet) -> i64 {
    match (&p.data, p.type_id){
        (PackData::Literal(x),_) => *x
        ,(PackData::Operator(subpacks), tid) if tid <= 3 => subpacks.iter().map(eval).reduce(operator(tid)).unwrap()
        ,(PackData::Operator(subpacks), tid) => {
            let subs: Vec::<i64> = subpacks.iter().map(eval).collect();
            if comparison(tid)(&subs[0], &subs[1]) {1} else {0}
        }
    }
}

#[aoc(day16, part2)]
pub fn part2(input: &[u8]) -> i64 {
    let p = parse_all(input);
    eval(&p)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, take_bits, BitStream};

    #[test]
    fn test0_0(){ assert_eq!(take_bits(&mut BitStream::new(b"0"), 4), 0x0); }
    #[test]
    fn test0_1(){ assert_eq!(take_bits(&mut BitStream::new(b"1"), 4), 0x1); }
    #[test]
    fn test0_2(){ assert_eq!(take_bits(&mut BitStream::new(b"2"), 4), 0x2); }
    #[test]
    fn test0_3(){ assert_eq!(take_bits(&mut BitStream::new(b"3"), 4), 0x3); }
     #[test]
    fn test0_9(){ assert_eq!(take_bits(&mut BitStream::new(b"9"), 4), 0x9); }
    #[test]
    fn test0_a(){ assert_eq!(take_bits(&mut BitStream::new(b"A"), 4), 0xA); }
    #[test]
    fn test0_f(){ assert_eq!(take_bits(&mut BitStream::new(b"F"), 4), 0xF); }

    #[test]
    fn test1_1(){ assert_eq!(part1(b"38006F45291200"), 9); }
    #[test]
    fn test1_2(){ assert_eq!(part1(b"EE00D40C823060"), 14); }
    #[test]
    fn test1_3(){ assert_eq!(part1(b"8A004A801A8002F478"), 16); }
    #[test]
    fn test1_4(){ assert_eq!(part1(b"620080001611562C8802118E34"), 12); }
    #[test]
    fn test1_5(){ assert_eq!(part1(b"C0015000016115A2E0802F182340"), 23); }
    #[test]
    fn test1_6(){ assert_eq!(part1(b"A0016C880162017C3686B18A3D4780"), 31); }

    #[test]
    fn test2_1(){ assert_eq!(part2(b"C200B40A82"), 3); }
    #[test]
    fn test2_2(){ assert_eq!(part2(b"04005AC33890"), 54); }
    #[test]
    fn test2_3(){ assert_eq!(part2(b"880086C3E88112"), 7); }
    #[test]
    fn test2_4(){ assert_eq!(part2(b"CE00C43D881120"), 9); }
    #[test]
    fn test2_5(){ assert_eq!(part2(b"D8005AC2A8F0"), 1); }
    #[test]
    fn test2_6(){ assert_eq!(part2(b"F600BC2D8F"), 0); }
    #[test]
    fn test2_7(){ assert_eq!(part2(b"9C005AC2F8F0"), 0); }
    #[test]
    fn test2_8(){ assert_eq!(part2(b"9C0141080250320F1802104A08"), 1); }

}

