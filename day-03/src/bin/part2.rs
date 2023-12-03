use core::num;
use std::{collections::{HashSet, hash_set}, num::ParseIntError};

fn main() {
    let data = include_str!("./data.txt");
    let result = process(data);
    dbg!(result);
}

//#[derive(Default, Debug)]

fn process(data: &str) -> Result<i32, ParseIntError> {
    let linelength = data.lines().next().unwrap().len() as i32 + 1 ;
    let datalength = data.len();
    let gears = data.chars().zip(0i32..)
        .filter_map(|tup| {
            if tup.0 == '*' {
                Some(tup.1)
            } else {
                None
            }
        });

    
    
    let mut numbers: Vec<i32> = vec![];
    for gear in gears {
        let mut gearnumbers: HashSet<i32> = HashSet::new();
        for yoffset in -1i32..=1 {
            for xoffset in -1..=1 {
                let trycharpos: Result<usize, _> = (gear + yoffset * linelength + xoffset).try_into();
                if trycharpos.is_err() {
                    continue;
                }
                let mut charpos = trycharpos.unwrap();
                if charpos < datalength {
                    if data.chars().nth(charpos).unwrap().is_digit(10) {
                        while data.chars().nth(charpos).unwrap().is_digit(10) {
                            if charpos == 0 {
                                break;
                            }
                            charpos -= 1;
                        } 
                        charpos += 1;
                        let mut endpos = charpos;
                        while data.chars().nth(endpos).unwrap_or('.').is_digit(10) {
                            endpos += 1;
                        }

                        gearnumbers.insert(data[charpos..endpos].parse()?);
                    }
                }
            }
        }
        if gearnumbers.len() == 2 {
            numbers.push(gearnumbers.iter().fold(1, |acc, x| acc * x));
        }
    }
    Ok(numbers.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let data = ".467.114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = process(data);
        assert_eq!(result, Ok(467835));
    }
}