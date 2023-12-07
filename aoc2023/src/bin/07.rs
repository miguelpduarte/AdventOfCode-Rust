#![feature(test)]

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

extern crate test;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    pub bid: usize,
    pub hand_type: HandType,
    pub numberized_hand: Vec<u8>,
}

impl Hand {
    fn new(hand_str: &str, bid: usize) -> Hand {
        let hand_map = hand_str
            .chars()
            .fold(HashMap::<char, u8>::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            });

        let mut max_equals_heap = hand_map.values().cloned().collect::<BinaryHeap<_>>();

        let hand_type = match max_equals_heap.pop().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => match max_equals_heap.pop() {
                Some(2) => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            },
            2 => match max_equals_heap.pop() {
                Some(2) => HandType::TwoPair,
                _ => HandType::OnePair,
            },
            _ => HandType::HighCard,
        };

        Hand {
            bid,
            hand_type,
            numberized_hand: hand_str.chars().map(card_to_number).collect(),
        }
    }
}

#[derive(Debug)]
struct JokerHand {
    pub bid: usize,
    pub hand_type: HandType,
    pub numberized_hand: Vec<u8>,
}

impl JokerHand {
    fn new(hand_str: &str, bid: usize) -> JokerHand {
        let mut hand_map = hand_str
            .chars()
            .fold(HashMap::<char, u8>::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            });

        // Number of jokers in hand, that can "upgrade" hand types
        let joker_delta = hand_map.remove(&'J').unwrap_or(0);

        let mut max_equals_heap = hand_map.values().cloned().collect::<BinaryHeap<_>>();

        // We can just add the joker_delta here, because the jokers should always convert to the most frequent card
        // Even when we have a trio, it is better to make it a four of a kind, rather than a full house.
        // .unwrap_or(0) handles cases in which the whole hand are jokers.
        let hand_type = match max_equals_heap.pop().unwrap_or(0) + joker_delta {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => match max_equals_heap.pop() {
                Some(2) => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            },
            2 => match max_equals_heap.pop() {
                Some(2) => HandType::TwoPair,
                _ => HandType::OnePair,
            },
            _ => HandType::HighCard,
        };

        JokerHand {
            bid,
            hand_type,
            numberized_hand: hand_str.chars().map(card_to_number_jokerized).collect(),
        }
    }
}

fn card_to_number(c: char) -> u8 {
    if c.is_numeric() {
        c.to_digit(10).unwrap().try_into().unwrap()
    } else {
        match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Unexpected card"),
        }
    }
}

fn card_to_number_jokerized(c: char) -> u8 {
    if c.is_numeric() {
        c.to_digit(10).unwrap().try_into().unwrap()
    } else {
        match c {
            'T' => 10,
            'J' => 0, // Now weakest individual cards
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Unexpected card"),
        }
    }
}

fn solve_day(input: String) -> (usize, usize) {
    let mut hands = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            Hand::new(hand, bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    // println!("pre-sort: {:?}", hands);

    hands.sort_unstable_by(|a, b| {
        if a.hand_type != b.hand_type {
            return a.hand_type.cmp(&b.hand_type);
        }

        // Hands are of the same type, so now we compare individual cards
        for (a_card, b_card) in a.numberized_hand.iter().zip(b.numberized_hand.iter()) {
            match a_card.cmp(b_card) {
                Ordering::Equal => continue,
                // If greater or less, we have found our ordering
                x => {
                    return x;
                }
            }
        }

        // Sanity
        unreachable!("Could not find ordering for cards!");
    });

    /*
    hands.sort_unstable_by(|a, b| {
        // println!(
        //     "Comparing: {}_{:?} {}_{:?}",
        //     a.bid, a_pair_counts, b.bid, b_pair_counts
        // );

        if a.max_equals != b.max_equals {
            // If one of the max pairs is larger than the other, it is always stronger
            return a.max_equals.cmp(&b.max_equals);
        }

        // The max pairs is the same for both

        // Check for either full house or two pair
        if a.max_equals == 3 || a.max_equals == 2 {
            let a_second = a.second_max_equals.unwrap();
            let b_second = b.second_max_equals.unwrap();
            if a_second == 2 && b_second != 2 {
                // A is and B not
                return Ordering::Greater;
            } else if b_second == 2 {
                // B is and A not
                return Ordering::Less;
            }
        }

        println!("Tied! {:?} and {:?}", a, b);
        // Cannot decide based on #pairs, so we must check the items between each other to break ties.
        for (a_card, b_card) in a.numberized_hand.iter().zip(b.numberized_hand.iter()) {
            println!("Checking ({}, {})", a_card, b_card);
            match a_card.cmp(b_card) {
                Ordering::Equal => continue,
                // If greater or less, we have found our ordering
                x => {
                    println!("Untied:{:?}", x);
                    return x;
                }
            }
        }

        // Sanity, probably should be unreachable?
        // Ordering::Equal
        unreachable!("Oh no!");
    });
    */

    // println!("{:?}", hands);

    let p1 = hands
        .into_iter()
        .enumerate()
        // .inspect(|x| println!("{:?}", x))
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum();

    let mut hands_p2 = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            JokerHand::new(hand, bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    // println!("pre-sort: {:?}", hands);

    hands_p2.sort_unstable_by(|a, b| {
        if a.hand_type != b.hand_type {
            return a.hand_type.cmp(&b.hand_type);
        }

        // Hands are of the same type, so now we compare individual cards
        for (a_card, b_card) in a.numberized_hand.iter().zip(b.numberized_hand.iter()) {
            match a_card.cmp(b_card) {
                Ordering::Equal => continue,
                // If greater or less, we have found our ordering
                x => {
                    return x;
                }
            }
        }

        // Sanity
        unreachable!("Could not find ordering for cards!");
    });

    let p2 = hands_p2
        .into_iter()
        .enumerate()
        // .inspect(|x| println!("{:?}", x))
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum();

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 6440);
    assert_eq!(res.1, 5905);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "7.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 248113761);
    assert_eq!(res.1, 246285222);
}

aoc2023::day_main!("7.in");
