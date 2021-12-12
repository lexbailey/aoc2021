use std::str;
use itertools::Itertools;

fn neighbours(x:i64, y:i64, w:i64, h:i64) -> impl Iterator<Item=(i64,i64)>{
    const UP: (i64,i64) = (0,-1);
    const DOWN: (i64,i64) = (0,1);
    const LEFT: (i64,i64) = (-1,0);
    const RIGHT: (i64,i64) = (1,0);

    const DIRS: [(i64,i64);4] = [UP, DOWN, LEFT, RIGHT];

    DIRS.iter().map(
        move |(dx,dy)|
        (x+dx,y+dy)
    ).filter(
        move |(x,y)|
        (x>=&0) && (y>=&0) && (x<&w) && (y<&h)
    )
}


#[aoc(day9, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let lines: Vec<&[u8]> = str::from_utf8(input).unwrap().trim().lines().map(|s| s.as_bytes()).collect();
    let h = lines.len();
    let w = lines[0].len();

    let mut mins: Vec<u8> = vec!();
    
    for (x,y) in (0..w).cartesian_product(0..h){
        let c = lines[y][x]-48;
        let mut min = true;
        for (nx,ny) in neighbours(x as i64,y as i64,w as i64,h as i64){
            let nc = lines[ny as usize][nx as usize]-48;
            min &= c < nc;
        }
        if min{mins.push(c)}
    }
   
    mins.iter().fold(0_i64, |a,b| a + *b as i64) + mins.len() as i64

}

fn basin_size(grid: &mut Vec<Vec<u8>>, x: usize, y: usize, w: usize, h: usize) -> i64{
    const MASK:u8 = !0x20;
    let c: u8 = grid[y][x];
    grid[y][x] &= MASK;
    let mut size = 1;
    for (nx,ny) in neighbours(x as i64, y as i64, w as i64, h as i64){
        let t = grid[ny as usize][nx as usize];
        if (t>c) && (t != 57){
            size += basin_size(grid, nx as usize, ny as usize, w, h);
        }
    }
    size
}

#[aoc(day9, part2)]
pub fn part2(input: &[u8]) -> i64 {
    let lines: Vec<&[u8]> = str::from_utf8(input).unwrap().trim().lines().map(|s| s.as_bytes()).collect();
    let w = lines[0].len();
    let h = lines.len();

    // Make a mutable copy of the grid
    let mut mlines: Vec<Vec<u8>> = Vec::with_capacity(h);
    for l in lines.iter(){
        mlines.push(Vec::from(*l));
    }

    let mut basins: Vec<i64> = vec!();
    
    for (x,y) in (0..w).cartesian_product(0..h){
        let c = lines[y][x]-48;
        let mut min = true;
        for (nx,ny) in neighbours(x as i64, y as i64, w as i64, h as i64){
            let nc = lines[ny as usize][nx as usize]-48;
            min &= c < nc;
        }
        if min{
            // Calculate basin size for x,y
            basins.push(basin_size(&mut mlines, x, y, w, h));
        }
    }

    basins.sort_unstable();
    basins.iter().rev().take(3).fold(1_i64, |a,b| a * *b as i64)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1(){
        assert_eq!(part1(
            b"2199943210\n3987894921\n9856789892\n8767896789\n9899965678\n"), 15);
    }

    #[test]
    fn test2(){
        assert_eq!(part2(
            b"2199943210\n3987894921\n9856789892\n8767896789\n9899965678\n"), 1134);
    }

}
