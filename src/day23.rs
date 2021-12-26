use std::cmp::min;

fn cost(c: u8) -> i64 {
    match c{
        0 => 1
        ,1 => 10
        ,2 => 100
        ,3 => 1000
        ,_ => panic!("unknown type: {}", c)
    }
}

fn path_blocked(c: [u8;7], from: usize, to: usize) -> bool{
    match (from, to){
         (0,0) => c[1]!=9
        ,(0,1) => c[1]!=9||c[2]!=9
        ,(0,2) => c[1]!=9||c[2]!=9||c[3]!=9
        ,(0,3) => c[1]!=9||c[2]!=9||c[3]!=9||c[4]!=9

        ,(1,0) => false
        ,(1,1) => c[2]!=9
        ,(1,2) => c[2]!=9||c[3]!=9
        ,(1,3) => c[2]!=9||c[3]!=9||c[4]!=9

        ,(2,0) => false
        ,(2,1) => false
        ,(2,2) => c[3]!=9
        ,(2,3) => c[3]!=9||c[4]!=9

        ,(3,0) => c[2]!=9
        ,(3,1) => false
        ,(3,2) => false
        ,(3,3) => c[4]!=9

        ,(4,0) => c[3]!=9||c[2]!=9
        ,(4,1) => c[3]!=9
        ,(4,2) => false
        ,(4,3) => false

        ,(5,0) => c[4]!=9||c[3]!=9||c[2]!=9
        ,(5,1) => c[4]!=9||c[3]!=9
        ,(5,2) => c[4]!=9
        ,(5,3) => false

        ,(6,0) => c[5]!=9||c[4]!=9||c[3]!=9||c[2]!=9
        ,(6,1) => c[5]!=9||c[4]!=9||c[3]!=9
        ,(6,2) => c[5]!=9||c[4]!=9
        ,(6,3) => c[5]!=9
        ,_ => panic!("mystery path")
    }
}

fn steps(from: usize, to: usize) -> i64{
    match (from, to){
         (0,0) => 3
        ,(0,1) => 5
        ,(0,2) => 7
        ,(0,3) => 9

        ,(1,0) => 2
        ,(1,1) => 4
        ,(1,2) => 6
        ,(1,3) => 8

        ,(2,0) => 2
        ,(2,1) => 2
        ,(2,2) => 4
        ,(2,3) => 6

        ,(3,0) => 4
        ,(3,1) => 2
        ,(3,2) => 2
        ,(3,3) => 4

        ,(4,0) => 6
        ,(4,1) => 4
        ,(4,2) => 2
        ,(4,3) => 2

        ,(5,0) => 8
        ,(5,1) => 6
        ,(5,2) => 4
        ,(5,3) => 2

        ,(6,0) => 9
        ,(6,1) => 7
        ,(6,2) => 5
        ,(6,3) => 3
        ,_ => panic!("mystery path")
    }
}

fn shortest_path(corridor: [u8;7], room_head: [u8;4], room_tail: [u8;4]) -> Option<i64>{
    assert!(
        (corridor.iter().filter(|c|**c!=9).count() +
        [room_head, room_tail].iter().flatten().filter(|c|**c!=9).count()) == 8
    );
    if corridor == [9;7]{
        if (0..=3).all(|i|{
            room_head[i] as usize == i
            && room_tail[i] as usize == i
        }) {
            return Some(0);
        }
    }
    let mut min_cost: Option<i64> = None;
    let mut did_step = false;
    for c in 0..=6 {
        let p = corridor[c];
        if p != 9{
            let step_cost = cost(p);
            let h = p as usize;
            if room_tail[h] != 9 && room_head[h] != 9 { continue; }
            if room_tail[h] != 9 && room_tail[h] != p { continue; }
            if !path_blocked(corridor, c, h) && room_head[h] == 9{
                let mut new_corridor = corridor;
                new_corridor[c] = 9;
                let mut new_head = room_head;
                let mut new_tail = room_tail;
                let mut distance = steps(c, h);
                if new_tail[h] == 9{
                    new_tail[h] = p;
                    distance += 1;
                }
                else{
                    new_head[h] = p;
                }
                let cost = step_cost * distance;
                did_step = true;
                if let Some(subpath) = shortest_path(new_corridor, new_head, new_tail){
                    let tot_cost = cost + subpath;
                    min_cost = min_cost.and_then(|m|Some(min(m, tot_cost))).or(Some(tot_cost))
                }
            }
        }
    }
    if did_step {return min_cost;}
    // No motion _into_ a room available, move out instead
    for r in 0..=3_u8{
        if (room_head[r as usize] != 9 && room_head[r as usize] != r) || (room_tail[r as usize] != 9 && room_tail[r as usize] != r) {
            let mut p = room_head[r as usize];
            if p == 9{
                p = room_tail[r as usize];
            }
            let p = p;
            let step_cost = cost(p);
            for c in 0..=6{
                if path_blocked(corridor, c, r as usize) { continue; }
                let mut new_corridor = corridor;
                if new_corridor[c as usize] != 9{
                    continue;
                }
                new_corridor[c as usize] = p;
                let mut new_head = room_head;
                let mut new_tail = room_tail;
                let mut distance = steps(c as usize, r as usize);
                if new_head[r as usize] != 9{
                    new_head[r as usize] = 9;
                }
                else{
                    new_tail[r as usize] = 9;
                    distance += 1;
                }
                let cost = step_cost * distance;
                if let Some(subpath) = shortest_path(new_corridor, new_head, new_tail){
                    let tot_cost = cost + subpath;
                    min_cost = min_cost.and_then(|m|Some(min(m, tot_cost))).or(Some(tot_cost))
                }
            }
        }
    }
    min_cost
}

#[aoc(day23, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let corridor = [9;7];
    let mut room_head = [9;4];
    let mut room_tail = [9;4];
    room_head[0] = input[31]-65;
    room_head[1] = input[33]-65;
    room_head[2] = input[35]-65;
    room_head[3] = input[37]-65;

    room_tail[0] = input[45]-65;
    room_tail[1] = input[47]-65;
    room_tail[2] = input[49]-65;
    room_tail[3] = input[51]-65;

    if let Some(a) = shortest_path(corridor, room_head, room_tail) {
        a
    }
    else{
        -1
    }
}

#[aoc(day23, part2)]
pub fn part2(input: &[u8]) -> i64 {
    -1
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1(){
        assert_eq!(part1(
b"#############
#...........#
###B#A#C#D###
  #A#B#C#D#
  #########
"
        ), 46);
    }

    #[test]
    fn test2(){
        assert_eq!(part1(
b"#############
#...........#
###B#A#C#D###
  #B#A#C#D#
  #########
"
        ), 114);
    }

    #[test]
    fn test3(){
        assert_eq!(part1(
b"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
"
        ), 12521);
    }
/*
    #[test]
    fn test2(){
        assert_eq!(part1(
            b"\ndc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc\n"), 19);
    }
*/
}



