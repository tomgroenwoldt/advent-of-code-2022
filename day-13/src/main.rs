#![feature(iter_array_chunks)]
use itertools::{EitherOrBoth, Itertools};
use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, PartialEq, Eq)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn new() -> Self {
        Packet::List(vec![])
    }
}

fn read_packet(line: &Vec<char>, index: &mut usize, outer_packet: &mut Packet) {
    loop {
        if *index > line.len() - 1 {
            break;
        }

        // List ends.
        if line[*index] == ']' {
            break;
        }

        // New list starts.
        if line[*index] == '[' {
            *index += 1;
            let mut packet = Packet::new();
            read_packet(line, index, &mut packet);
            if let Packet::List(vec) = outer_packet {
                vec.push(packet);
            }
        }

        if let Some(mut digit) = line[*index].to_digit(10) {
            // Because I'm reading the numbers char-wise, I am not able
            // to parse a 10. It took me way too long to figure out, that
            // my input had 10s in it. The example input was restricted to
            // single digit numbers.
            if line[*index + 1].is_ascii_digit() {
                digit = 10;
            }
            if let Packet::List(vec) = outer_packet {
                vec.push(Packet::Number(digit));
            }
        }
        *index += 1;
    }
}

fn packets_in_right_order(
    left: &[Packet],
    right: &[Packet],
    packet_status: &mut bool,
    recursion_stop: &mut bool,
) {
    for pair in left.iter().zip_longest(right) {
        if *recursion_stop {
            return;
        }
        match pair {
            EitherOrBoth::Both(left, right) => match (left, right) {
                (Packet::Number(left), Packet::Number(right)) => match left.cmp(right) {
                    Ordering::Less => {
                        *packet_status = true;
                        *recursion_stop = true;
                    }
                    Ordering::Equal => {
                        continue;
                    }
                    Ordering::Greater => {
                        *packet_status = false;
                        *recursion_stop = true;
                    }
                },
                (Packet::Number(left), right) => {
                    let right_clone = right.clone();
                    packets_in_right_order(
                        &[Packet::List(vec![Packet::Number(*left)])],
                        &[right_clone],
                        packet_status,
                        recursion_stop,
                    );
                }
                (left, Packet::Number(right)) => {
                    let left_clone = left.clone();
                    packets_in_right_order(
                        &[left_clone],
                        &[Packet::List(vec![Packet::Number(*right)])],
                        packet_status,
                        recursion_stop,
                    );
                }
                (Packet::List(left), Packet::List(right)) => {
                    packets_in_right_order(left, right, packet_status, recursion_stop);
                }
            },
            // Left runs out of input.
            EitherOrBoth::Right(_) => {
                *packet_status = true;
                *recursion_stop = true;
            }
            // Right runs out of input.
            EitherOrBoth::Left(_) => {
                *packet_status = false;
                *recursion_stop = true;
            }
        }
    }
}

fn order_packets(packets: &mut Vec<Packet>) {
    // Push divider packets.
    packets.push(Packet::List(vec![Packet::Number(2)]));
    packets.push(Packet::List(vec![Packet::Number(6)]));

    // Bubble sort.
    for _ in 0..packets.len() {
        for j in 0..packets.len() - 1 {
            let mut packet_status = true;
            let mut recursion_stop = false;

            if let (Packet::List(left), Packet::List(right)) = (&packets[j], &packets[j + 1]) {
                packets_in_right_order(left, right, &mut packet_status, &mut recursion_stop);
            } else {
                panic!("Error parsing packets.");
            }

            if !packet_status {
                packets.swap(j, j + 1);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut packets: Vec<Packet> = vec![];

    for line in reader.lines().flatten() {
        let mut index = 1;
        if line.is_empty() {
            continue;
        }

        let line = line.chars().collect::<Vec<_>>();
        let mut packet = Packet::new();
        read_packet(&line, &mut index, &mut packet);
        packets.push(packet);
    }

    let ordered_packets = packets
        .iter()
        .array_chunks::<2>()
        .map(|[left, right]| {
            let mut packet_status = true;
            let mut recursion_stop = false;
            if let (Packet::List(left), Packet::List(right)) = (left, right) {
                packets_in_right_order(left, right, &mut packet_status, &mut recursion_stop);
                packet_status
            } else {
                panic!("Error parsing packets.");
            }
        })
        .collect::<Vec<_>>();

    let ordered_packets_indices_sum = ordered_packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| **packet)
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    println!(
        "The sum of the indices of ordered packet pairs is {}.",
        ordered_packets_indices_sum
    );

    order_packets(&mut packets);

    let divider_packet_indices = packets
        .into_iter()
        .enumerate()
        .filter(|(_, packet)| {
            *packet == Packet::List(vec![Packet::Number(2)])
                || *packet == Packet::List(vec![Packet::Number(6)])
        })
        .map(|(i, _)| i + 1)
        .collect::<Vec<_>>();

    println!(
        "The divider packets are {}th and {}th. Thus the decoder key is {}.",
        divider_packet_indices[0],
        divider_packet_indices[1],
        divider_packet_indices[0] * divider_packet_indices[1],
    );

    Ok(())
}
