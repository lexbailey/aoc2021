use std::str;

fn input_nums(input: &[u8]) -> (i32,i32,i32,i32){
    let nums = str::from_utf8(&input[13..]).unwrap().trim();
    let pairs: Vec<&str> = nums.split(",").map(|s|s.trim()).collect();
    let x : Vec<i32>= pairs[0][2..].split("..").map(|a|a.parse::<i32>().unwrap()).collect();
    let y : Vec<i32>= pairs[1][2..].split("..").map(|a|a.parse::<i32>().unwrap()).collect();
    (x[0],x[1],y[0],y[1])
}


fn tri(n: i32) -> i32{
    (n*(n+1))>>1
}

#[aoc(day17, part1)]
pub fn part1(input: &[u8]) -> i32 {
    let (_xmin,_xmax,ymin,_ymax) = input_nums(input);
    let v = -(ymin+1);
    tri(v)
}

fn sim(x: i32, y:i32, minx:i32, maxx:i32, miny:i32, maxy:i32) -> i32 {
    let mut xvel  = x;
    let mut yvel  = y;
    let mut xpos  = 0;
    let mut ypos  = 0;
    
    while xpos <= maxx && ypos >= miny{
        xpos += xvel;
        ypos += yvel;
        if xpos >= minx && xpos <= maxx && ypos >= miny && ypos <= maxy{
            return 1;
        }
        if xvel > 0 {xvel -=1;}
        yvel -=1;
    }
    0
}

#[aoc(day17, part2)]
fn part2(input:&[u8]) -> i32{
    let (xmin,xmax,ymin,ymax) = input_nums(input);
    let mut n = 0;
    for x in 0..=xmax+1{
        for y in ymin-1..=(-ymin){
            n += sim(x,y,xmin,xmax,ymin,ymax);
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1(){
        assert_eq!(part1(
b"target area: x=20..30, y=-10..-5"
        ), 45);
    }

    #[test]
    fn test2(){
        assert_eq!(part2(
b"target area: x=20..30, y=-10..-5"
        ), 112);
    }
}

