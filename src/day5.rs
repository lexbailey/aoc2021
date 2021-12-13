use std::str;
use std::cmp::{max,min};
use itertools::Itertools;

fn coords(input: &[u8]) -> Vec<(i64,i64,i64,i64)> {
    let lines: Vec<&[u8]> = str::from_utf8(input).unwrap().trim().lines().map(str::as_bytes).collect();
    let mut result = vec!();
    for line in lines.iter() {
        let mut a = str::from_utf8(line).unwrap().split(" -> ").map(|s|s.split(","));
        let nums: Vec<i64> = a.next().unwrap().chain(a.next().unwrap()).map(|s|s.parse::<i64>().unwrap()).collect();
        result.push((nums[0], nums[1], nums[2], nums[3]));
    }
    result
}

fn limits(coords: &Vec<(i64, i64, i64, i64)>) -> (i64, i64) {
    let mut maxx = 0;
    let mut maxy = 0;
    for (x1,y1,x2,y2) in coords {
        maxx = max(maxx, max(*x1, *x2));
        maxy = max(maxy, max(*y1, *y2));
    }
    (maxx, maxy)
}

fn is_hor_ver(x1: i64, y1:i64, x2:i64, y2:i64) -> bool {
    x1 == x2 || y1 == y2
}

fn draw_lines(grid: &mut Vec<Vec<i64>>, coords: &Vec<(i64,i64,i64,i64)>){
    for (x1,y1,x2,y2) in coords.iter() {
        if is_hor_ver(*x1,*y1,*x2,*y2) {
            for (x,y) in (min(*x1,*x2)..=max(*x1,*x2)).cartesian_product(min(*y1,*y2)..=max(*y1,*y2)) {
                grid[x as usize][y as usize] += 1;
            }
        }
    }
}

fn draw_lines_with_diagonal(grid: &mut Vec<Vec<i64>>, coords: &Vec<(i64,i64,i64,i64)>){
    draw_lines(grid, coords);
    for (x1,y1,x2,y2) in coords.iter() {
        if !is_hor_ver(*x1,*y1,*x2,*y2) {
            let sx = min(*x1,*x2);
            let ex = max(*x1,*x2);
            let mut x = sx;
            let mut y = if sx == *x1 {*y1} else {*y2};
            let dy = if sx == *x1 {
                    if *y2 > *y1 {1} else {-1}
                } else {
                    if *y1 > *y2 {1} else {-1}
                };
            while x <= ex{
                grid[x as usize][y as usize] += 1;
                x += 1;
                y += dy;
            }
        }
    }
}

fn solve(input: &[u8], draw_fun: fn(grid: &mut Vec<Vec<i64>>, coords: &Vec<(i64,i64,i64,i64)>)) -> i64 {
    let c = coords(input);
    let (maxx, maxy) = limits(&c);
    let mut grid : Vec<Vec<i64>> = Vec::with_capacity(1+maxx as usize);
    for _ in 0..=maxx as usize {
        grid.push(vec![0; 1+maxy as usize]);
    }
    
    draw_fun(&mut grid, &c);

    let mut total = 0;

    for (x,y) in (0..=maxx).cartesian_product(0..=maxy) {
        total += if grid[x as usize][y as usize] > 1 {1} else {0};
    }

    total
}

#[aoc(day5, part1)]
pub fn part1(input: &[u8]) -> i64 {
    solve(input, draw_lines)
}

#[aoc(day5, part2)]
pub fn part2(input: &[u8]) -> i64 {
    solve(input, draw_lines_with_diagonal)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1(){
        assert_eq!(part1(
            b"0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2"), 5);
    }

    #[test]
    fn test2(){
        assert_eq!(part2(
            b"0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2"), 12);
    }

}



