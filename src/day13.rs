use std::str;
use std::cmp::{min,max};

fn coords_and_folds(input: &[u8]) -> (Vec<(i64,i64)>, Vec<(u8, i64)>){
    let lines: Vec<&str> = str::from_utf8(input).unwrap().trim().lines().collect();
    let mut coords: Vec<(i64,i64)> = Vec::with_capacity(lines.len());
    let iter = lines.into_iter();
    let mut do_folds = false;
    let mut folds: Vec<(u8,i64)> = vec![];
    for line in iter{
        if !do_folds{
            if line == ""{
                do_folds = true;
            }
            else{
                let xy:Vec<&str> = line.split(",").collect();
                coords.push(
                    (xy[0].parse::<i64>().unwrap(),xy[1].parse::<i64>().unwrap())
                );
            }
        }
        else{
            let c_val:Vec<&str> = line.split("=").collect();
            let c = c_val[0].as_bytes();
            let c = c[c.len()-1];
            let val = c_val[1].parse::<i64>().unwrap();
            folds.push((c,val));
        }
    }
    
    (coords, folds)
}

fn reflect((x,y):(i64,i64), dir:u8, val:i64) -> (i64,i64){
    (
        if dir==121{x}else{min(x,val+(val-x))}
    ,
        if dir==120{y}else{min(y,val+(val-y))}
    )
}

fn limits(coords: &Vec<(i64,i64)>) -> (usize, usize) {
    let mut maxx = 0;
    let mut maxy = 0;
    for (x,y) in coords{
        maxx = max(maxx, *x);
        maxy = max(maxy, *y);
    }
    (maxx as usize, maxy as usize)
}

fn render<'a>(coords: &Vec<(i64,i64)>) -> String {
    let (maxx, maxy) = limits(&coords);
    let mut lines: Vec<Vec<u8>> = Vec::with_capacity(maxy+1);
    for _y in 0..=maxy{
        let mut s = Vec::with_capacity(maxx+1);
        for _x in 0..=maxx{
            s.push(32)
        }
        lines.push(s);
    }
    for (x,y) in coords{
        lines[*y as usize][*x as usize] = 35
    }
    let strings: Vec<&str> = lines.iter().map(|s|str::from_utf8(s).unwrap()).collect();
    let mut result: String = String::with_capacity((maxy+1)*(maxx+2));
    for s in strings{
        result.push_str(s);
        result.push_str("\n");
    }
    result
}

#[aoc(day13, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let (mut coords, folds) = coords_and_folds(input);
    for (c,val) in folds{
        coords = coords.iter().map(|xy|reflect(*xy,c,val)).collect();
        break;
    }
    coords.sort_unstable();
    coords.dedup();
    coords.len() as i64
}

#[aoc(day13, part2)]
pub fn part2(input: &[u8]) -> String {
    let (mut coords, folds) = coords_and_folds(input);
    for (c,val) in folds{
        coords = coords.iter().map(|xy|reflect(*xy,c,val)).collect();
    }
    coords.sort_unstable();
    coords.dedup();
    let s = render(&coords);
    print!("{}\n", s);
    s
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1(){
        assert_eq!(part1(
            b"6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\n\nfold along y=7"), 17); }

    #[test]
    fn test2(){
        assert_eq!(part2(
            b"6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\n\nfold along y=7\nfold along x=5"), "#####\n#   #\n#   #\n#   #\n#####\n"); }
}



