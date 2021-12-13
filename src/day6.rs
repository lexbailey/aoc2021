use std::str;

fn step(fish: &mut Vec<i64>){
    let n_new = fish.iter().map(|f|if *f==0{1}else{0}).sum();
    for f in fish.iter_mut(){
        *f -= 1;
        if *f < 0 {*f = 6;}
    }
    for _ in 0..n_new{
        fish.push(8);
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let mut fish: Vec<i64> = str::from_utf8(input).unwrap().trim().split(",").map(|a|a.parse::<i64>().unwrap()).collect();
    for _ in 0..80{ step(&mut fish); }
    fish.len() as i64
}

fn reduce_buckets(b: &Vec<(i64, i64)>) -> Vec<(i64,i64)>{
    let mut result: Vec<(i64,i64)> = vec![];
    for i in 0..=8{
        result.push((i,b.iter().filter(|(f,_n)|*f==i).map(|(_f,n)|*n).sum()))
    }
    result
}

fn simulate_buckets(b: &mut Vec<(i64,i64)>){
    for i in 0..b.len(){
        let (f,n) = b[i];
        b.push((f,-n));
        if f == 0 {
            b.push((8, n));
            b.push((6, n));
        }
        else{
            b.push((f-1,n));
        }
    }
}

#[aoc(day6, part2)]
pub fn part2(input: &[u8]) -> i64 {
    let fish: Vec<i64> = str::from_utf8(input).unwrap().trim().split(",").map(|a|a.parse::<i64>().unwrap()).collect();
    let mut buckets: Vec<(i64,i64)> = Vec::with_capacity(9);
    for f in fish{
        buckets.push((f, 1));
    }
    buckets = reduce_buckets(&buckets);
    for _ in 0..256{
        simulate_buckets(&mut buckets);
        buckets = reduce_buckets(&buckets);
    }
    buckets.iter().map(|(_f,n)|n).sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1_1(){
        assert_eq!(part1(b"3,4,3,1,2"), 5934);
    }

    #[test]
    fn test2_1(){
        assert_eq!(part2(b"3,4,3,1,2"), 26984457539);
    }
}

