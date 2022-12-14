use crate::util::open_file;
use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

type Packets = Vec<Pair>;

#[derive(Debug)]
pub struct Pair {
    left: Packet,
    right: Packet,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Packet {
    Integer(u8),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(input: &str) -> Self {
        packet(input).unwrap().1
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(l), Packet::Integer(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => l.cmp(r),
            (Packet::Integer(l), Packet::List(r)) => vec![Packet::Integer(*l)].cmp(r),
            (Packet::List(l), Packet::Integer(r)) => l.cmp(&vec![Packet::Integer(*r)]),
        }
    }
}

fn packet(input: &str) -> IResult<&str, Packet> {
  alt(
    (
      delimited(
        tag("["), 
        separated_list0(tag(","), packet), 
        tag("]")
      ).map(Packet::List),
      nom::character::complete::u8.map(Packet::Integer),
    )
  )(input)
}

fn parse_pairs(input: &str) -> IResult<&str, Vec<Pair>> {
  separated_list1(
    tag("\n\n"),
    separated_pair(
      packet, 
      newline, 
      packet
    ).map(|(left, right)| Pair { left, right }),
  )(input)
}

pub fn pairs() {
  let contents = open_file("data/day13.txt");
  let (_, pairs) = parse_pairs(&contents).unwrap();

  let result:usize = pairs
    .iter()
    .enumerate()
    .filter_map(|(i, Pair { left, right })| match left.cmp(right) {
        Ordering::Less => Some(i + 1),
        _ => None,
    })
    .sum();

  println!("Result: {}", result);
}

pub fn pairs_two() {
  let contents = open_file("data/day13.txt");
  let (_, pairs) = parse_pairs(&contents).unwrap();

  let mut packets: Vec<&Packet> = pairs
      .iter()
      .flat_map(|Pair { left, right }| [left, right])
      .collect();

  packets.sort_unstable();

  let i = packets.binary_search(&&Packet::parse("[[2]]")).unwrap_err() + 1;
  let j = packets.binary_search(&&Packet::parse("[[6]]")).unwrap_err() + 2;

  println!("Result: {}", i * j);
}