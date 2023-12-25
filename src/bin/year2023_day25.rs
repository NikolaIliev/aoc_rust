use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use aoc_rust::read_input;
use itertools::Itertools;

use rand::Rng;

#[derive(Debug, Clone)]
struct SuperVertex<'a> {
    vertices: Vec<&'a str>,
    edges: HashMap<&'a str, HashSet<&'a str>>,
}

fn contract<'a>(sv: &mut SuperVertex<'a>, other: &mut SuperVertex<'a>) {
    for (_, conns) in &mut sv.edges {
        for &v in &other.vertices {
            conns.remove(v);
        }
    }

    for (_, conns) in &mut other.edges {
        for &v in &sv.vertices {
            conns.remove(v);
        }
    }

    for (v, conns) in &mut other.edges {
        if sv.edges.get(v).is_none() {
            sv.edges.insert(v, conns.clone());
        } else {
            for &vv in conns.iter() {
                sv.edges.get_mut(v).unwrap().insert(vv);
            }
        }
    }

    sv.vertices.append(&mut other.vertices);
}

fn parse_graph<'a>(input: &'a str) -> Vec<SuperVertex<'a>> {
    input
        .lines()
        .fold(
            HashMap::<&'a str, SuperVertex<'a>>::new(),
            |mut graph, line| {
                let (left, right) = line.split_once(": ").unwrap();

                if graph.get(left).is_none() {
                    graph.insert(
                        left,
                        SuperVertex {
                            vertices: vec![left],
                            edges: HashMap::<&'a str, HashSet<&'a str>>::new(),
                        },
                    );

                    graph
                        .get_mut(left)
                        .unwrap()
                        .edges
                        .insert(left, HashSet::<&'a str>::new());
                }

                for r in right.split(" ") {
                    graph
                        .get_mut(left)
                        .unwrap()
                        .edges
                        .get_mut(left)
                        .unwrap()
                        .insert(r);

                    if graph.get(r).is_none() {
                        graph.insert(
                            r,
                            SuperVertex {
                                vertices: vec![r],
                                edges: HashMap::<&'a str, HashSet<&'a str>>::new(),
                            },
                        );

                        graph
                            .get_mut(r)
                            .unwrap()
                            .edges
                            .insert(r, HashSet::<&'a str>::new());
                    }

                    graph
                        .get_mut(r)
                        .unwrap()
                        .edges
                        .get_mut(r)
                        .unwrap()
                        .insert(left);
                }

                graph
            },
        )
        .into_values()
        .collect_vec()
}

// https://en.wikipedia.org/wiki/Karger%27s_algorithm
fn part_1(input: &str) -> String {
    let graph = parse_graph(input);
    let mut rng = rand::thread_rng();

    loop {
        let mut g = graph.clone();

        while g.len() > 2 {
            let mut sv = g.swap_remove(rng.gen_range(0..g.len()));

            let mut v: &str = "";

            while sv.edges.get(v).is_none() || sv.edges.get(v).unwrap().is_empty() {
                v = sv.vertices[rng.gen_range(0..sv.vertices.len())];
            }

            let other_v = *sv
                .edges
                .get(v)
                .unwrap()
                .iter()
                .nth(rng.gen_range(0..sv.edges.get(v).unwrap().len()))
                .unwrap();

            let mut target_sv = g
                .iter_mut()
                .find(|sv| sv.vertices.iter().any(|&v| v == other_v))
                .unwrap();

            contract(&mut target_sv, &mut sv);
        }
        if g[0]
            .edges
            .iter()
            .map(|(_, conns)| conns.len())
            .sum::<usize>()
            == 3
        {
            return (g[0].vertices.len() * g[1].vertices.len()).to_string();
        }
    }
}

fn main() {
    let input = read_input("2023", "25");

    let start_part_1 = Instant::now();
    let part_1_result = part_1(&input);
    let part_1_time = start_part_1.elapsed();

    println!();
    println!("Part 1: {} ({:?})", part_1_result, part_1_time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r"
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
        "
        .trim();

        assert_eq!(part_1(input), "54");
    }
}
