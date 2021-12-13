use std::str;
use std::cmp::max;

fn bingo(card: &Vec<Vec<i32>>, num: i32) -> i32 {
    let s: i32 = card.iter().map(|line|line.iter().map(|n|max(*n,0)).sum::<i32>()).sum();
    s*num
}

#[aoc(day4, part1)]
pub fn part1(input: &[u8]) -> i32 {
    let mut l = str::from_utf8(input).unwrap().trim().split("\n\n");
    let nums = l.next().unwrap().split(",").map(str::parse::<i32>).map(Result::unwrap);
    let mut cards: Vec<Vec<Vec<i32>>> = vec![];
    for card_data in l{
        let mut card: Vec<Vec<i32>> = Vec::with_capacity(5);
        for line in card_data.lines(){
            let line: Vec<i32> = line.split(" ").filter(|s|s.trim()!="").map(str::parse::<i32>).map(Result::unwrap).collect();
            card.push(line);
        }
        cards.push(card);
    } 
    for num in nums{
        for card in cards.iter_mut(){
            for x in 0..5{
                for y in 0..5{
                    if card[x][y] == num{
                        card[x][y] = -1;
                    }
                }
            }
            for xy in 0..5{
                if (0..5).all(|y|card[xy][y] < 0) || (0..5).all(|x|card[x][xy]<0){
                    return bingo(card, num);
                }
            }
        }
    }
    0
}

#[aoc(day4, part2)]
pub fn part2(input: &[u8]) -> i32 {
    let mut l = str::from_utf8(input).unwrap().trim().split("\n\n");
    let nums = l.next().unwrap().split(",").map(str::parse::<i32>).map(Result::unwrap);
    let mut cards: Vec<Vec<Vec<i32>>> = vec![];
    for card_data in l{
        let mut card: Vec<Vec<i32>> = Vec::with_capacity(5);
        for line in card_data.lines(){
            let line: Vec<i32> = line.split(" ").filter(|s|s.trim()!="").map(str::parse::<i32>).map(Result::unwrap).collect();
            card.push(line);
        }
        cards.push(card);
    } 
    let n_cards = cards.len();
    let mut n_bingos = 0;
    for num in nums{
        for card in cards.iter_mut(){
            for x in 0..5{
                for y in 0..5{
                    if card[x][y] == num{
                        card[x][y] = -1;
                    }
                }
            }
            for xy in 0..5{
                if (0..5).all(|y|card[xy][y] < 0) || (0..5).all(|x|card[x][xy]<0){
                    n_bingos += 1;
                    if n_bingos == n_cards{
                        return bingo(card, num);
                    }
                    else{
                        for xy in 0..5 {card[xy][xy] = 99;}
                    }
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1(){
        assert_eq!(part1(
b"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"
        ), 4512);

    }

    #[test]
    fn test2(){
        assert_eq!(part2(
b"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"
        ), 1924);

    }

}

