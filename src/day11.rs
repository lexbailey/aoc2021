use std::str;
use std::ops::Add;

fn build_grid(input: &[u8]) -> (Vec<Vec<i64>>, usize, usize){
    let lines: Vec<&[u8]> = str::from_utf8(input).unwrap().trim().lines().map(str::as_bytes).collect();
    let h = lines.len();
    let w = lines[0].len();
    let mut grid : Vec<Vec<i64>> = Vec::with_capacity(h);
    for line in lines{
        grid.push(Vec::from_iter(line.into_iter().map(|c|(*c-48)as i64)));
    }
    (grid, w, h)
}

fn sim_step(g: &mut Vec<Vec<i64>>, w:usize, h:usize) -> i64{
    // Increment
    for y in 0..h{
        for x in 0..w{
            g[y][x] += 1;
        }
    }
    // Flash
    let mut n_flash = 0;
    let mut last_n_flash = -1;
    while last_n_flash != n_flash{
        last_n_flash = n_flash;
        for y in 0..h{
            for x in 0..w{
                if g[y][x] > 9 {
                    n_flash += 1;
                    g[y][x] = -2;
                    for (dx, dy) in [
                        (0,0+1)
                        ,(0+1,0)
                        ,(0+1,0+1)
                        ,(0,0-1)
                        ,(0-1,0)
                        ,(0-1,0-1)
                        ,(0-1,0+1)
                        ,(0+1,0-1)
                    ]{
                        let nx = (x as i64)+dx;
                        let ny = (y as i64)+dy;
                        if nx>=0 && ny>=0 && nx<w as i64 && ny<h as i64{
                            let ux = nx as usize;
                            let uy = ny as usize;
                            g[uy][ux] += 1;
                            if g[uy][ux] < 0 {g[uy][ux] = -2;}
                        }
                    }
                }
            }
        }
    }
    for y in 0..h{
        for x in 0..w{
            if g[y][x] <0 {g[y][x] = 0;}
        }
    }
    n_flash
}

#[aoc(day11, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let (mut grid, w, h) = build_grid(input);
    (0..100).map(|_|
        sim_step(&mut grid, w, h)
    ).reduce(i64::add).unwrap()
}

#[aoc(day11, part2)]
pub fn part2(input: &[u8]) -> i64 {
    let (mut grid, w, h) = build_grid(input);
    let mut n_steps = 1;
    let max_flash = (w*h) as i64;
    while sim_step(&mut grid, w, h) < max_flash {
        n_steps += 1;
    }
    n_steps
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1_1(){
        assert_eq!(part1(
            b"5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526\n"), 1656);
    }

    #[test]
    fn test2_1(){
        assert_eq!(part2(
            b"5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526\n"), 195);
    }
}

