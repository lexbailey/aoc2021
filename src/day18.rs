use std::str;
use std::fmt;
use std::string::ToString;
use std::cmp::max;

#[derive(PartialEq, Clone)]
enum SnailNum{
    Single(i64)
    ,Pair(Box<SnailNum>, Box<SnailNum>)
}

impl<'a> SnailNum{
    fn pair(a: &SnailNum, b: &SnailNum) -> SnailNum{
        SnailNum::Pair(Box::new(a.clone()), Box::new(b.clone()))
    }

    fn from_bytes(b: &[u8]) -> SnailNum{
        let c1 = b[0];
        if c1 != b'['{
            return SnailNum::Single(str::from_utf8(b).unwrap().parse::<i64>().unwrap());
        }
        let mut p: usize = 1;
        let mut sep: usize = 0;
        let mut n_open = 1;
        loop {
            let c = b[p];
            match c {
                b'[' => {n_open += 1;}
                ,b']' => {
                    n_open -= 1;
                    if n_open == 0{
                        break;
                    }
                }
                ,b',' if n_open== 1 =>{
                    sep = p;
                }
                ,_ => ()
            }
            p += 1;
        }
        SnailNum::pair(
            &SnailNum::from_bytes(&b[1..sep])
            ,&SnailNum::from_bytes(&b[sep+1..p])
        )
    }
}

impl fmt::Display for SnailNum{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self{
            Single(a) => write!(f, "{}", i64::to_string(a))
            ,Pair(a, b) =>{
                write!(f, "[{},{}]", SnailNum::to_string(a), SnailNum::to_string(b))
            }
        }
    }
}

use SnailNum::{Single,Pair};

fn mag(s: &SnailNum) -> i64{
    match s{
        SnailNum::Single(a) => *a
        ,SnailNum::Pair(a,b) => (mag(a)*3) + (mag(b)*2)
    }
}

struct Exploder{
    left: Option<i64>
    ,right: Option<i64>
}

fn push_right(a: &SnailNum, from_left: i64) -> SnailNum{
    match a{
        Single(a) => Single(a+from_left)
        ,Pair(a,b) => SnailNum::pair(&push_right(a,from_left), b)
    }
}

fn push_left(a: &SnailNum, from_right: i64) -> SnailNum{
    match a{
        Single(a) => Single(a+from_right)
        ,Pair(a,b) => SnailNum::pair(a, &push_left(b,from_right))
    }
}

fn resolve_explosion(a: &SnailNum, b: &SnailNum, depth: i64) -> (SnailNum, Exploder, bool) {
    let (na, exp, done) = explode(a, depth+1);
    if exp.left == None && exp.right == None && !done{
        let (nb, exp, done) = explode(b, depth+1);
        if exp.left == None && exp.right == None{
            (SnailNum::pair(&na,&nb), Exploder{left:None, right:None}, done)
        }
        else{
            match exp{
                Exploder{left:Some(l), right:r} => (SnailNum::pair(&push_left(a, l), &nb), Exploder{left:None, right:r}, true)
                ,_ => (SnailNum::pair(&na,&nb), exp, done)
            }
        }
    }
    else{
        match exp{
            Exploder{left:l, right:Some(r)} => (SnailNum::pair(&na, &push_right(b, r)), Exploder{left:l, right:None}, true)
            ,_ => (SnailNum::pair(&na,b), exp, done)
        }
    }
}

fn explode(num: &SnailNum, depth: i64) -> (SnailNum, Exploder, bool){
    match num{
        SnailNum::Single(_) => (num.clone(), Exploder{left:None, right:None}, false)
        ,SnailNum::Pair(a,b) if depth >= 4 => {
            let a = *a.clone();
            let b = *b.clone();
            match (a,b){
                (SnailNum::Single(a), SnailNum::Single(b)) => (SnailNum::Single(0), Exploder{left:Some(a), right:Some(b)}, true)
                ,_=>panic!("malformed")
            }
        }
        ,SnailNum::Pair(a,b) => {
            resolve_explosion(a,b, depth)
        }
    }
}

fn split(n:&SnailNum, depth: i64) -> SnailNum{
    match n{
        Single(a) if *a >= 10 => {
            SnailNum::pair(&Single(a>>1), &Single((a>>1)+(a&1)))
        }
        ,Single(_a) => n.clone()
        ,Pair(a,b) => {
            let sa = split(a, depth+1);
            if sa != **a {
                SnailNum::pair(&sa, b)
            }
            else{
                SnailNum::pair(&sa, &split(b, depth+1))
            }
        }
    }
}

fn reduce<'a>(a: &'a SnailNum) -> SnailNum{
    let (nn, _, did_explode) = explode(a, 0);
    if did_explode{nn}
    else{
        let la = split(a, 0);
        la
    }
}

fn add(a: &SnailNum, b: &SnailNum) -> SnailNum{
    let mut s1 = SnailNum::pair(a,b);
    loop{
        let s2 = reduce(&s1);
        if s1 == s2 {
            return s2;
        }
        s1 = s2;
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let nums = input.split(|a|*a==b'\n').filter(|a|a!=b"").map(SnailNum::from_bytes);
    mag(&nums.reduce(|a,b|add(&a,&b)).unwrap())
}

#[aoc(day18, part2)]
pub fn part2(input: &[u8]) -> i64 {
    let nums: Vec<SnailNum> = input.split(|a|*a==b'\n').filter(|a|a!=b"").map(SnailNum::from_bytes).collect();
    let mut max_mag: i64 = 0;
    for a in &nums{
        for b in &nums{
            max_mag = max(max_mag, mag(&add(&a,&b)));
        }
    }
    max_mag
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, mag, SnailNum, add};

    #[test]
    fn test_mag(){
        assert_eq!(mag(&SnailNum::from_bytes(b"[[1,2],[[3,4],5]]")), 143);
        assert_eq!(mag(&SnailNum::from_bytes(b"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")), 1384);
        assert_eq!(mag(&SnailNum::from_bytes(b"[[[[1,1],[2,2]],[3,3]],[4,4]]")), 445);
        assert_eq!(mag(&SnailNum::from_bytes(b"[[[[3,0],[5,3]],[4,4]],[5,5]]")), 791);
        assert_eq!(mag(&SnailNum::from_bytes(b"[[[[5,0],[7,4]],[5,5]],[6,6]]")), 1137);
        assert_eq!(mag(&SnailNum::from_bytes(b"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")), 3488);
    }

    #[test]
    fn test_add_1(){
        assert_eq!(
            add(
                &add(
                    &add(
                        &SnailNum::from_bytes(b"[1,1]")
                        ,&SnailNum::from_bytes(b"[2,2]")
                    )
                    ,&SnailNum::from_bytes(b"[3,3]")
                )
                ,&SnailNum::from_bytes(b"[4,4]")
            ).to_string()
            , "[[[[1,1],[2,2]],[3,3]],[4,4]]"
        );
    }

    #[test]
    fn test_add_2(){
        assert_eq!(
            add(
                &add(
                    &add(
                        &add(
                            &SnailNum::from_bytes(b"[1,1]")
                            ,&SnailNum::from_bytes(b"[2,2]")
                        )
                        ,&SnailNum::from_bytes(b"[3,3]")
                    )
                    ,&SnailNum::from_bytes(b"[4,4]")
                )
                ,&SnailNum::from_bytes(b"[5,5]")
            ).to_string()
            , "[[[[3,0],[5,3]],[4,4]],[5,5]]"
        );
    }

    #[test]
    fn test_add_3(){
        assert_eq!(
            add(
                &add(
                    &add(
                        &add(
                            &add(
                                &SnailNum::from_bytes(b"[1,1]")
                                ,&SnailNum::from_bytes(b"[2,2]")
                            )
                            ,&SnailNum::from_bytes(b"[3,3]")
                        )
                        ,&SnailNum::from_bytes(b"[4,4]")
                    )
                    ,&SnailNum::from_bytes(b"[5,5]")
                )
                ,&SnailNum::from_bytes(b"[6,6]")
            ).to_string()
            , "[[[[5,0],[7,4]],[5,5]],[6,6]]"
        );
    }

    #[test]
    fn test_add_large_1(){
        assert_eq!(add(&SnailNum::from_bytes(b"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")
        ,&SnailNum::from_bytes(b"[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]")).to_string()
        ,"[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
    }    

    #[test]
    fn test_add_large_2(){
        assert_eq!(add(&SnailNum::from_bytes(b"[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
        ,&SnailNum::from_bytes(b"[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]")).to_string()
        ,"[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]");
    }

    #[test]
    fn test_add_large_3(){
        
        assert_eq!(add(&SnailNum::from_bytes(b"[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]")
        ,&SnailNum::from_bytes(b"[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]")).to_string()
        ,"[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]");
        
    }    

    #[test]
    fn test_add_large_4(){
        assert_eq!(add(&SnailNum::from_bytes(b"[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]")
        ,&SnailNum::from_bytes(b"[7,[5,[[3,8],[1,4]]]]")).to_string()
        ,"[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]");
        
    }    

    #[test]
    fn test_add_large_5(){
        assert_eq!(add(&SnailNum::from_bytes(b"[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]")
        ,&SnailNum::from_bytes(b"[[2,[2,2]],[8,[8,1]]]")).to_string()
        ,"[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]");
        
    }    

    #[test]
    fn test_add_large_6(){
        assert_eq!(add(&SnailNum::from_bytes(b"[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]")
        ,&SnailNum::from_bytes(b"[2,9]")).to_string()
        ,"[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]");
        
    }    

    #[test]
    fn test_add_large_7(){
        assert_eq!(add(&SnailNum::from_bytes(b"[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]")
        ,&SnailNum::from_bytes(b"[1,[[[9,3],9],[[9,0],[0,7]]]]")).to_string()
        ,"[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]");
        
    }    

    #[test]
    fn test_add_large_8(){
        assert_eq!(add(&SnailNum::from_bytes(b"[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]")
        ,&SnailNum::from_bytes(b"[[[5,[7,4]],7],1]")).to_string()
        ,"[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]");
        
    }    

    #[test]
    fn test_add_large_9(){
        assert_eq!(add(&SnailNum::from_bytes(b"[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]")
        ,&SnailNum::from_bytes(b"[[[[4,2],2],6],[8,7]]")).to_string()
        ,"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    }

    #[test]
    fn test1(){
        assert_eq!(part1(
b"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"
        ), 4140);
    }

    #[test]
    fn test2(){
        assert_eq!(part2(
b"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"
        ), 3993);
    }

}

