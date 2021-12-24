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
    (0..3).map(|i|{
        let a_s = *va[i].start();
        let a_e = *va[i].end();
        let b_s = *vb[i].start();
        let b_e = *vb[i].end();
        max(a_s, b_s)..=min(a_e, b_e)
    }).collect()
}

fn vol_subtract(va: &Volume, vb: &Volume) -> Vec<Volume> {
    let overlap = vol_overlap(va, vb);
    let overlap_sz = volume(&overlap);
    if overlap_sz == 0{
        return vec![va.clone()];
    }
    let mut results: Vec<Volume> = Vec::with_capacity(6); // 6 is worst case size, trade more ram for fewer resizes.
    // we will end up with up to 7 volumes, the first is the overlap volume
    // results.push(overlap.clone());
    // the next three are the ones that make the volume that covers
    // what is to the left, above, and behind the overlap
    //let full = vec![va[0].start()..=va[0].end(), va[1].start()..va[1].end(), va[2].start()..va[2].end()]
    let ov = overlap;
    let v1: Volume = vec![*va[0].start()..=ov[0].start()-1, va[1].clone(), va[2].clone()];
    let v2: Volume = vec![*ov[0].start()..=*va[0].end(), *va[1].start()..=ov[1].start()-1, va[2].clone()];
    let v3: Volume = vec![*ov[0].start()..=*va[0].end(), *ov[1].start()..=*va[1].end(), *va[2].start()..=ov[2].start()-1];
    // The last three are the ones that are to the right, below, and in front of the overlap
    let v4: Volume = vec![*ov[0].end()+1..=*va[0].end(), *ov[1].start()..=*va[1].end(), *ov[2].start()..=*va[2].end()];
    let v5: Volume = vec![*ov[0].start()..=min(*ov[0].end(), *va[0].end()), *ov[1].end()+1..=*va[1].end(), *ov[2].start()..=*va[2].end()];
    let v6: Volume = vec![*ov[0].start()..=min(*ov[0].end(), *va[0].end()), *ov[1].start()..=min(*ov[1].end(), *va[1].end()), *ov[2].end()+1..=*va[2].end()];
    // Some of these volumes might be empty, only add the ones that are not
    for v in [v1,v2,v3,v4,v5,v6]{
        if volume(&v) > 0{
            results.push(v);
        }
    }
    results
}

fn compute_volume(input: &[u8], filter: bool) -> i64 {
    let lines = input.split(|a|*a==b'\n').filter(|a|a.len()>0);
    let steps = lines.map(|l|(l[1]==b'n',
        parse_volume(&l[3..])
    ));

    let mut vols: Vec<(bool, Volume)> = Vec::new();

    for (set_state, s) in steps{
        if filter && s.iter().map(|r|r.start().abs() > 50 && r.end().abs() > 50).any(|b|b) {
            continue
        }
        let mut new_vols: Vec<(bool, Volume)> = Vec::with_capacity((vols.len()+1)*2);
        for (old_state, vol) in vols{
            for v in vol_subtract(&vol, &s){
                new_vols.push((old_state, v));
            }
        }
        if set_state{
            new_vols.push((set_state, s));
        }
        vols = new_vols;
    }
    vols.iter().map(|(on,v)|if !on {0} else {volume(v)}).sum()
}

#[aoc(day22, part1)]
pub fn part1(input: &[u8]) -> i64 {
    compute_volume(input, true)
}

#[aoc(day22, part2)]
pub fn part2(input: &[u8]) -> i64 {
    compute_volume(input, false)
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
        let overlap = vol_overlap(&v1, &v2);
        let overlap_vol = volume(&overlap);
        let parts = vol_subtract(&v1, &v2);
        assert_eq!(parts.len(), 2);
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


    #[test]
    fn test1_2(){
        assert_eq!(
            part1(
b"on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682
"
            ), 590784
        );
    }

    #[test]
    fn test2(){
        assert_eq!(part2(
b"on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507
"
            ), 2758514936282235
        );
    }
}

// 1285501151402480 is too high
