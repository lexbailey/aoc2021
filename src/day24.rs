use std::str;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::cmp::{min,max};
use std::ops::{Div, Sub, Mul};

#[derive(Copy,Clone,Debug)]
enum Val{
    Reg(usize)
    ,Num(i64)
}

impl Display for Val{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self{
            Val::Reg(0) => write!(f, "w")
            ,Val::Reg(1) => write!(f, "x")
            ,Val::Reg(2) => write!(f, "y")
            ,Val::Reg(3) => write!(f, "z")
            ,Val::Reg(a) => write!(f, "<??register id:{}?>", a)
            ,Val::Num(a) => write!(f, "{}", a)
        }
    }
}

#[derive(Copy,Clone)]
enum Op{
    Inp(usize)
    ,Add(usize, Val)
    ,Mul(usize, Val)
    ,Div(usize, Val)
    ,Mod(usize, Val)
    ,Eql(usize, Val)
}

impl Display for Op{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        if let Op::Inp(a) = self{
            write!(f, "inp {}", Val::Reg(*a))
        }
        else{
            match self{
                Op::Add(a, b) => write!(f, "add {} {}", Val::Reg(*a), b)
                ,Op::Mul(a, b) => write!(f, "mul {} {}", Val::Reg(*a), b)
                ,Op::Div(a, b) => write!(f, "div {} {}", Val::Reg(*a), b)
                ,Op::Mod(a, b) => write!(f, "mod {} {}", Val::Reg(*a), b)
                ,Op::Eql(a, b) => write!(f, "eql {} {}", Val::Reg(*a), b)
                ,_ => unreachable!()
            }
        }
    }
}

fn reg_index(r: &str) -> Option<usize>{
    let n = r.as_bytes()[0];
    if n > 100 {// any number between 57 and 118 could go here
        Some((n-119).into())
    }
    else{
        None
    }
}

impl Op{
    fn from_str(s: &str) -> Op{
        let mut parts = s.split(" ");
        let op_name = parts.next().unwrap();
        let dest = reg_index(parts.next().unwrap()).unwrap();
        if op_name == "inp"{
            Op::Inp(dest)
        }
        else {
            let src = parts.next().unwrap();
            let src = reg_index(src).and_then(|a|Some(Val::Reg(a))).unwrap_or_else(||Val::Num(src.parse::<i64>().unwrap()));
            match op_name{
                "add" => Op::Add(dest, src)
                ,"mul" => Op::Mul(dest, src)
                ,"div" => Op::Div(dest, src)
                ,"mod" => Op::Mod(dest, src)
                ,"eql" => Op::Eql(dest, src)
                ,_ => panic!("invalid operation")
            }
        }
    }
}

#[derive(Debug)]
struct AluState{
    regs: [i64;4]
}

fn eval(v: Val, s: &AluState) -> i64{
    match v {
        Val::Reg(i) => s.regs[i]
        ,Val::Num(n) => n
    }
}

fn run(instrs: &Vec<Op>, inputs: &mut dyn Iterator<Item=i64>) -> AluState{
    let mut s = AluState{regs:[0,0,0,0]};
    let mut inputs = inputs.into_iter();
    for i in instrs{
        match i{
            Op::Inp(dest) => {s.regs[*dest] = inputs.next().unwrap();}
            Op::Add(dest, src) => {s.regs[*dest] = s.regs[*dest] + eval(*src, &s);}
            Op::Mul(dest, src) => {s.regs[*dest] = s.regs[*dest] * eval(*src, &s);}
            Op::Div(dest, src) => {s.regs[*dest] = s.regs[*dest] / eval(*src, &s);}
            Op::Mod(dest, src) => {s.regs[*dest] = s.regs[*dest] % eval(*src, &s);}
            Op::Eql(dest, src) => {s.regs[*dest] = if s.regs[*dest] == eval(*src, &s) {1} else {0};}
        }
    }
    s
}

fn parse_program(input: &str) -> Vec<Op>{
    input.lines().map(|l|Op::from_str(l)).collect()
}

#[derive(Clone,PartialEq)]
enum Expr{
    Constant(i64)
    ,Integer(i64)
    ,Bounded(i64,i64,Box<Expr>)
    ,Sum(Box<Expr>,Box<Expr>)
    ,Product(Box<Expr>,Box<Expr>)
    ,Quotient(Box<Expr>,Box<Expr>)
    ,Remainder(Box<Expr>,Box<Expr>)
    ,WasEq(Box<Expr>,Box<Expr>)
}
/*
impl PartialEq for Expr{
    fn eq(&self, other: &Rhs) -> bool {
        match (self, other){
            (Constant(a), Constant(b)) if a == b => true
            ,(Integer(a), Integer(b)_) => false
            ,(_, Integer()) => false
            ,(Bounded(_,_), _) => false
            ,(_, Bounded(_,_)) => false
        }
    }
}
*/
impl Display for Expr{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Expr::*;
        match self{
            Constant(a) => write!(f, "{}",a)
            ,Integer(a) => write!(f, "?<{}>",a)
            ,Bounded(min,max,expr) => write!(f, "{}..{}?<{}>", min, max, expr)
            ,Sum(a,b) => write!(f, "({} + {})", a, b)
            ,Product(a,b) => write!(f, "({} * {})", a, b)
            ,Quotient(a,b) => write!(f, "({} / {})", a, b)
            ,Remainder(a,b) => write!(f, "({} % {})", a, b)
            ,WasEq(a,b) => write!(f, "({} == {})", a, b)
        }
    }
}

impl Debug for Expr{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

impl Debug for Op{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let s = format!("{}", self);
        write!(f, "{:10}", s)
    }
}

fn simplify(expr: &Expr) -> Expr{
    use Expr::*;
    match expr {
        Bounded(a, b, expr) if a == b => Constant(*a)
        ,Product(a, b) => match (&**a, &**b){
            (a, Constant(0)) => Constant(0)
            ,(a, Constant(1)) => simplify(a)
            ,(Constant(0), a) => Constant(0)
            ,(Constant(1), a) => simplify(a)
            ,(Constant(a), Constant(b)) => Constant(a*b)
            ,(Bounded(a_min, a_max, bexpr), Constant(b)) => simplify(&Bounded(a_min*b, a_max*b, Box::new(simplify(&Product(bexpr.clone(), Box::new(Constant(*b)))))))
            ,(Constant(b), Bounded(a_min, a_max, bexpr)) => simplify(&Bounded(a_min*b, a_max*b, Box::new(simplify(&Product(Box::new(Constant(*b)), bexpr.clone())))))
            ,(Bounded(a_min, a_max, aexpr), Bounded(b_min, b_max, bexpr)) => {
                let c1 = a_min * b_min;
                let c2 = a_max * b_min;
                let c3 = a_min * b_max;
                let c4 = a_max * b_max;
                let low = min(min(c1,c2),min(c3,c4));
                let high = max(max(c1,c2),max(c3,c4));
                simplify(
                    &Bounded(low, high, Box::new(simplify(&Product(aexpr.clone(), bexpr.clone()))))
                )
            }
            ,_=>expr.clone()
        } 
        ,Sum(a,b) => match (&**a, &**b){
            (Constant(a), Constant(b)) => Constant(a+b)
            ,(Constant(a), Bounded(min, max, bexpr)) => simplify(&Bounded(min+a, max+a, Box::new(simplify(&Sum(Box::new(Constant(*a)), bexpr.clone())))))
            ,(Bounded(min, max, bexpr), Constant(a)) => simplify(&Bounded(min+a, max+a, Box::new(simplify(&Sum(bexpr.clone(), Box::new(Constant(*a)))))))
            ,(a, Constant(0)) => simplify(a)
            ,(Constant(0), a) => simplify(a)
            ,_=>expr.clone()
        }
        ,Quotient(a,b) => match (&**a, &**b){
            (Constant(a), Constant(b)) => Constant(a/b)
            ,(Constant(a), Bounded(min, max, bexpr)) => simplify(&Bounded(min/a, max/a, Box::new(simplify(&Quotient(Box::new(Constant(*a)), bexpr.clone())))))
            ,(Bounded(min, max, bexpr), Constant(a)) => simplify(&Bounded(min/a, max/a, Box::new(simplify(&Quotient(bexpr.clone(), Box::new(Constant(*a)))))))
            ,(a, Constant(1)) => simplify(a)
            ,_=>expr.clone()
        }
        ,Remainder(a,b) => match (&**a, &**b){
            (Constant(a), Constant(b)) => Constant(a%b)
            ,(Bounded(min_, max_, aexpr), Constant(b)) if b > max_ => simplify(a)
            ,(Bounded(min_, max_, aexpr), Constant(b)) => simplify(&Bounded(if max_ > b {0} else {*min_}, min(*max_,b-1), Box::new(Remainder(aexpr.clone(), Box::new(Constant(*b))))))
            ,(a, Constant(b)) => simplify(&Bounded(0, b-1, Box::new(Remainder(Box::new(a.clone()), Box::new(Constant(*b))))))
            ,_=>expr.clone()
        }
        ,WasEq(a,b) => match (&**a, &**b){
            (Bounded(sa, ea, _), Bounded(sb, eb, _)) if ea < sb || sa > eb => Constant(0)
            ,(Constant(a), Bounded(min, max, _)) if a < min || a > max => Constant(0)
            ,(Bounded(min, max, _), Constant(a)) if a < min || a > max => Constant(0)
            ,(Constant(a), Constant(b)) => Constant(if a==b {1} else {0})
            ,_=>Bounded(0,1,Box::new(expr.clone()))
        }
        ,_ => expr.clone()
    }
}


#[derive(PartialEq,Copy,Clone,Debug)]
enum Symbol{
    Unknown()
    ,Exact(i64)
    ,Range(i64, i64)
    ,NotEqual(i64)
}

impl Mul for Symbol{
    type Output = Symbol;
    fn mul(self, rhs: Symbol) -> Self::Output{
        use Symbol::*;
        match (self, rhs) {
            (Unknown(), _) => Unknown()
            ,(_, Unknown()) => Unknown()
            ,(_, Exact(0)) => Exact(0)
            ,(Exact(0), _) => Exact(0)
            ,(Exact(a), Exact(b)) => Exact(a*b)
            ,(Exact(a), Range(min, max)) => Range(a*min, a*max)
            ,(Range(min, max), Exact(a)) => Range(min*a, max*a)
            ,(Range(mina, maxa), Range(minb, maxb)) => Range(mina*minb,maxa*maxb)
            ,(NotEqual(a), Exact(b)) => NotEqual(a*b)
            ,(Exact(a), NotEqual(b)) => NotEqual(a*b)
            ,_ => Unknown()
        }
    }
}

impl Div for Symbol{
    type Output = Symbol;
    fn div(self, rhs: Symbol) -> Self::Output{
        use Symbol::*;
        match (self, rhs) {
            (Unknown(), _) => Unknown()
            ,(_, Unknown()) => Unknown()
            ,(_, Exact(0)) => Unknown()
            ,(Exact(a), Exact(b)) => Exact(a/b)
            ,(Exact(a), Range(min, max)) if 0 >= min && 0 <= max => Unknown()
            ,(Exact(a), Range(min, max)) => Range(a/min, a/max)
            ,(Range(min, max), Exact(a)) => Range(min/a, max/a)
            ,(Range(mina, maxa), Range(minb, maxb)) => {
                if minb == 0 || maxb == 0{
                    if mina == 0 && minb == 0{
                        Range(0, maxa/maxb)
                    }
                    else{
                        Unknown()
                    }
                }
                else{
                    Range(mina/maxb,maxa/minb)
                }
            }
            ,(NotEqual(a), Exact(b)) => NotEqual(a/b)
            ,(Exact(a), NotEqual(b)) => NotEqual(a/b)
            ,_ => Unknown()
        }
    }
}

impl Sub for Symbol{
    type Output = Symbol;
    fn sub(self, rhs: Symbol) -> Self::Output{
        use Symbol::*;
        match (self, rhs) {
            (Unknown(), _) => Unknown()
            ,(_, Unknown()) => Unknown()
            ,(Exact(a), Exact(b)) => Exact(a-b)
            ,(Exact(a), Range(min, max)) => Range(a-min, a-max)
            ,(Range(min, max), Exact(a)) => Range(min-a, max-a)
            ,(Range(mina, maxa), Range(minb, maxb)) => Range(mina-maxb,maxa-minb)
            ,(NotEqual(a), Exact(b)) => NotEqual(a-b)
            ,(Exact(a), NotEqual(b)) => NotEqual(a-b)
            ,_ => Unknown()
        }
    }
}

fn sym_hints(exprs: &[Expr;4]) -> [Symbol;4]{
    use Symbol::*;
    use Expr::*;
    let mut result = [Unknown();4];
    for i in 0..=3{
        result[i] = match exprs[i]{
            Constant(a) => Exact(a)
            ,Bounded(min, max, _) => Range(min, max)
            ,_ => Unknown()
        }
    }
    result
}

fn annotate(program: &Vec<Op>, input_bounds: Option<(i64,i64)>) -> Vec<(Op, [Symbol;4])>{
    const START_STATE: Expr = Expr::Constant(0);
    let mut reg_states = [START_STATE;4];
    let mut result: Vec<(Op, [Symbol;4])> = Vec::new();
    let expr = |v,regs:&[Expr;4]|{
        match v{
            Val::Num(a) => Expr::Constant(a)
            ,Val::Reg(a) => regs[a].clone()
        }
    };
    let mut next_id = 1000;
    let mut line = 1;
    for op in program{
        let before_states = reg_states.clone();
        match op{
            Op::Inp(a) => {
                if let Some((min,max)) = input_bounds{
                    reg_states[*a] = Expr::Bounded(min, max, Box::new(Expr::Integer(next_id)));next_id += 1000;
                }
                else{
                    reg_states[*a] = Expr::Integer(next_id);next_id += 1000;
                }
            }
            Op::Add(a, b) => {reg_states[*a] = Expr::Sum(Box::new(reg_states[*a].clone()), Box::new(expr(*b,&reg_states).clone()));}
            Op::Mul(a, b) => {reg_states[*a] = Expr::Product(Box::new(reg_states[*a].clone()), Box::new(expr(*b,&reg_states).clone()));}
            Op::Div(a, b) => {reg_states[*a] = Expr::Quotient(Box::new(reg_states[*a].clone()), Box::new(expr(*b,&reg_states).clone()));}
            Op::Mod(a, b) => {reg_states[*a] = Expr::Remainder(Box::new(reg_states[*a].clone()), Box::new(expr(*b,&reg_states).clone()));}
            Op::Eql(a, b) => {reg_states[*a] = Expr::WasEq(Box::new(reg_states[*a].clone()), Box::new(expr(*b, &reg_states).clone()))}
        }
        for i in 0..=3{
            reg_states[i] = simplify(&reg_states[i]);
        }
        //println!("After executing {:?} register state is: {:?}", op, reg_states);
        if (0..=3).all(|i|before_states[i] == reg_states[i]){
            //println!("{} After executing {:?} register state is unchanged", line, op);
        }
        else{
            //println!("{} After executing {:?}: {:?}", line, op, reg_states);
            let hints = sym_hints(&reg_states);
            println!("{}: {:?} {:?}", line, op, hints);
            line += 1;
            result.push((op.clone(), hints));
        }
    }
    //println!("{:?}", reg_states[3]);
    result
}

fn merge_symbols(a: Symbol, b: Symbol) -> Symbol{
    use Symbol::*;
    match (a, b){
        (Unknown(), b) => b
        ,(a, Unknown()) => a
        ,(Exact(a), _) => Exact(a)
        ,(_, Exact(b)) => Exact(b)
        ,(Range(mina, maxa), Range(minb, maxb)) => {
            let low = max(mina, minb);
            let high = min(maxa, maxb);
            if low == high {
                Exact(low)
            }
            else{
                Range(low, high)
            }
        }
        ,_ => a
    }
}

fn rev_exec(program: Vec<(Op, [Symbol;4])>, finals:[Symbol;4]) -> Vec<Symbol> {
    let mut states = finals;
    let mut in_n = 0;
    use Op::*;
    use Symbol::*;
    use Val::*;
    let mut inputs: Vec<Symbol> = Vec::new();
    println!("{:?}", states);
    for (op, hints) in program.iter().rev(){
        println!("{:?}", op);
        for i in 0..=3{
            states[i] = merge_symbols(hints[i], states[i]);
        }
        //println!("State with pre-hints: {:?}", states);
        match op{
            Inp(a) => {
                //println!("Input end-{} = {:?}", in_n, states[*a]);
                in_n += 1;
                inputs.push(states[*a]);
            }
            ,Mul(a,b) => {
                let this = states[*a];
                let other = match b { Reg(b) => states[*b] ,Num(n) => Exact(*n) };
                states[*a] = this/other;
            }
            ,Div(a,b) => {
                let this = states[*a];
                let other = match b { Reg(b) => states[*b] ,Num(n) => Exact(*n) };
                states[*a] = this*other;
            }
            ,Mod(a,b) =>{
                //let this = states[*a];
                //let other = match b { Reg(b) => states[*b] ,Num(n) => Exact(*n) };
                states[*a] = Unknown()
            }
            ,Eql(a,b) => {
                let this = states[*a];
                let other = match b { Reg(b) => states[*b] ,Num(n) => Exact(*n) };
                states[*a] = match this{
                    Exact(1) => other
                    ,Exact(0) => match other {
                        Exact(b) => NotEqual(b)
                        ,_ => Unknown()
                    }
                    _ => Unknown()
                }
            }
            ,Add(a,b) => {
                let this = states[*a];
                let other = match b { Reg(b) => states[*b] ,Num(n) => Exact(*n) };
                states[*a] = this - other;
            }
            ,_ => {panic!("not implemented");}
        }
        println!("New states:       {:?}", states);
    }
    inputs.reverse();
    inputs
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> i64 {
    let monad = parse_program(input); // monad is a silly name for this thing <_<
    let monad = annotate(&monad, Some((1,9)));
    use Symbol::*;
    let outputs = [Unknown(), Unknown(), Unknown(), Exact(0)];
    let inputs = rev_exec(monad, outputs);
    println!("{:?}", inputs);
    /*
    let nums = vec![1..9;14];
    let mut id: i64 = 11111111111111;
    loop {
        id = (id-1).to_string().chars().map(|c| if c == '0' {'1'} else {c}).collect::<String>().parse::<i64>().unwrap();
        let sid = id.to_string();
        let mut inputs = sid.as_bytes().iter().map(|c|(c-48)as i64);
        let result = run(&monad, &mut inputs);
        let valid = result.regs[3] == 0;
        if (id % 100001) == 0{
            println!("{}: {}", sid, valid);
        }
        if valid{
            return id;
        }
        if id == 0{
            break;
        }
    }*/
    -1
}

#[aoc(day24, part2)]
pub fn part2(input: &[u8]) -> i64 {
    -1
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, run, parse_program, Symbol, rev_exec};

    #[test]
    fn test_a(){
        let prog = "inp x
mul x -1";
        let instrs = parse_program(prog);
        let mut inputs = vec![4].into_iter();
        let mut result = run(&instrs, &mut inputs);
        assert_eq!(result.regs[1], -4);
    }

    #[test]
    fn test_b(){
        let prog = "inp z
inp x
mul z 3
eql z x
";
        let instrs = parse_program(prog);
        let mut inputs1 = vec![4,12].into_iter();
        let mut inputs2 = vec![4,11].into_iter();
        let result1 = run(&instrs, &mut inputs1);
        let result2 = run(&instrs, &mut inputs2);
        println!("{:?}", result1);
        println!("{:?}", result2);
        assert_eq!(result1.regs[3], 1);
        assert_eq!(result2.regs[3], 0);
    }

    #[test]
    fn test_rev_1(){
        use Symbol::*;
        let prog =
"inp x
mul x 3
";
        let instrs = parse_program(prog);
        let outputs = [Unknown(), Exact(12), Unknown(), Unknown()];
        let result = rev_exec(instrs, outputs);
        assert_eq!(result[0], Exact(4));
    }

    #[test]
    fn test_rev_2(){
        use Symbol::*;
        let prog =
"inp x
inp y
mul x 3
mul y 0
add y x
eql y 12
";
        let instrs = parse_program(prog);
        let outputs = [Unknown(), Exact(12), Exact(1), Unknown()];
        let result = rev_exec(instrs, outputs);
        assert_eq!(result[0], Exact(4));
        assert_eq!(result[1], Unknown());

        let instrs = parse_program(prog);
        let outputs = [Unknown(), Exact(6), Exact(0), Unknown()];
        let result = rev_exec(instrs, outputs);
        assert_eq!(result[0], Exact(2));
        assert_eq!(result[1], Unknown());

    }

    #[test]
    fn test2(){
        assert_eq!(part1(
            ""), 19);
    }
}



