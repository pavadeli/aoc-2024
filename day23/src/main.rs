use common::*;
use id_arena::{Arena, Id};
use std::{
    cmp::max_by_key,
    collections::{HashMap, HashSet},
};

struct Node {
    name: SS,
    id: Id<Node>,
    links: Vec<Id<Node>>,
}

fn part1(input: SS) -> usize {
    let nodes = parse(input);
    let mut seen = HashSet::with_capacity(nodes.len());
    let mut sets = HashSet::new();
    for (id1, n1) in nodes.iter().filter(|(_, n)| n.name.starts_with('t')) {
        seen.insert(id1);
        for &id2 in n1.links.iter().filter(|&n| !seen.contains(n)) {
            for &id3 in nodes[id2]
                .links
                .iter()
                .filter(|&n| nodes[*n].links.contains(&id1) && !seen.contains(n))
            {
                let mut set = [id1, id2, id3];
                set.sort();
                sets.insert(set);
            }
        }
    }
    sets.len()
}

fn part2(input: SS) -> String {
    let nodes = parse(input);
    bron_kerbosch(&nodes, vec![], nodes.iter().map(|t| t.0).collect(), vec![])
        .into_iter()
        .map(|n| nodes[n].name)
        .sorted()
        .join(",")
}

/// Implementation of the Bron-Kerbosch algoritm with pivoting, pseudocode from
/// Wikipedia:
/// ```text
/// algorithm BronKerbosch2(R, P, X) is
///    if P and X are both empty then
///        report R as a maximal clique
///    choose a pivot vertex u in P ⋃ X
///    for each vertex v in P \ N(u) do
///        BronKerbosch2(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
///        P := P \ {v}
///        X := X ⋃ {v}
/// ```
fn bron_kerbosch(
    nodes: &Arena<Node>,
    r: Vec<Id<Node>>,
    mut p: Vec<Id<Node>>,
    mut x: Vec<Id<Node>>,
) -> Vec<Id<Node>> {
    // if P and X are both empty then
    if p.is_empty() && x.is_empty() {
        // report R as a maximal clique
        return r;
    }
    // choose a pivot vertex u in P ⋃ X
    let u = p
        .iter()
        .chain(&x)
        .map(|&n| &nodes[n])
        .max_by_key(|n| n.links.len())
        .unwrap();
    // for each vertex v in P \ N(u) do
    let mut vertices = p.clone();
    vertices.retain(|n| !u.links.contains(n));
    let mut max = vec![];
    for v in vertices {
        let v = &nodes[v];
        // BronKerbosch2(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
        let maybe_max = bron_kerbosch(
            nodes,
            r.iter().copied().chain([v.id]).collect(),
            p.iter().copied().filter(|n| v.links.contains(n)).collect(),
            x.iter().copied().filter(|n| v.links.contains(n)).collect(),
        );
        max = max_by_key(max, maybe_max, Vec::len);
        // P := P \ {v}
        p.retain(|&n| n != v.id);
        // X := X ⋃ {v}
        x.push(v.id);
    }
    max
}

fn parse(input: SS) -> Arena<Node> {
    // first parse the graph as name => list of names
    let graph = input
        .lines()
        .flat_map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            [(a, b), (b, a)]
        })
        .into_group_map();

    // Now build our arena with efficient node addressing.
    let mut arena = Arena::new();
    let ids: HashMap<_, _> = graph
        .keys()
        .map(|&name| {
            (
                name,
                arena.alloc_with_id(|id| Node {
                    name,
                    id,
                    links: vec![],
                }),
            )
        })
        .collect();
    let mut link_ids: HashMap<_, _> = graph
        .into_iter()
        .map(|(k, v)| (ids[k], v.into_iter().map(|n| ids[n]).collect()))
        .collect();
    for node in arena.iter_mut() {
        node.1.links = link_ids.remove(&node.0).unwrap_or_default();
    }
    arena
}

boilerplate! {
    part1 => { test -> 7, real -> 1119 }
    part2 => { test -> "co,de,ka,ta", real -> "av,fr,gj,hk,ii,je,jo,lq,ny,qd,uq,wq,xc" }
}
