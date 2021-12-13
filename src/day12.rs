use std::str;
use petgraph::graph::{UnGraph,NodeIndex};
use petgraph::visit::EdgeRef;
use std::ops::Add;

#[derive(Copy, Clone)]
struct Node<'a>{
    is_big: bool
    ,is_start: bool
    ,is_end: bool
    ,name: &'a str
    ,visit_max: Option<i64>
    ,visits: i64
}

impl<'a> Node<'a>{
    fn from_name(n:&'a str) -> Node{
        let start = n=="start";
        let end = n=="end";
        let big = n.as_bytes()[0] < 97;
        Node{
            name: n
            ,is_start: start
            ,is_end: end
            ,is_big: big
            ,visits: 0
            ,visit_max:
                if start || end {Some(1)}
                else{
                    if big {None}
                    else {Some(1)}
                }
        }
    }
}

fn paths(g: &mut UnGraph<Node,()>, start: NodeIndex, end: NodeIndex) -> i64{
    if start == end {1}
    else{
        {
            let mut n = g.node_weight_mut(start).unwrap();
            n.visits += 1
        }
        let sum = g.edges(start).filter_map(|e|
            {
                let node_id = e.target();
                let next_node = g.node_weight(node_id).unwrap();
                if next_node.visit_max.and_then(|m|Some(next_node.visits < m)).or(Some(true)).unwrap() {
                    Some(node_id)
                }
                else {
                    None
                }
            }
        ).collect::<Vec<_>>().into_iter().map(|next| {
            paths(g, next, end)
        }).reduce(i64::add).unwrap_or(0);
        {
            let mut n = g.node_weight_mut(start).unwrap();
            n.visits -= 1;
        }
        sum
    }
}

fn big_paths(g: &mut UnGraph<Node,()>, start: NodeIndex, end: NodeIndex) -> i64{
    let doubleable = g.node_indices().filter_map(|i|{
        let n = g.node_weight(i).unwrap();
        if !n.is_big && !n.is_start && !n.is_end {
            Some(i)
        }
        else{
            None
        }
    }).collect::<Vec<_>>();
    let m = doubleable.len() as i64 - 1;
    doubleable.into_iter().map(|b|{
        {
            let n = g.node_weight_mut(b).unwrap();
            assert!(n.visit_max == Some(1));
            assert!(n.is_start == false);
            assert!(n.is_end == false);
            n.visit_max = Some(2);
        }
        let p = paths(g, start, end);
        {
            let n = g.node_weight_mut(b).unwrap();
            n.visit_max = Some(1);
        }
        p
    }).reduce(i64::add).unwrap() - (m * paths(g, start, end))
}

fn build_graph(input: &[u8]) -> (UnGraph<Node,()>, NodeIndex, NodeIndex){
    let edge_pairs: Vec<Vec<&str>> = str::from_utf8(input).unwrap().trim().lines().map(|s|s.split("-").collect()).collect();
    let mut g: UnGraph<Node, ()>  = UnGraph::<_, _>::new_undirected();
    let mut start: Option<NodeIndex<_>> = None;
    let mut end: Option<NodeIndex<_>> = None;
    for edge in edge_pairs{
        let nodes: Vec<_> = edge.iter().map(|n|
            match g.node_indices().find(|w|g.node_weight(*w).unwrap().name==*n) {
                Some(id) => id,
                None => {
                    let newnode = Node::from_name(n);
                    let isstart = newnode.is_start;
                    let isend = newnode.is_end;
                    let n_id: NodeIndex<_> = g.add_node(newnode);
                    if isstart { start = Some(n_id); }
                    if isend { end = Some(n_id); }
                    n_id
                },
            }
        ).collect();
        g.add_edge(nodes[0], nodes[1],());
        if g.node_weight(nodes[0]).unwrap().is_big && g.node_weight(nodes[1]).unwrap().is_big{
            print!("BIG CAVES!!!!!! Big cave adjacent to big cave!\n")
        }
    }
    (g, start.unwrap(), end.unwrap())
}

#[aoc(day12, part1)]
pub fn part1(input: &[u8]) -> i64 {
    let (mut g, start, end) = build_graph(input);
    paths(&mut g, start, end)
}

#[aoc(day12, part2)]
pub fn part2(input: &[u8]) -> i64 {
    let (mut g, start, end) = build_graph(input);
    big_paths(&mut g, start, end)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test1_1(){
        assert_eq!(part1(
            b"start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end\n"), 10);
    }
    #[test]
    fn test1_2(){
        assert_eq!(part1(
            b"\ndc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc\n"), 19);
    }
    #[test]
    fn test1_3(){
        assert_eq!(part1(
            b"\nfs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW")
            ,226
        )
    }

    #[test]
    fn test2_1(){
        assert_eq!(part2(
            b"start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end\n"), 36);
    }
    #[test]
    fn test2_2(){
        assert_eq!(part2(
            b"\ndc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc\n"), 103);
    }
    #[test]
    fn test2_3(){
        assert_eq!(part2(
            b"\nfs-end\nhe-DX\nfs-he\nstart-DX\npj-DX\nend-zg\nzg-sl\nzg-pj\npj-he\nRW-he\nfs-DX\npj-RW\nzg-RW\nstart-pj\nhe-WI\nzg-he\npj-fs\nstart-RW")
            ,3509
        )
    }
}



