fn main() {
    let data = include_str!("./data.txt");
    let result = process(data);
    dbg!(result);
}

fn translate_line(input: &str) -> Vec<u32> {
    let mut out: Vec<u32> = vec![];
    //let chars = input.chars().collect();
    for index in 0..input.len() {
        if input[index..].starts_with("one") {
            out.push(1)
        } else if input[index..].starts_with("two") {
            out.push(2)
        } else if input[index..].starts_with("three") {
            out.push(3)
        } else if input[index..].starts_with("four") {
            out.push(4)
        } else if input[index..].starts_with("five") {
            out.push(5)
        } else if input[index..].starts_with("six") {
            out.push(6)
        } else if input[index..].starts_with("seven") {
            out.push(7)
        } else if input[index..].starts_with("eight") {
            out.push(8)
        } else if input[index..].starts_with("nine") {
            out.push(9)
        } else if input[index..].starts_with("zero") {
            out.push(0)
        } else if input[index..].chars().next().unwrap().is_digit(10) {
            out.push(input[index..].chars().next().unwrap().to_digit(10).unwrap())
        }
    }
    out
}

fn process(data: &str) -> u32 {
    let mut res = 0;
    for line in data.lines() {
        let nums = translate_line(line);

        res += 10 * nums.first().unwrap_or(&0);
        res +=      nums.last().unwrap_or(&0);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let data = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        ";
        let result = process(data);
        assert_eq!(result, 281);
    }
}