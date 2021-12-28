/*
fn debug(g: &Vec<Vec<u8>>){
    for line in g{
        println!("{}", str::from_utf8(line).unwrap());
    }
}
*/

fn parse_grid(input: &[u8]) -> Vec<Vec<u8>> {
    input.split(|a|*a==b'\n').filter(|a|a!=b"").map(|a|a.to_vec()).collect()
}

fn step(g: &Vec<Vec<u8>>) -> (Vec<Vec<u8>>, bool) {
    let mut did_move = false;
    let width = g[0].len();
    let g1: Vec<Vec<u8>> = g.iter().map(|line|{
        let mut newline: Vec<u8> = Vec::with_capacity(width);
        let free: Vec<bool> = line.iter().map(|c|*c==b'.').collect();
        for (i, c) in line.iter().enumerate(){
            let next_free = free[(i+1)%width];
            let move_prev = line[(i+width-1)%width] == b'>';
            newline.push(match c {
                b'v' => b'v' // will not move on first step
                ,b'.' => if move_prev { did_move = true; b'>' } else { b'.' }
                ,b'>' => if next_free { did_move = true; b'.' } else { b'>' }
                ,_ => unreachable!()
            });
        }
        newline
    }).collect();
    let gfree: Vec<Vec<bool>> = g1.iter().map(|line|{
        line.iter().map(|c|*c==b'.').collect()
    }).collect();
    let height = g.len();
    let mut newgrid = Vec::with_capacity(height);
    for (i, line) in g1.iter().enumerate(){
        let next_free = &gfree[(i+1)%height];
        let move_prev: Vec<bool> = g1[(i+height-1)%height].iter().map(|c|*c==b'v').collect();
        let mut newline: Vec<u8> = Vec::with_capacity(width);
        for (j, c) in line.iter().enumerate(){
            let next_free = next_free[j];
            let move_prev = move_prev[j];
            newline.push(match c{
                b'>' => b'>' // will not move on second step
                ,b'.' => if move_prev { did_move = true; b'v' } else { b'.' }
                ,b'v' => if next_free { did_move = true; b'.' } else { b'v' }
                ,_ => unreachable!()
            })
        }
        newgrid.push(newline);
    }
    (newgrid, did_move)
}

#[aoc(day25, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let mut grid = parse_grid(input);
    let mut steps = 1; // the first non-motion step is counted, so start at 1 instead of 0
    loop{
        let step_result = step(&grid);
        grid = step_result.0;
        let did_step = step_result.1;
        if did_step{
            steps += 1;
        }
        else{
            break
        }
    }
    steps
}

#[aoc(day25, part2)]
pub fn part2(input: &[u8]) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, parse_grid, step};

    #[test]
    fn test_step(){
        let g = parse_grid(b"...>>>>>...\n");
        let g1 = parse_grid(b"...>>>>.>..\n");
        let g2 = parse_grid(b"...>>>.>.>.\n");
        let (step1, _) = step(&g);
        let (step2, _) = step(&step1);
        assert_eq!(g1, step1);
        assert_eq!(g2, step2);
    }

    #[test]
    fn test_step_wrap(){
        let g = parse_grid(b"...>\n");
        let g1 = parse_grid(b">...\n");
        let (step1, _) = step(&g);
        assert_eq!(g1, step1);
    }

    #[test]
    fn test_step_full(){
        let g = parse_grid(
b"..........
.>v....v..
.......>..
..........
"
        );
        let g1 = parse_grid(
b"..........
.>........
..v....v>.
..........
"
        );
        let (step1, _) = step(&g);
        assert_eq!(g1, step1);
    }

    #[test]
    fn test1(){
        assert_eq!(part1(
b"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"
        ), 58);
    }

    #[test]
    fn test2(){
        assert_eq!(part2(
            b""), -1);
    }
}



