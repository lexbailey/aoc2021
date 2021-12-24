use std::str;
use std::ops::RangeInclusive;
use std::cmp::{min,max};

type Volume = Vec<RangeInclusive<i64>>;

trait RangeLen{
    fn len(&self) -> i64;
}

impl RangeLen for RangeInclusive<i64>{
    fn len(&self) -> i64 {
        let l = (self.end() - self.start()) +1;
        if l < 0 {0} else {l}
    }
}

fn parse_volume(list: &[u8]) -> Volume{
    let xyz = list.split(|a|*a==b',');
    xyz.map(|r|{
        let mut a = r.split(|c|*c==b'=');
        a.next();
        let mut c = a.next().unwrap().split(|c|*c==b'.');
        let start = c.next().unwrap();
        c.next();
        let end = c.next().unwrap();
        let start = str::from_utf8(start).unwrap().parse::<i64>().unwrap();
        let end = str::from_utf8(end).unwrap().parse::<i64>().unwrap();
        start..=end
        }
    ).collect()
}

fn volume(v: &Volume) -> i64 {
    v.iter().map(|r|r.len()).product()
}

fn vol_overlap(va: &Volume, vb: &Volume) -> Volume {
    let o1: Volume = (0..3).map(|i|{
        let a = va[i].start();
        let b = vb[i].end();
        *a..=*b
    }).collect();
    let o2: Volume = (0..3).map(|i|{
        let a = va[i].end();
        let b = vb[i].start();
        *b..=*a
    }).collect();
    let mut result = va;
    let mut min_vol = i64::MAX;
    for v in [&o1, &o2, va, vb]{
        let vol = volume(v);
        if vol > 0 && vol < min_vol{
            min_vol = vol;
            result = v;
        }
    }
    result.clone()
}

fn vol_subtract(va: &Volume, vb: &Volume) -> Vec<Volume> {
    let overlap = vol_overlap(va, vb);
    let overlap_sz = volume(&overlap);
    if overlap_sz == 0{
        return vec![va.clone()];
    }
    let mut results: Vec<Volume> = Vec::with_capacity(7); // 7 is worst case size, trade more ram for fewer resizes.
    // we will end up with up to 7 volumes, the first is the overlap volume
    // results.push(overlap.clone());
    // the next three are the ones that make the volume that covers
    // what is to the left, above, and behind the overlap
    //let full = vec![va[0].start()..=va[0].end(), va[1].start()..va[1].end(), va[2].start()..va[2].end()]
    let v1: Volume = vec![*va[0].start()..=overlap[0].start()-1, va[1].clone(), va[2].clone()];
    let v2: Volume = vec![*overlap[0].start()..=*va[0].end(), *va[1].start()..=overlap[1].start()-1, va[2].clone()];
    let v3: Volume = vec![*overlap[0].start()..=*va[0].end(), *overlap[1].start()..=*va[1].end(), *va[2].start()..=overlap[2].start()-1];
    // The last three are the ones that are to the right, below, and in front of the overlap
    let v4: Volume = vec![*overlap[0].end()+1..=*va[0].end(), *overlap[1].start()..=*va[1].end(), *overlap[2].start()..=*va[2].end()];
    let v5: Volume = vec![overlap[0].clone(), *overlap[1].end()+1..=*va[1].end(), *overlap[2].start()..=*va[2].end()];
    let v6: Volume = vec![overlap[0].clone(), overlap[1].clone(), *overlap[2].end()+1..=*va[2].end()];
    // Some of these volumes might be empty, only add the ones that are not
    for v in [v1,v2,v3,v4,v5,v6]{
        if volume(&v) > 0{
            results.push(v);
        }
    }
    results
}

#[aoc(day22, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let lines = input.split(|a|*a==b'\n').filter(|a|a.len()>0);
    let steps = lines.map(|l|(l[1]==b'n',
        parse_volume(&l[3..])
    ));

    let mut vols: Vec<(bool, Volume)> = Vec::new();

    for (set_state, s) in steps{
        let mut new_vols: Vec<(bool, Volume)> = Vec::with_capacity((vols.len()+1)*2);
        println!("process: {:?}", s);
        for (old_state, vol) in vols{
            println!("  check: {:?}", vol);
            for v in vol_subtract(&vol, &s){
                println!("    push volume: {} {}", old_state, volume(&v));
                new_vols.push((old_state, v));
            }
        }
        if set_state{
            println!("    push new volume: {} {}", set_state, volume(&s));
            new_vols.push((set_state, s));
        }
        vols = new_vols;
        println!("{:?}\n{}", vols, vols.iter().map(|(on,v)|if !on {0} else {volume(v)}).sum::<i64>());
    }
    //println!("{:?}", vols);
    vols.iter().map(|(on,v)|if !on {0} else {volume(v)}).sum()
}
#[aoc(day22, part2)]
pub fn part2(input: &[u8]) -> i64 {
    -1
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, volume, vol_overlap, vol_subtract};
    
    #[test]
    fn test_vol(){
        assert_eq!(
            volume(&vec![0..=2,0..=3,0..=4])
            ,60
        );
    }

    #[test]
    fn test_vol_subtract(){
        let v1 = vec![0..=9, 0..=9, 0..=9];
        let v2 = vec![4..=19, 4..=19, 4..=19];
        let v1_vol = volume(&v1);
        let overlap = vol_overlap(&v1, &v2);
        let overlap_vol = volume(&overlap);
        assert_eq!(overlap_vol, 6*6*6);
        let parts = vol_subtract(&v1, &v2);
        let tot_vol: i64 = overlap_vol + parts.iter().map(volume).sum::<i64>();
        assert_eq!(tot_vol, v1_vol);
    }

    #[test]
    fn test_vol_subtract2(){
        let v1 = vec![4..=19, 4..=19, 4..=19];
        let v2 = vec![0..=9, 0..=9, 0..=9];
        let v1_vol = volume(&v1);
        let overlap = vol_overlap(&v1, &v2);
        let overlap_vol = volume(&overlap);
        assert_eq!(overlap_vol, 6*6*6);
        let parts = vol_subtract(&v1, &v2);
        let tot_vol: i64 = overlap_vol + parts.iter().map(volume).sum::<i64>();
        assert_eq!(tot_vol, v1_vol);
    }

    #[test]
    fn test_vol_subtract3(){
        let v1 = vec![0..=9, 0..=9, 0..=9];
        let v2 = vec![0..=9, 0..=9, 0..=9];
        let v1_vol = volume(&v1);
        let v2_vol = volume(&v2);
        let overlap = vol_overlap(&v1, &v2);
        let overlap_vol = volume(&overlap);
        assert_eq!(overlap_vol, 10*10*10);
        let parts = vol_subtract(&v1, &v2);
        assert_eq!(parts.len(), 0);
        assert_eq!(overlap_vol, v1_vol);
        assert_eq!(overlap_vol, v2_vol);
    }

    #[test]
    fn test_vol_subtract4(){
        let v1 = vec![0..=9, 0..=9, 0..=9];
        let v2 = vec![2..=7, 2..=7, 2..=7];
        let v1_vol = volume(&v1);
        let v2_vol = volume(&v2);
        let overlap = vol_overlap(&v1, &v2);
        let overlap_vol = volume(&overlap);
        let parts = vol_subtract(&v1, &v2);
        assert_eq!(parts.len(), 6);
        let tot_vol: i64 = overlap_vol + parts.iter().map(volume).sum::<i64>();
        assert_eq!(tot_vol, v1_vol);
    }

    #[test]
    fn test_vol_subtract5(){
        let v1 = vec![10..=12, 10..=12, 10..=12];
        let v2 = vec![11..=13, 11..=13, 11..=13];
        let v1_vol = volume(&v1);
        let v2_vol = volume(&v2);
        let overlap = vol_overlap(&v1, &v2);
        let overlap_vol = volume(&overlap);
        let parts = vol_subtract(&v1, &v2);
        assert_eq!(parts.len(), 3);
        let tot_vol: i64 = overlap_vol + parts.iter().map(volume).sum::<i64>();
        assert_eq!(tot_vol, v1_vol);
    }

    #[test]
    fn test_vol_subtract6(){
        let v1 = vec![10..=10, 10..=12, 10..=12];
        let v2 = vec![9..=11, 9..=11, 9..=11];
        let v1_vol = volume(&v1);
        let v2_vol = volume(&v2);
        let overlap = vol_overlap(&v1, &v2);
        let overlap_vol = volume(&overlap);
        let parts = vol_subtract(&v1, &v2);
        println!("{:?}", parts);
        assert_eq!(parts.len(), 5);
        let tot_vol: i64 = overlap_vol + parts.iter().map(volume).sum::<i64>();
        assert_eq!(tot_vol, v1_vol);
    }




    #[test]
    fn test1(){
        assert_eq!(
            part1(
b"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10
"
            )
        , 39);
    }
}

