use std::str;

fn parse(input: &[u8]) -> (&str, Vec<((u8,u8),[(u8,u8);2])>) {
    let mut lines = str::from_utf8(input).unwrap().lines();
    let template = lines.next().unwrap();
    assert_eq!(lines.next(), Some(""));
    let rules: Vec<((u8,u8),[(u8,u8);2])> = lines.map(|line|{
        let r = line.split(" -> ").collect::<Vec<&str>>();
        let i = r[0].as_bytes();
        let n = r[1].as_bytes()[0];
        ((i[0], i[1])
        ,[(i[0],n),(n,i[1])])
    }).collect();
    (template, rules)
}

fn reduce_buckets(b: &Vec<((u8,u8), i64)>) -> Vec<((u8,u8),i64)>{
    let mut result: Vec<((u8,u8),i64)> = vec![];
    let mut bucket_keys: Vec<(u8,u8)> = b.iter().map(|(k,_v)|*k).collect();
    bucket_keys.sort_unstable();
    bucket_keys.dedup();
    for k in bucket_keys{
        result.push((k,b.iter().filter(|(f,_n)|*f==k).map(|(_f,n)|*n).sum()))
    }
    result
}

fn simulate_buckets(b: &mut Vec<((u8,u8),i64)>, rules: &Vec<((u8,u8),[(u8,u8);2])>){
    for i in 0..b.len(){
        let (pair,n) = b[i];
        for (from, tos) in rules{
            if pair == *from{
                for to in tos{
                    b.push((*to,n));
                }
                b.push((*from,-n));
                break;
            }
        }
    }
}

fn simulate(input: &[u8], steps: i64) -> i64 {
    let (template, rules) = parse(input);
    let mut template1 = Vec::new();
    template1.extend_from_slice(template.as_bytes());
    // Non-double-counted chars
    let ndc1 = template1[0];
    let ndc2 = template1[template1.len()-1];
    let mut template = template1.into_iter();
    let mut prev = template.next().unwrap();
    let mut pairs: Vec<((u8, u8), i64)> = Vec::new();
    for cur in template{
        let pair = (prev,cur);
        pairs.push((pair, 1));
        prev = cur;
    }
    pairs = reduce_buckets(&pairs);
    for _ in 0..steps{
        simulate_buckets(&mut pairs, &rules);
        pairs = reduce_buckets(&pairs);
    }
    let mut counts: [i64;256] = [-1;256];
    counts[ndc1 as usize] += 1;
    counts[ndc2 as usize] += 1;
    for ((c1,c2),n) in pairs{
        counts[c1 as usize] += n;
        counts[c2 as usize] += n;
    }
    let mut nz_counts: Vec<(u8, i64)> = Vec::new();
    for (c, count) in counts.into_iter().enumerate(){
        if count>=0 {
            nz_counts.push((c as u8, (count+1)>>1));
        }
    }
    let mut largest = 0;
    let mut smallest = std::i64::MAX;
    for (_c, count) in nz_counts{
        if count > largest{
            largest = count;
        }
        if count < smallest{
            smallest = count;
        }
    }
    largest - smallest
}

#[aoc(day14, part1)]
pub fn part1(input: &[u8]) -> i64 {
    simulate(input, 10)
}

#[aoc(day14, part2)]
pub fn part2(input: &[u8]) -> i64 {
    simulate(input, 40)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test0(){
        assert_eq!(part1(
b"NNNCB

AA -> B"
        ), 2);
    }

    #[test]
    fn test1(){
        assert_eq!(part1(
b"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"
        ), 1588);
    }

    #[test]
    fn test2(){
        assert_eq!(part2(
b"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
"
            ),2188189693529
        )
    }
}



