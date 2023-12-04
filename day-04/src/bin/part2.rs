use std::{collections::HashSet, num::ParseIntError};

fn main() {
    let data = include_str!("./data.txt");
    let result = process(data);
    dbg!(result);
}

fn get_numberset(str_of_nums: &str) -> Result<HashSet<i32>, ParseIntError> {
    let mut res = HashSet::new();
    for item in str_of_nums.trim().split_whitespace() {
        res.insert(item.parse()?);
    }
    Ok(res)
}



fn process(data: &str) -> usize {
    let scratchcard_wins = data.lines()
        .map(|x| x.split_once(':').unwrap().1)
        .map(|x| x.split_once('|').unwrap())
        .map(|tup| (tup.0, get_numberset(tup.1).unwrap()))
        .map(|tup| {
            tup.0.trim().split_whitespace()
                .map(|n| n.parse().unwrap()).filter(|n| tup.1.contains(n))
                .count()
        });
    let mut scratchcard_copies = vec![1; data.lines().count()];
    let mut sum = 0;
    for (index, wins) in scratchcard_wins.enumerate(){
        for shift in 1..=wins {
            if scratchcard_copies.len() > index+shift {
                scratchcard_copies[index+shift] += scratchcard_copies[index]
            }
        }
        sum += scratchcard_copies[index];
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = process(data);
        assert_eq!(result, 30);
    }
}