fn main() {
    let data = include_str!("./data.txt");
    let result = process(data);
    dbg!(result);
}

#[derive(Debug)]
struct Map{
    ranges: Vec<Maprange>
}

#[derive(Debug)]
struct Maprange{
    lb: i64,
    ub: i64,
    offset: i64
}

fn parse_map(data: &str) -> Map {
    Map{ranges: data.split_once('\n').unwrap().1 //ignore name
        .lines()
        .map(|line| {let mut nums = line.split_ascii_whitespace();
            let dest_range = nums.next().unwrap().parse::<i64>().unwrap();
            let lb = nums.next().unwrap().parse().unwrap();             
            let ub = lb + nums.next().unwrap().parse::<i64>().unwrap();
            let offset = dest_range - lb;
            Maprange{lb, ub, offset}
        }).collect()
    }
}

fn process(data: &str) -> i64 {
    let (seeds, maps) = data.split_once("\n\n").unwrap();
    let maps_iter = maps.split("\n\n").map(parse_map);
    let mut seeds_vec: Vec<i64> = seeds.split_once(':').unwrap().1
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    for map in maps_iter{
        for seed_idx in 0..seeds_vec.len() {
            for rg in &map.ranges {
                if rg.lb <= seeds_vec[seed_idx] && rg.ub > seeds_vec[seed_idx] {
                    seeds_vec[seed_idx] += rg.offset;
                    break;
                }
            }
        }
    }
    *seeds_vec.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let data = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = process(data);
        assert_eq!(result, 35);
    }
}