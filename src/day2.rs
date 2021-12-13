use std::str;

#[aoc(day2, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let moves = str::from_utf8(input).unwrap().lines().map(|l|l.split(" ").collect::<Vec<&str>>());
    let mut depth = 0;
    let mut tot_dist = 0;
    for move_ in moves{
        let dir = move_[0].bytes().next().unwrap();
        let dist = move_[1].parse::<i64>().unwrap();
        if dir == 'f' as u8{tot_dist += dist;}
        if dir == 'd' as u8{depth += dist;}
        if dir == 'u' as u8{depth -= dist;}
    }
    depth * tot_dist
}

#[aoc(day2, part2)]
pub fn part2(input: &[u8]) -> i64 {
    let moves = str::from_utf8(input).unwrap().lines().map(|l|l.split(" ").collect::<Vec<&str>>());
    let mut depth = 0;
    let mut aim = 0;
    let mut tot_dist = 0;
    for move_ in moves{
        let dir = move_[0].bytes().next().unwrap();
        let dist = move_[1].parse::<i64>().unwrap();
        if dir == 'f' as u8{
            tot_dist += dist;
            depth += aim * dist;
        }
        if dir == 'd' as u8{aim += dist;}
        if dir == 'u' as u8{aim -= dist;}
    }
    depth * tot_dist
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1(){
        assert_eq!(part1(
b"forward 5
down 5
forward 8
up 3
down 8
forward 2
"
        ), 150);
    }

    #[test]
    fn test2(){
        assert_eq!(part2(b"forward 5
down 5
forward 8
up 3
down 8
forward 2
"), 900);
    }
}

