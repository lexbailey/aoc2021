use std::str;
use std::ops::Add;

fn both_parts(input: &[u8]) -> (i64, i64) {
    let s = str::from_utf8(input).unwrap().trim().lines();
    const WN_SZ: usize = 3;
    let wn_len: i64 = WN_SZ.try_into().unwrap();
    let mut last: Option<i64> = None;
    let mut last_wind_sum: Option<i64> = None;
    let mut incs: i64 = 0;
    let mut window: [i64; WN_SZ] = [0,0,0];
    let mut window_ptr = 0;
    let mut sw_incs: i64 = 0;
    let mut i: i64 = 0;
    for n in s {
        match n.parse::<i64>() {
            Ok(a) => {
                    if let Some(l) = last {
                        if a > l { incs += 1; }
                    }
                    last = Some(a);
                    window[window_ptr] = a;
                    window_ptr += 1;
                    window_ptr %= WN_SZ;
                    if i >= wn_len-1 {
                        let sum = Some(window.iter().fold(0, i64::add)).unwrap();
                        if let Some(l) = last_wind_sum {
                            if sum > l { sw_incs += 1; }
                        }
                        last_wind_sum = Some(sum);
                    }
                },
            Err(_e) => (),
        }
        i += 1;
    }
    (incs, sw_incs)
}

#[aoc(day1, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let (p1, _p2) = both_parts(input);
    p1
}
#[aoc(day1, part2)]
pub fn part2(input: &[u8]) -> i64 {
    let (_p1, p2) = both_parts(input);
    p2
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1(){
        assert_eq!(part1(b"199\n200\n208\n210\n200\n207\n240\n269\n260\n263"), 7)
    }

    #[test]
    fn test2(){
        assert_eq!(part2(b"199\n200\n208\n210\n200\n207\n240\n269\n260\n263"), 5)
    }
}
