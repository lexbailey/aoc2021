use std::str;
use std::cmp::{min,max};

#[aoc(day7, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let crabs: Vec<i64> = str::from_utf8(input).unwrap().trim().split(",").map(|a|a.parse::<i64>().unwrap()).collect();
    let minp = *crabs.iter().reduce(min).unwrap();
    let maxp = *crabs.iter().reduce(max).unwrap();
    let mut min_cost: Option<i64> = None;
    for p in minp..=maxp{
        let cost = crabs.iter().map(|x|i64::abs(x-p)).sum();
        print!("Position {} costs {}\n", p, cost);
        min_cost = Some(match min_cost {None => cost, Some(c) => min(c,cost)});
    }
    min_cost.unwrap()
}

#[aoc(day7, part2)]
pub fn part2(input: &[u8]) -> i64 {
    let crabs: Vec<i64> = str::from_utf8(input).unwrap().trim().split(",").map(|a|a.parse::<i64>().unwrap()).collect();
    let minp = *crabs.iter().reduce(min).unwrap();
    let maxp = *crabs.iter().reduce(max).unwrap();
    let mut min_cost: Option<i64> = None;
    for p in minp..=maxp{
        let cost = crabs.iter().map(|x|{let d = i64::abs(x-p); (d*(d+1))/2}).sum();
        print!("Position {} costs {}\n", p, cost);
        min_cost = Some(match min_cost {None => cost, Some(c) => min(c,cost)});
    }
    min_cost.unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1_1(){
        assert_eq!(part1(b"16,1,2,0,4,2,7,1,2,14\n"), 37);
    }

    #[test]
    fn test2_1(){
        assert_eq!(part2(b"16,1,2,0,4,2,7,1,2,14\n"), 168);
    }
}

