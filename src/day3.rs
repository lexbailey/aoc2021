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

#[aoc(day3, part2)]
pub fn part2(input: &[u8]) -> i64 {
    let mut a:Vec<&str> = str::from_utf8(input).unwrap().trim().lines().collect();
    let mut b:Vec<&str> = a.clone();
    let width = a[0].len();
    for n in 0..width {
        let mut sum_a = 0;
        let mut sum_b = 0;
        
        for l in &a {
            let l = l.as_bytes();
            sum_a += if l[n as usize] == '0' as u8 {-1} else {1};
        }
        for l in &b {
            let l = l.as_bytes();
            sum_b += if l[n as usize] == '0' as u8 {-1} else {1};
        }
        if a.len() > 1{
            let keep_a = if sum_a >= 0 {'1'}else{'0'};
            a = a.into_iter().filter(|s|s.as_bytes()[n]==keep_a as u8).collect();
        }
        if b.len() > 1{
            let keep_b = if sum_b >= 0 {'0'}else{'1'};
            b = b.into_iter().filter(|s|s.as_bytes()[n]==keep_b as u8).collect();
        }
    }
    let oxy = i64::from_str_radix(a[0], 2).unwrap();
    let co2 = i64::from_str_radix(b[0], 2).unwrap();
    print!("oxy: {}, co2: {}\n",oxy, co2);
    oxy * co2
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
