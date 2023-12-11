fn main() {
    let data = include_str!("./data.txt");
    let result = process(data);
    dbg!(result);
}

fn to_num(data: &str) ->  f64 {
    data.split_once(':').unwrap().1
        .trim().replace(' ', "")
        .parse().unwrap()
}


fn process(data: &str) -> i64 {
    let (time, distance) = data.split_once('\n').unwrap();
    let t = to_num(time);
    let d = to_num(distance);
    let ub = (t + (t.powf(2.) - 4.*d).sqrt())/2.;
    let lb = (t - (t.powf(2.) - 4.*d).sqrt())/2.;
    (ub.ceil() - lb.floor() - 1. ) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let data = "Time:      7  15   30
Distance:  9  40  200";
        let result = process(data);
        assert_eq!(result, 71503);
    }
}