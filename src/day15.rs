use std::str;
use petgraph::graph::{Graph,DiGraph,NodeIndex};
use petgraph::algo;

fn repeat(g: Vec<Vec<char>>, reps: i32) -> Vec<Vec<char>>{
    let start_h = g.len() as i32;
    let start_w = g[0].len() as i32;
    let mut newgrid: Vec<Vec<char>> = Vec::with_capacity((start_h*reps) as usize);
    for i in 0..reps{
        for line in &g{
            let mut newline : Vec<char> = Vec::with_capacity((start_w*reps) as usize);
            for j in 0..reps{
                for c in line{
                    // TODO is there a simpler way to do this arithmetic?
                    let oldval = c.to_string().parse::<i32>().unwrap();
                    let mut newval = (oldval+i)%10;
                    if newval<oldval{newval += 1;}
                    let oldval = newval;
                    newval = (oldval+j)%10;
                    if newval<oldval{newval += 1;}
                    newline.push(newval.to_string().chars().next().unwrap());
                }
            }
            newgrid.push(newline);
        }
    }
    newgrid
}

fn build_graph(input: &[u8], reps: i32) -> (Graph<i32,i32>, NodeIndex, NodeIndex){
    let mut grid: Vec<Vec<char>> = str::from_utf8(input).unwrap().trim().lines().map(|s|s.trim().chars().collect()).collect();
    if reps>1{
        grid = repeat(grid, reps);
    }
    let mut g: Graph<i32, i32>  = DiGraph::<_, _>::new();
    let w = grid[0].len();
    let h = grid.len();
    let mut nodes: Vec<Vec<NodeIndex>> = Vec::with_capacity(h);
    for y in 0..h{
        nodes.push(Vec::with_capacity(w));
        for x in 0..w{
            let val = grid[y][x].to_string().parse::<i32>().unwrap();
            let node = g.add_node(val);
            if x > 0{
                g.add_edge(nodes[y][x-1], node, val);
                g.add_edge(node, nodes[y][x-1], g[nodes[y][x-1]]);
            }
            if y > 0{
                g.add_edge(nodes[y-1][x], node, val);
                g.add_edge(node, nodes[y-1][x], g[nodes[y-1][x]]);
            }
            nodes[y].push(node);
        }
    }
    (g, nodes[0][0], nodes[h-1][w-1])
}

#[aoc(day15, part1)]
pub fn part1(input: &[u8]) -> i32 {
    let (g, start, end) = build_graph(input, 1);
    let path = algo::astar(&g, start, |n|n==end, |e|*e.weight(), |n|g[n]).unwrap();
    path.0
}

#[aoc(day15, part2)]
pub fn part2(input: &[u8]) -> i32 {
    let (g, start, end) = build_graph(input, 5);
    let path = algo::astar(&g, start, |n|n==end, |e|*e.weight(), |n|g[n]).unwrap();
    path.0
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1(){
        assert_eq!(part1(
b"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"
        ), 40);
    }

    #[test]
    fn test2(){
        assert_eq!(part2(
b"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"
        ), 315);
    }
}

