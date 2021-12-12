use std::str;

#[aoc(day3, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let a = str::from_utf8(input).unwrap().trim().lines();
    let mut first = true;
    let mut width = 0;
    let mut sums: Vec<i64> = vec![0;0];
    for l in a {
        if first{
            first = false;
            width = l.len();
            sums = vec![0; width];
        }
        for n in 0..width {
            let l = l.as_bytes();
            sums[n] += if l[n as usize] == '0' as u8 {-1} else {1};
        }
    }
    let gamma: String = sums.into_iter().map(|x| if x<=0{'0'}else{'1'}).collect();
    let gamma_v = i64::from_str_radix(&gamma, 2).unwrap();
    let sigma: String = gamma.chars().map(|x| if x=='0'{'1'}else{'0'}).collect();
    let sigma_v = i64::from_str_radix(&sigma, 2).unwrap();
    return gamma_v * sigma_v;
}

fn filter_lines(lines: &[String], bit: usize, mode: i64) -> Box<[String]>{
    let mut first = true;
    let mut width = 0;
    let mut sums: Vec<i64> = vec![0;0];
    for l in lines.into_iter() {
        if first{
            first = false;
            width = l.len();
            sums = vec![0; width];
        }
        for n in 0..width {
            let l = l.as_bytes();
            sums[n] += if l[n as usize] == '0' as u8 {-1} else {1};
            print!("{} ", sums[n]);
        }
        print!("\n");
    }
    let gamma: String = sums.into_iter().map(|x| if x<0{'0'}else{'1'}).collect();
    let sigma: String = gamma.chars().map(|x| if x=='0'{'1'}else{'0'}).collect();
    let value = if mode==0{gamma}else{sigma};
    //let result = lines.filter(|a| a[bit] == value[bit]);
    //value[bit]
    Box::new([])
}

#[aoc(day3, part2)]
pub fn part2(input: &[u8]) -> i64 {
    let a = str::from_utf8(input).unwrap().trim().lines();
    let mut first = true;
    let mut width = 0;
    let mut sums: Vec<i64> = vec![0;0];
    for l in a {
        if first{
            first = false;
            width = l.len();
            sums = vec![0; width];
        }
        for n in 0..width {
            let l = l.as_bytes();
            sums[n] += if l[n as usize] == '0' as u8 {-1} else {1};
            print!("{} ", sums[n]);
        }
        print!("\n");
    }
    return 2;
}


#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1(){
        assert_eq!(part1(b"00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"), 198)
    }

    #[test]
    fn test2(){
        assert_eq!(part2(b"00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"), 230)
    }
}
