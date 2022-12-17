use std::cmp::Ordering;

use crate::Solution;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Value(u32),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (lhs, rhs) = (self, other);
        match (lhs, rhs) {
            (Self::Value(l), Self::Value(r)) => l.partial_cmp(r),
            (Self::Value(l), Self::List(_r)) => {
                Self::List(vec![Self::Value(*l)]).partial_cmp(rhs)
            }
            (Self::List(_l), Self::Value(r)) => {
                lhs.partial_cmp(&Self::List(vec![Self::Value(*r)]))
            }
            (Self::List(l), Self::List(r)) => {
                for i in 0..l.len() {
                    if i >= r.len() {
                        return Some(Ordering::Greater);
                    }
                    if let Some(x) = l[i].partial_cmp(&r[i]) {
                        match x {
                            Ordering::Less | Ordering::Greater => return Some(x),
                            Ordering::Equal => continue,
                        }
                    }
                }
                if l.len() == r.len() {
                    return Some(Ordering::Equal);
                } else {
                    return Some(Ordering::Less);
                };
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Day13;

impl Solution for Day13 {
    fn part1(&self, input: &str) -> String {
        let pairs = parse_input(input);
        let mut result = 0;
        for i in 0..pairs.len() {
            if let Some(Ordering::Less) = pairs[i][0].partial_cmp(&pairs[i][1]) {
                result += i + 1;
            }
        }
        format!("{result}")
    }

    fn part2(&self, input: &str) -> String {
        let mut pairs = parse_input(input);
        let left = parse_packet("[[2]]").0.unwrap();
        let right = parse_packet("[[6]]").0.unwrap();
        let dividers = vec![left.clone(), right.clone()];
        pairs.push(dividers);
        let mut packets: Vec<&Packet> = pairs.iter().flatten().collect();
        packets.sort();
        let x = packets.iter().position(|p| **p == left).unwrap() + 1;
        let y = packets.iter().position(|p| **p == right).unwrap() + 1;
        format!("{}", x * y)
    }
}

fn parse_input(input: &str) -> Vec<Vec<Packet>> {
    input
        .trim()
        .split("\n\n")
        .map(|s| s.split('\n').map(|l| parse_packet(l).0.unwrap()).collect())
        .collect()
}

fn parse_packet(input: &str) -> (Option<Packet>, usize) {
    match &input[0..1] {
        "[" => {
            let mut packets = Vec::new();
            let mut read = 1;
            while read < input.len() {
                let (packet, new) = parse_packet(&input[read..]);
                read += new;
                if let Some(p) = packet {
                    packets.push(p);
                }
                if &input[read - 1..read] == "]" {
                    read += 1;
                    break;
                }
            }
            (Some(Packet::List(packets)), read)
        }
        "]" => (None, 1),
        "," => (None, 1),
        "0" => (Some(Packet::Value(0)), 1),
        "2" => (Some(Packet::Value(2)), 1),
        "3" => (Some(Packet::Value(3)), 1),
        "4" => (Some(Packet::Value(4)), 1),
        "5" => (Some(Packet::Value(5)), 1),
        "6" => (Some(Packet::Value(6)), 1),
        "7" => (Some(Packet::Value(7)), 1),
        "8" => (Some(Packet::Value(8)), 1),
        "9" => (Some(Packet::Value(9)), 1),
        "1" => {
            if &input[1..2] == "0" {
                (Some(Packet::Value(10)), 2)
            } else {
                (Some(Packet::Value(1)), 1)
            }
        }
        _ => panic!(),
    }
}

impl Day13 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex1() {
        let soln = Day13 {};
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
        let result = soln.part1(input);
        assert_eq!(result, "13");
    }
}
