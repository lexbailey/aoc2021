use std::str;
use petgraph::graph::{Graph,UnGraph,NodeIndex};
use petgraph::visit::EdgeRef;
use std::ops::Add;
use std::cell::RefCell;

#[derive(Copy, Clone)]
struct Node<'a>{
    is_big: bool
    ,visited: bool
    ,is_start: bool
    ,is_end: bool
    ,name: &'a str
}

impl<'a> Node<'a>{
    fn from_name(n:&'a str) -> Node{
        Node{
            name: n
            ,is_start: n=="start"
            ,is_end: n=="end"
            ,is_big: n.as_bytes()[0] < 97
            ,visited: false
        }
    }
}

fn paths(g: &mut UnGraph<Node,()>, start: NodeIndex, end: NodeIndex) -> i64{
    if start == end {1}
    else{
        g.node_weight_mut(start).unwrap().visited = true;
        let nexts: Vec<_> = g.edges(start).filter_map(|e|
            {
                let node_id = e.target();
                let next_node = g.node_weight(node_id).unwrap();
                if next_node.is_big || !next_node.visited {
                    Some(node_id)
                }
                else {
                    None
                }
            }
        ).collect();
        let sum = nexts.into_iter().map(|next| {
            paths(g, next, end)
        }).reduce(i64::add).unwrap_or(0);
        g.node_weight_mut(start).unwrap().visited = false;
        sum
    }
}


#[aoc(day12, part1)]
pub fn part1(input: &[u8]) -> i64 {
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

    print!("{} nodes found\n", g.node_count());
    print!("{} edges found\n", g.edge_count());

    paths(&mut g, start.unwrap(), end.unwrap())
}

#[aoc(day12, part2)]
pub fn part2(input: &[u8]) -> i64 {
    0
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

    //#[test]
    //fn test2(){
    //    assert_eq!(part2(
    //        b"start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end\n"), -1);
    //}

}



