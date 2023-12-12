use std::collections::HashMap;

fn main() {
    let data = include_str!("./data.txt");
    let result = process(data);
    dbg!(result);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAkind,
    FullHouse,
    FourOfAkind,
    FiveOfAkind
}

#[derive(Debug)]
struct Hand<'a>{
    hand_type: HandType,
    cards: &'a str,
    bet: usize
}




fn count_chars(slice: &str) -> HashMap<char, usize>{
    let mut result= HashMap::new();
    for element in slice.chars() {
        result.entry(element).and_modify(|count| *count += 1).or_insert(1);
    }
    result
}

fn parse_hand_type(data: &str) ->  HandType{
    let mut vals = count_chars(data).into_values().collect::<Vec<_>>();
    vals.sort();
    vals.reverse();
    match vals.as_slice() {
        [5] => HandType::FiveOfAkind,
        [4,1] => HandType::FourOfAkind,
        [3,2] => HandType::FullHouse,
        [3, ..] => HandType::ThreeOfAkind,
        [2,2, ..] => HandType::TwoPair,
        [2, ..] => HandType::OnePair,
        _ => HandType::HighCard
    }
}

fn letter_to_int(c: char) -> u8 {
    match c {
        'A' => 14, 
        'K' => 13,
        'Q' => 12, 
        'J' => 11,
        'T' => 10,
        c if c.is_digit(10) => c.to_digit(10).unwrap() as u8,
        _ => 0
    }
}

fn process(data: &str) -> usize {
    let mut hands: Vec<_> = data.lines().map(|line| line.split_once(' ').unwrap())
        .map(|(cards, bet)|{
            Hand{cards, hand_type: parse_hand_type(cards), bet: bet.parse().unwrap()}
        }).collect();
    hands.sort_by(|a,b| {
        let mut a_chars = a.cards.chars();
        let mut b_chars = b.cards.chars();
        Ord::cmp(&a.hand_type, &b.hand_type)
            .then(Ord::cmp(&letter_to_int(a_chars.next().unwrap()), &letter_to_int(b_chars.next().unwrap())))
            .then(Ord::cmp(&letter_to_int(a_chars.next().unwrap()), &letter_to_int(b_chars.next().unwrap())))
            .then(Ord::cmp(&letter_to_int(a_chars.next().unwrap()), &letter_to_int(b_chars.next().unwrap())))
            .then(Ord::cmp(&letter_to_int(a_chars.next().unwrap()), &letter_to_int(b_chars.next().unwrap())))
            .then(Ord::cmp(&letter_to_int(a_chars.next().unwrap()), &letter_to_int(b_chars.next().unwrap())))
        });
    hands.iter().enumerate().map(|(num, hand)| (num+1)*hand.bet).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let data = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = process(data);
        assert_eq!(result, 6440);
    }
}