use std::str;

fn build_grid(input: &[u8]) -> (Vec<Vec<u8>>, usize, usize){
    let lines: Vec<&[u8]> = str::from_utf8(input).unwrap().trim().lines().map(str::as_bytes).collect();
    let h = lines.len();
    let w = lines[0].len();
    let mut grid : Vec<Vec<u8>> = Vec::with_capacity(h);
    for line in lines{
        grid.push(Vec::from_iter(line.into_iter().map(|c|*c-48)));
    }
    (grid, w, h)
}

fn sim_step(g: &mut Vec<Vec<u8>>, w:usize, h:usize){
    // Increment
    for y in 0..h{
        for x in 0..w{
            g[y][x] += 1;
        }
    }
    // Flash
    for y in 0..h{
        for x in 0..w{
            if g[y][x] > 9 {
            }
        }
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let (grid, w, h) = build_grid(input);
    for y in 0..h{
        for x in 0..w{
            print!("{},",grid[y][x])
        }
        print!("\n");
    }
    0
}

#[aoc(day11, part2)]
pub fn part2(input: &[u8]) -> i64 {
    0
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
            b""), -1);
    }
}



