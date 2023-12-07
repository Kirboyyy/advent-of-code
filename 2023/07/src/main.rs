use std::{cmp::Ordering, collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Can't read file!");

    let mut tuples: Vec<((HandResult, Vec<char>), u32)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                check_hand(parts.next().unwrap()),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();

    tuples.sort_by(|(a, _), (b, _)| {
        let result_a = a.0 as u8;
        let result_b = b.0 as u8;

        if result_a != result_b {
            return result_a.cmp(&result_b);
        }

        for i in 0..a.1.len().min(b.1.len()) {
            let card_a = a.1.iter().nth(i).unwrap();
            let card_b = b.1.iter().nth(i).unwrap();

            let index_a = CARDS.iter().position(|c| c == card_a).unwrap();
            let index_b = CARDS.iter().position(|c| c == card_b).unwrap();

            if index_a != index_b {
                return index_b.cmp(&index_a);
            }
        }
        return Ordering::Equal;
    });

    let mut value = 0;
    let mut i = 1;
    for (t, bet) in tuples {
        println!("{i}: {:?}", t);
        value += i * bet;
        i += 1;
    }

    println!("result is {value}");
}

const CARDS: &'static [char] = &[
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', '1', 'J',
];

#[derive(Clone, Copy, Debug)]
enum HandResult {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn check_hand(hand: &str) -> (HandResult, Vec<char>) {
    let chars: Vec<char> = hand.chars().collect();
    let distinct_chars: HashSet<char> = chars.clone().into_iter().collect();
    let joker_count = chars.iter().clone().filter(|&c| c == &'J').count();

    if distinct_chars.len() == 1 {
        return (HandResult::FiveOfAKind, chars);
    }

    let chars_iter = chars.iter().into_iter();
    let mut distinct_chars_iter = distinct_chars.iter();

    let first_char = distinct_chars_iter.next().unwrap();
    let first_count = chars_iter.clone().filter(|&c| c == first_char).count();

    let second_char = distinct_chars_iter.next().unwrap();
    let second_count = chars_iter.clone().filter(|&c| c == second_char).count();

    if distinct_chars.len() == 2 {
        if first_count == 4 || second_count == 4 {
            if joker_count > 0 {
                return (HandResult::FiveOfAKind, chars);
            }
            return (HandResult::FourOfAKind, chars);
        }
        return (HandResult::FullHouse, chars);
    }

    let third_char = distinct_chars_iter.next().unwrap();
    let third_count = chars_iter.clone().filter(|&c| c == third_char).count();

    if distinct_chars.len() == 3 {
        if first_count == 3 || second_count == 3 || third_count == 3 {
            if joker_count > 0 {
                return (HandResult::FourOfAKind, chars);
            }
            return (HandResult::ThreeOfAKind, chars);
        }
        if (first_count == 2 && second_count == 2)
            || (first_count == 2 && third_count == 2)
            || (second_count == 2 && third_count == 2)
        {
            if joker_count > 0 {
                if joker_count == 2 {
                    return (HandResult::FourOfAKind, chars);
                }
                return (HandResult::FullHouse, chars);
            }
            return (HandResult::TwoPair, chars);
        }
    }


    let fourth_char = distinct_chars_iter.next().unwrap();
    let fourth_count = chars_iter.clone().filter(|&c| c == fourth_char).count();

    if first_count == 2 || second_count == 2 || third_count == 2 || fourth_count == 2 {
        if joker_count > 0 {
            if joker_count == 2 {
                return (HandResult::ThreeOfAKind, chars);
            }
            return (HandResult::ThreeOfAKind, chars);
        }
        return (HandResult::OnePair, chars);
    }

    if joker_count > 0 {
        return (HandResult::OnePair, chars);
    }
    return (HandResult::HighCard, chars);
}
