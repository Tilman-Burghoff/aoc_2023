fn main() {
    let data = include_str!("./data.txt");
    let result = process(data);
    dbg!(result);
}

fn process(data: &str) -> u32 {
    let mut res = 0;
    for line in data.lines() {
        res += 10 * line.chars().filter_map(|x| x.to_digit(10)).next().unwrap_or_default();
        res +=      line.chars().filter_map(|x| x.to_digit(10)).last().unwrap_or_default();
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let data = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let result = process(data);
        assert_eq!(result, 142);
    }
}