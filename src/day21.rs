use std::str;
use std::cmp::{min, max};
use std::collections::HashMap;

fn starts(input: &[u8]) -> (i64, i64){
    let ns: Vec<i64> = str::from_utf8(input).unwrap().lines().map(|l|{
            let mut a = l.split(": ");
            a.next();
            a.next().unwrap().parse::<i64>().unwrap()
        }
    ).collect();
    (ns[0],ns[1])
}

struct DetDie{
    p: i64
}

impl Iterator for DetDie{
    type Item=i64;
    fn next(&mut self) -> Option<Self::Item>{
        self.p += 1;
        if self.p > 100 {self.p = 1;}
        Some(self.p)
    }
}

struct Roller<'a>{
    die: &'a mut dyn Iterator<Item=i64>
    ,n_rolls: i64
}

impl Iterator for Roller<'_>{
    type Item=i64;
    fn next(&mut self) -> Option<Self::Item> {
        Some((0..self.n_rolls).map(|_|self.die.next().unwrap()).sum())
    }
}

#[aoc(day21, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let (mut p1, mut p2) = starts(input);
    p1 -=1;
    p2 -=1;

    let mut p1score = 0;
    let mut p2score = 0;

    let mut cur_p = 1;
    let mut total_rolls = 0;

    let mut die = DetDie{p:0};
    let roller = Roller{die:&mut die, n_rolls:3};
    for roll in roller{
        if cur_p == 1 {
            p1 = (p1 + roll) % 10;
            p1score += p1+1;
            cur_p = 2;
        }
        else{
            p2 = (p2 + roll) % 10;
            p2score += p2+1;
            cur_p = 1;
        }
        total_rolls += 3;
        if p1score >= 1000 || p2score >= 1000{
            break;
        }
    }
    min(p1score, p2score) * total_rolls
}

// Pairs of (number rolled, number of times it will roll)
const QDIE: [(i64,i64);7] = [
        (3,1)
        ,(4,3)
        ,(5,6)
        ,(6,7)
        ,(7,6)
        ,(8,3)
        ,(9,1)
    ];

#[derive(Eq, PartialEq, Hash, Debug)]
struct GameState{
    p1pos: i64
    ,p2pos: i64
    ,p1score: i64
    ,p2score: i64
    ,nextplayer: i64
    ,winner: Option<i64>
}

fn next_state(s: &GameState, roll: i64) -> GameState{
    if s.nextplayer == 1{
        let pos = (s.p1pos + roll) % 10;
        let newscore = s.p1score + pos + 1;
        GameState{
            p1pos: pos
            ,p2pos: s.p2pos
            ,p1score: newscore
            ,p2score: s.p2score
            ,nextplayer: 2
            ,winner: if newscore >= 21 {Some(1)} else {None}
        }
    }
    else {
        let pos = (s.p2pos + roll) % 10;
        let newscore = s.p2score + pos + 1;
        GameState{
            p1pos: s.p1pos
            ,p2pos: pos
            ,p1score: s.p1score
            ,p2score: newscore
            ,nextplayer: 1
            ,winner: if newscore >= 21 {Some(2)} else {None}
        }
    }
}

#[aoc(day21, part2)]
pub fn part2(input: &[u8]) -> i64 {
    let (mut p1, mut p2) = starts(input);
    p1 -=1;
    p2 -=1;
    let mut universes = HashMap::<GameState, i64>::new();
    // Start with one universe where p1 and p2 start in the given position
    universes.insert(
        GameState{p1pos:p1, p2pos: p2, p1score:0, p2score:0, nextplayer:1, winner:None}
    , 1);
    //simulate all universes
    loop {
        let mut new_universes = HashMap::<GameState,i64>::new();
        let mut done = true;
        for (state, n) in universes{
            if state.winner == None{
                done = false;
                let next_states = QDIE.map(|(roll, mult)|(next_state(&state, roll), mult));
                for (state, num) in next_states{
                    let n_prev = *new_universes.get(&state).unwrap_or(&0);
                    new_universes.insert(state, (n*num)+n_prev);
                }
            }
            else{
                let n_prev = *new_universes.get(&state).unwrap_or(&0);
                new_universes.insert(state, n + n_prev);
            }
        }
        universes = new_universes; 
        if done {break;}
    }

    let mut p1tot = 0;
    let mut p2tot = 0;

    for (state, n) in universes{
        match state.winner{
            None => unreachable!()
            ,Some(1) => {p1tot += n}
            ,Some(2) => {p2tot += n}
            ,Some(_) => unreachable!()
        }
    }
    max(p1tot, p2tot)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1(){
        assert_eq!(part1(
b"Player 1 starting position: 4
Player 2 starting position: 8
"
        ), 739785);
    }
    #[test]
    fn test2(){
        assert_eq!(part2(
b"Player 1 starting position: 4
Player 2 starting position: 8
"
        ), 444356092776315);
    }
}

