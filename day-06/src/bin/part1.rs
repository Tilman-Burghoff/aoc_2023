fn main() {
    let data = include_str!("./data.txt");
    let result = process(data);
    dbg!(result);
}

fn to_num_iter(data: &str) ->  impl Iterator<Item = f64> + '_ {
    data.split_once(':').unwrap().1
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
}


fn process(data: &str) -> i64 {
    let (time, distance) = data.split_once('\n').unwrap();
    to_num_iter(time).zip(to_num_iter(distance))
        .map(|(t, d)| {
            let ub = (t + (t.powf(2.) - 4.*d).sqrt())/2.;
            let lb = (t - (t.powf(2.) - 4.*d).sqrt())/2.;
            (ub.ceil() - lb.floor() - 1. )as i64
        }).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let data = "Time:      7  15   30
Distance:  9  40  200";
        let result = process(data);
        assert_eq!(result, 288);
    }
}