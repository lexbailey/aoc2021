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

fn shortest_path(corridor: [u8;7], rooms: &mut[&mut Vec<u8>;4], room_size: usize) -> Option<i64>{
    assert!(
        (corridor.iter().filter(|c|**c!=9).count() +
        rooms.iter().map(|room|room.iter().filter(|c|**c!=9).count()).sum::<usize>()) == room_size*4
    );
    if (0..=3).all(|i|{
        rooms[i].len()==room_size &&
        rooms[i].iter().all(|c|*c as usize==i)
    }) {
        return Some(0);
    }
    let mut min_cost: Option<i64> = None;
    let mut did_step = false;
    // try to step into a room
    for c in 0..=6 {
        let p = corridor[c];
        if p != 9{
            let h = p as usize;
            let n_inside = rooms[h].len();
            if n_inside >= room_size { continue; }
            if rooms[h].iter().any(|c|*c as usize !=h) { continue; }
            if !path_blocked(corridor, c, h){
                let mut new_corridor = corridor;
                new_corridor[c] = 9;
                rooms[h].push(p);
                let distance = steps(c, h) + ((room_size)-rooms[h].len()) as i64;
                let step_cost = cost(p);
                let cost = step_cost * distance;
                did_step = true;
                if let Some(subpath) = shortest_path(new_corridor, rooms, room_size){
                    let tot_cost = cost + subpath;
                    min_cost = min_cost.and_then(|m|Some(min(m, tot_cost))).or(Some(tot_cost))
                }
                rooms[h].pop();
            }
        }
    }
    if did_step {return min_cost;}
    // No motion _into_ a room available, move out instead
    for r in 0..=3_u8{
        // if room contains any items that need moving out
        if rooms[r as usize].iter().any(|c|*c!=r) {
            for c in 0..=6{
                if corridor[c] != 9 { continue; }
                if path_blocked(corridor, c, r as usize) { continue; }
                let mut new_corridor = corridor;
                let p = rooms[r as usize].pop().unwrap();
                new_corridor[c as usize] = p;
                let distance = steps(c as usize, r as usize) + ((room_size-1)-rooms[r as usize].len())as i64;
                let step_cost = cost(p);
                let cost = step_cost * distance;
                if let Some(subpath) = shortest_path(new_corridor, rooms, room_size){
                    let tot_cost = cost + subpath;
                    min_cost = min_cost.and_then(|m|Some(min(m, tot_cost))).or(Some(tot_cost))
                }
                rooms[r as usize].push(p);
            }
        }
    }
    min_cost
}

#[aoc(day23, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let corridor = [9;7];
    let sz = 2;
    let mut r1: Vec<u8> = Vec::with_capacity(sz);
    let mut r2: Vec<u8> = Vec::with_capacity(sz);
    let mut r3: Vec<u8> = Vec::with_capacity(sz);
    let mut r4: Vec<u8> = Vec::with_capacity(sz);
    r1.push(input[45]-65);
    r2.push(input[47]-65);
    r3.push(input[49]-65);
    r4.push(input[51]-65);

    r1.push(input[31]-65);
    r2.push(input[33]-65);
    r3.push(input[35]-65);
    r4.push(input[37]-65);

    if let Some(a) = shortest_path(corridor, &mut[&mut r1, &mut r2, &mut r3, &mut r4], sz) {
        a
    }
    else{
        -1
    }
}

#[aoc(day23, part2)]
pub fn part2(input: &[u8]) -> i64 {
    let corridor = [9;7];
    let sz = 4;
    let mut r1: Vec<u8> = Vec::with_capacity(sz);
    let mut r2: Vec<u8> = Vec::with_capacity(sz);
    let mut r3: Vec<u8> = Vec::with_capacity(sz);
    let mut r4: Vec<u8> = Vec::with_capacity(sz);
    r1.push(input[45]-65);
    r2.push(input[47]-65);
    r3.push(input[49]-65);
    r4.push(input[51]-65);

    // push the additional patch
    r1.push(3);
    r2.push(1);
    r3.push(0);
    r4.push(2);
    r1.push(3);
    r2.push(2);
    r3.push(1);
    r4.push(0);

    r1.push(input[31]-65);
    r2.push(input[33]-65);
    r3.push(input[35]-65);
    r4.push(input[37]-65);


    if let Some(a) = shortest_path(corridor, &mut[&mut r1, &mut r2, &mut r3, &mut r4], sz) {
        a
    }
    else{
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1_0(){
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
    fn test1_1(){
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
    fn test1_2(){
        assert_eq!(part1(
b"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
"
        ), 12521);
    }

    #[test]
    fn test2(){
        assert_eq!(part2(
b"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
"
        ), 44169);
    }
}



