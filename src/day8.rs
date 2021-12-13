use std::str;

fn is_1478(s: &str) -> bool{
    let l = s.len();
    l == 2 || l == 3 || l == 4 || l == 7
}

#[aoc(day8, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let pairs: Vec<Vec<&str>> = str::from_utf8(input).unwrap().trim().split("\n").map(|a|a.trim().split("|").map(|s|s.trim()).collect()).collect();
    let digits: Vec<Vec<&str>> = pairs.iter().map(|p|p[1].split(" ").collect()).collect();
    digits.iter().map(|digits|
        digits.iter().map(|digit|
            if is_1478(digit){1}else{0}
        ).sum::<i64>()
    ).sum()
}

#[aoc(day8, part2)]
pub fn part2(input: &[u8]) -> i64 {
    let pairs: Vec<Vec<&str>> = str::from_utf8(input).unwrap().trim().split("\n").map(|a|a.trim().split("|").map(|s|s.trim()).collect()).collect();
    let patterns: Vec<Vec<&str>> = pairs.iter().map(|p|p[0].split(" ").collect()).collect();
    let digits: Vec<Vec<&str>> = pairs.iter().map(|p|p[1].split(" ").collect()).collect();
    let get_pattern = |n| patterns.iter().map(|patterns|{
        let mut v = Vec::<u8>::new();
        v.extend_from_slice(patterns.iter().find(|p|p.len()==n).unwrap().as_bytes());
        v.sort_unstable();
        v
    }).collect();
    let sample_1: Vec<Vec<u8>> = get_pattern(2);
    let sample_4: Vec<Vec<u8>> = get_pattern(4);
    let mut sum = 0;
    for ((digits, sample_1), sample_4) in digits.iter().zip(sample_1).zip(sample_4){
        let mut value = 0;
        for digit in digits{
            let l = digit.len();
            let d = match l {
                2 => 1
                ,3 => 7
                ,4 => 4
                ,7 => 8
                ,5 => {
                    let mut a: Vec<u8> = digit.bytes().filter(|b| sample_1.contains(b)).collect(); a.sort_unstable();
                    let a = a == sample_1;
                    if a {3}else{
                        let mut b: Vec<u8> = digit.bytes().filter(|b| sample_4.contains(b)).collect(); b.sort_unstable();
                        let b = b.len() == 3;
                        if b {5} else{2}
                    }
                }
                ,6 => {
                    let mut c: Vec<u8> = digit.bytes().filter(|b| sample_4.contains(b)).collect(); c.sort_unstable();
                    let c = c == sample_4;
                    if c{9}else{
                        let mut a: Vec<u8> = digit.bytes().filter(|b| sample_1.contains(b)).collect(); a.sort_unstable();
                        let a = a == sample_1;
                        if a{0}else{6}
                    }
                }
                ,_ => -2
            };
            print!("{} ", d);
            value = (value * 10) + d;
        }
        print!(" - {}\n", value);
        sum += value;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1(){
        assert_eq!(part1(
b"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\n"
        ), 26);
    }

    #[test]
    fn test2(){
        assert_eq!(part2(
b"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\n"
        ), 61229);
    }

}

