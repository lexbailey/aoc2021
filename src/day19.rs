use std::str;
use std::fmt;
use std::ops::Index;
use std::cmp::{Eq,PartialEq};
use std::slice::Iter;
use std::cmp::max;
use std::collections::HashSet;

#[derive(Debug,Clone,Copy,Hash)]
struct V3{
    x: i64
    ,y: i64
    ,z: i64
}

impl V3{
    const fn new(x: i64, y: i64, z: i64) -> V3{
        V3{
            x:x,y:y,z:z
        }
    }

    fn from_str(input: &str) -> V3{
        let v: Vec<i64> = input.split(",").map(str::parse::<i64>).map(Result::unwrap).collect();
        V3::new(v[0],v[1],v[2])
    }

    fn sub(&self, other: &Self) -> Self{
        V3::new(
            self.x - other.x
            ,self.y - other.y
            ,self.z - other.z
        )
    }

    fn add(&self, other: &Self) -> Self{
        V3::new(
            self.x + other.x
            ,self.y + other.y
            ,self.z + other.z
        )
    }
}

impl PartialEq for V3{
    fn eq(&self, other: &Self) -> bool {
        (0..3).map(|i| self[i] == other[i]).all(|b|b)
    }
}

impl Eq for V3{}

impl FromIterator<i64> for V3{
    fn from_iter<I: IntoIterator<Item=i64>>(i: I) -> Self{
        let mut i = i.into_iter();
        V3::new(i.next().unwrap(),i.next().unwrap(),i.next().unwrap())
    }
}

impl FromIterator<V3> for [V3; 3]{
    fn from_iter<I: IntoIterator<Item=V3>>(i: I) -> Self{
        let mut i = i.into_iter();
        [i.next().unwrap(),i.next().unwrap(),i.next().unwrap(),]
    }
}

impl Index<usize> for V3{
    type Output=i64;
    fn index(&self, index: usize) -> &i64{
        match index {
            0 => &self.x
            ,1 => &self.y
            ,2 => &self.z
            ,_ => panic!("index out of bounds")
        }
    }
}

impl fmt::Display for V3{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

// FmMat is a formattable matrix, because something something orphans???
struct FmMat<'a>{v:&'a[V3;3]}

impl fmt::Display for FmMat<'_>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "[{}\n {}\n {}]", self.v[0], self.v[1], self.v[2])
    }
}


fn transform(v: &V3, m:[V3;3]) -> V3{
    V3::new(
        (0..3).map(|i|v[i]*m[0][i]).sum()
        ,(0..3).map(|i|v[i]*m[1][i]).sum()
        ,(0..3).map(|i|v[i]*m[2][i]).sum()
    )
}


const IDENTITY: [V3;3]= [
          V3::new(1,0,0)
          ,V3::new(0,1,0)
          ,V3::new(0,0,1)
      ];

const RX90: [V3;3]= [
          V3::new(1,0,0)
          ,V3::new(0,0,-1)
          ,V3::new(0,1,0)
      ];

const RY90: [V3;3]= [
          V3::new(0,0,1)
          ,V3::new(0,1,0)
          ,V3::new(-1,0,0)
      ];

fn matmul(a: &[V3;3], b:&[V3;3]) -> [V3;3]{
    (0..3).map(|i|
        V3::from_iter((0..3).map(|j|
            (0..3).map(|k|
                a[i][k]*b[k][j]
            ).sum()
        ))
    ).collect()
}

fn twice(a:&[V3;3])->[V3;3]{matmul(a,a)}
fn thrice(a:&[V3;3])->[V3;3]{matmul(a, &twice(a))}

fn rotations() -> Vec<[V3; 3]>{
    let rots6 = [IDENTITY, RY90, twice(&RY90), matmul(&RX90, &RY90), matmul(&twice(&RX90), &RY90), matmul(&thrice(&RX90), &RY90)];
    let rots4 = [IDENTITY, RX90, twice(&RX90), thrice(&RX90)];
    rots6.map(|r3|
        rots4.map(|r4|
            matmul(&r3, &r4)
        )
    ).concat()
}

#[derive(Clone)]
struct Scanner{
    beacons: Vec<V3>
    ,transform: [V3;3]
    ,translate: V3 // TODO make transform bigger and make it include the translation?
    ,located: bool
}

struct BeaconIter<'a>{
    scanner: &'a Scanner
    ,beacons: Iter<'a,V3>
}

impl Scanner{
    fn beacon_iter(&self) -> BeaconIter{
        BeaconIter{scanner:&self, beacons: self.beacons.iter()}
    }

    //fn transform(&self, t: [V3;3]) -> Scanner{
    //    Scanner{beacons:self.beacons.clone(), transform:t}
    //}
}

impl Iterator for BeaconIter<'_>{
    type Item = V3;
    fn next(&mut self) -> Option<Self::Item> {
        match self.beacons.next() {
            None => None
            ,Some(v)=>Some(transform(v, self.scanner.transform).add(&self.scanner.translate))
        }
    }
}

fn parse_scanners(input: &[u8]) -> Vec<Scanner>{
    let scanners = str::from_utf8(input).unwrap().trim().split("\n\n").map(|s|s.lines());
    let mut result: Vec<Scanner> = Vec::new();
    for mut scanner in scanners{
        let mut s: Vec<V3> = Vec::new();
        scanner.next();
        for line in scanner{
            s.push(V3::from_str(line));
        }
        result.push(Scanner{beacons:s, transform:IDENTITY, translate:V3::new(0,0,0), located:false});
    }
    result
}

// number of beacons that overlap in these two scanners
fn max_overlap(a: &Scanner, b: &Scanner) -> (i64, V3){
    let mut max_o = 0;
    let mut max_delta = V3::new(0,0,0);
    for abeacon1 in a.beacon_iter().take(a.beacons.len()-11){
        for bbeacon1 in b.beacon_iter().take(b.beacons.len()-11){
            let delta = abeacon1.sub(&bbeacon1);
            let mut n: i64 = 0;
            for abeacon2 in a.beacon_iter(){
                for bbeacon2 in b.beacon_iter(){
                    let d2 = abeacon2.sub(&bbeacon2);
                    if d2 == delta{
                        n += 1;
                    }
                }
            }
            if n > max_o{
                max_o = n;
                max_delta = delta;
            }
        }
    }
    (max_o, max_delta)
}

fn mdist(a:&V3, b:&V3) -> i64{
    let d = b.sub(a);
    (0..3).map(|i|d[i].abs()).sum()
}

pub fn both_parts(input: &[u8]) -> (i64, i64) {
    let mut scanners = parse_scanners(input);

    scanners[0].located = true; // first scanner is coordinate base

    while !scanners.iter().all(|s|s.located){
        for i in 0..scanners.len(){
            for j in 0..scanners.len(){
                if j<=i {continue}
                // no information to gain by comparing located scanners
                if scanners[i].located == scanners[j].located {continue}
                let l = if scanners[i].located {i} else if scanners[j].located {j} else {unreachable!()};
                let u = if l==i{j} else{i};
                for rot in rotations(){
                    scanners[u].transform = rot;
                    let (overlap, delta) = max_overlap(&scanners[l], &scanners[u]);
                    if overlap >= 12{
                        scanners[u].translate = delta;
                        scanners[u].located = true;
                        break;
                    }
                }
            }
        }
    }

    let mut beacons: HashSet<V3> = HashSet::new();
    for scanner in &scanners{
        for beacon in scanner.beacon_iter(){
            beacons.insert(beacon);
        }
    }
    
    let n_beacons = beacons.len() as i64;

    let mut max_dist = 0;
    for i in 0..scanners.len(){
        for j in 0..scanners.len(){
            if j<=i {continue}
            let dist = mdist(&scanners[i].translate, &scanners[j].translate);
            max_dist = max(max_dist, dist);
        }
    }
    
    (n_beacons, max_dist)
}

#[aoc(day19, part1)]
pub fn part1(input: &[u8]) -> i64 {
    both_parts(input).0
}

#[aoc(day19, part2)]
pub fn part2(input: &[u8]) -> i64 {
    both_parts(input).1
}

#[cfg(test)]
mod tests {
    use super::{part1, both_parts, matmul, RX90, RY90, transform, V3, rotations};
    use std::collections::HashSet;
    use std::collections::hash_map::RandomState;

    const RX90Y90: [V3;3] = [
        V3::new(0,0,1)
        ,V3::new(1,0,0)
        ,V3::new(0,1,0)
    ];

    #[test]
    fn test_matmul(){
        let a = matmul(&RX90, &RY90);
        let b = RX90Y90;
        assert_eq!(a, b);
    }

    #[test]
    fn test_allrots(){
        let vector = V3::new(1,2,3);
        let rots: Vec<V3> = rotations().iter().map(|rot|{
            transform(&vector, *rot)
        }).collect();
        assert_eq!(rots.len(), 24);
        let rot_set: HashSet<V3,RandomState> = HashSet::from_iter(rots.into_iter());
        assert_eq!(rot_set.len(), 24);
    }

    const TEST_INPUT: &[u8] = 
b"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
"
    ;

    #[test]
    fn test1and2(){
        assert_eq!(both_parts(TEST_INPUT), (79, 3621));
    }

    const TEST_INPUT_SMALLER: &[u8] = 
b"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390
"
    ;

    #[test]
    fn test0(){
        assert_eq!(part1(TEST_INPUT_SMALLER), 38);
    }
}

