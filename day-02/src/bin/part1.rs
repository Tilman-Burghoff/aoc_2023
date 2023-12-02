use std::default;

fn main() {
    let data = include_str!("./data.txt");
    let result = process(data);
    dbg!(result);
}

#[derive(Default, Debug)]
struct Pull {
    red: u8,
    green: u8,
    blue: u8
}

fn game_allowed(line: &str) -> bool {
    line.split(';')
        .map(|x| {
            let mut result: Pull = Default::default();
            for color in  x.split(',') {
                let mut iter = color.trim().split(' ');
                match (iter.next().unwrap(), iter.next().unwrap()) {
                    (num, "red") => result.red = num.parse().unwrap(),
                    (num, "green") => result.green = num.parse().unwrap(),
                    (num, "blue") => result.blue = num.parse().unwrap(),
                    _ => ()
                }
            };
            result
        })
        .fold(true, |acc, x| acc && x.red <= 12 && x.green <= 13 && x.blue <= 14)
}

fn process(data: &str) -> usize {
    data.lines()
        .enumerate()
        .fold(0, |acc, x| {
            if game_allowed(x.1.split(':').last().unwrap()) {
                acc + x.0 + 1
            } else {
                acc
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = process(data);
        assert_eq!(result, 8);
    }
}