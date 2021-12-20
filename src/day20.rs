use std::str;

fn parse(input: &[u8]) -> (Vec<u8>, Vec<Vec<u8>>){
    let mut lines = str::from_utf8(input).unwrap().trim().lines();
    let lut_str = lines.next().unwrap();
    let lut: Vec<u8> = lut_str.bytes().map(|c|c&1).collect();
    assert_eq!(lut.len(), 512);
    assert_eq!(lines.next(), Some(""));
    let mut image: Vec<Vec<u8>> = Vec::new();
    for line in lines{
        image.push(Vec::from_iter(line.as_bytes().iter().map(|a|*a&1)));
    }
    (lut, image)
}

const NINE_D: [(i32, i32); 9] = [
     (-1,-1)
    ,(0,-1)
    ,(1,-1)
    ,(-1,0)
    ,(0,0)
    ,(1,0)
    ,(-1,1)
    ,(0,1)
    ,(1,1)
];

fn step_img(img: &Vec<Vec<u8>>, lut: &Vec<u8>, inf_pixel: u8) -> (Vec<Vec<u8>>, u8){
    let in_h = img.len();
    let in_w = img[0].len();
    let out_h = in_h + 2;
    let out_w = in_w + 2;
    let mut result:Vec<Vec<u8>> = Vec::with_capacity(out_h);
    for y in -1..=in_h as i32{
        let mut line = Vec::with_capacity(out_w);
        for x in -1..=in_w as i32{
            let ox = x+1;
            let oy = y+1;
            let mut index: u32 = 0;
            for (dx, dy) in NINE_D{
                let rx = x +dx;
                let ry = y +dy;
                let pixel: u8 = if rx>=0 && ry>=0 && rx<(in_w as i32) && ry<(in_h as i32){
                    img[ry as usize][rx as usize]
                }
                else{
                    inf_pixel
                };
                index = (index<<1)|(pixel as u32);
            }
            line.push(lut[index as usize]);
        }
        result.push(line);
    }
    let new_inf_pixel = if inf_pixel == 0 {lut[0]} else {lut[511]};
    (result, new_inf_pixel)
}

fn enhance(input: &[u8], n: i32) -> i64 {
    let (lut, img) = parse(input);
    let mut inf_pixel: u8 = 0;
    let mut img = img;
    for _ in 0..n{
        let (nimg, ninf_pixel) = step_img(&img, &lut, inf_pixel);
        img = nimg;
        inf_pixel=ninf_pixel;
    }
    img.iter().map(|line|line.iter().map(|a|*a as i64).sum::<i64>()).sum()
}

#[aoc(day20, part1)]
pub fn part1(input: &[u8]) -> i64 {
    enhance(input, 2)
}

#[aoc(day20, part2)]
pub fn part2(input: &[u8]) -> i64 {
    enhance(input, 50)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1(){
        assert_eq!(part1(INPUT_DATA), 35);
    }

    #[test]
    fn test2(){
        assert_eq!(part2(INPUT_DATA), 3351);
    }

    const INPUT_DATA: &[u8] = 
b"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
"
    ;

}

