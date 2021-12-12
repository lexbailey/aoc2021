use std::str;

fn score(char: u8) -> i64{
    if char == b')' {3}
    else if char == b']' {57}
    else if char == b'}' {1197}
    else if char == b'>' {25137}
    else {
        panic!();
    }
}

#[aoc(day10, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let lines: Vec<&[u8]> = str::from_utf8(input).unwrap().trim().lines().map(|s| s.as_bytes()).collect();
    let mut total_score = 0;
    const OPEN: &[u8] = b"([{<";
    //const CLOSE: &[u8] = b">}])";
    for line in lines.iter(){
        let mut stack: Vec<u8> = vec!();
        for c in line.iter(){
            if OPEN.contains(c) {
                stack.push(*c);
            }
            else{
                match stack.pop() {
                    Some(o) => {
                        if (*c as i64 - o as i64).abs() > 2 {
                            total_score += score(*c);
                            break;
                        }
                    },
                    None => {},
                }
            }
        }
    }
    total_score
}

fn c_score(char: u8) -> i64{
    if char == b'(' {1}
    else if char == b'[' {2}
    else if char == b'{' {3}
    else if char == b'<' {4}
    else {
        panic!();
    }
}

#[aoc(day10, part2)]
pub fn part2(input: &[u8]) -> i64 {
    let lines: Vec<&[u8]> = str::from_utf8(input).unwrap().trim().lines().map(|s| s.as_bytes()).collect();
    const OPEN: &[u8] = b"([{<";
    //const CLOSE: &[u8] = b">}])";
    let mut line_scores: Vec<i64> = vec!();
    for line in lines.iter(){
        let mut stack: Vec<u8> = vec!();
        let mut valid = true;
        for c in line.iter(){
            if OPEN.contains(c) {
                stack.push(*c);
            }
            else{
                match stack.pop() {
                    Some(o) => {
                        if (*c as i64 - o as i64).abs() > 2 {
                            valid = false; break;
                        }
                    },
                    None => {},
                }
            }
        }
        if valid {
            line_scores.push(stack.iter().rev().map(|c|c_score(*c)).fold(0,|a,b|(a*5)+b));
        }
    }
    line_scores.sort_unstable();
    line_scores[line_scores.len()>>1]
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1(){
        assert_eq!(part1(
            b"[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]"), 26397);
    }

    #[test]
    fn test2(){
        assert_eq!(part2(
            b"[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]"), 288957);
    }

}
