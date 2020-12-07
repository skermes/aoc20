use petgraph::{Directed, Direction};
use petgraph::graphmap::GraphMap;
use std::collections::HashSet;
use crate::aoc_error::AocError;

pub const NAME: &str = "Handy Haversacks";

#[derive(Debug)]
struct BagSpec<'a> {
    name: &'a str,
    contents: Vec<(&'a str, u8)>
}

impl BagSpec<'_> {
    fn from_str(line: &str) -> Result<BagSpec, AocError> {
        let (name, contents_str) = match line.split_once(" bags contain ") {
            Some((name, contents_str)) => (name, contents_str),
            None => return Err(AocError::Misc("Invalid bag spec".to_string()))
        };

        if contents_str == "no other bags." {
            Ok(BagSpec{ name, contents: vec![] })
        } else {
            let contents = contents_str
                .split(", ")
                .map(|spec| {
                    let tokens: Vec<&str> = spec.split_whitespace().collect();
                    if tokens.len() < 4 {
                        Err(AocError::Misc("Invalid bag spec".to_string()))
                    } else {
                        Ok((
                            &spec[2..(tokens[1].len() + tokens[2].len() + 3)],
                            tokens[0].parse::<u8>()?
                        ))
                    }
                })
                .collect::<Result<Vec<(&str, u8)>, AocError>>()?;

            Ok(BagSpec{ name, contents })
        }
    }
}

fn bag_graph(text: &str) -> Result<GraphMap<&str, u8, Directed>, AocError> {
    let specs: Vec<BagSpec> = text
        .lines()
        .map(BagSpec::from_str)
        .collect::<Result<Vec<BagSpec>, AocError>>()?;

    // This * 3 is a ballpark from eyeballing the input.
    let mut graph = GraphMap::<&str, u8, Directed>::with_capacity(specs.len(), specs.len() * 3);

    for spec in specs {
        graph.add_node(spec.name);
        for (name, weight) in spec.contents {
            // By using a GraphMap we can safely add keys multiple times.
            graph.add_node(name);
            // Because GraphMap stores edges in both directions it doesn't
            // really matter which direction we make the edge in, but this is
            // conceptually simpler for part one.
            graph.add_edge(name, spec.name, weight);
        }
    }

    Ok(graph)
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let graph = bag_graph(input)?;

    let mut visited: HashSet<&str> = HashSet::new();
    let mut to_visit: HashSet<&str> = HashSet::new();
    to_visit.insert("shiny gold");

    loop {
        if to_visit.is_empty() { break; }

        let popped = to_visit.iter().next().cloned().unwrap();
        visited.insert(popped);
        to_visit.remove(popped);

        for node in graph.neighbors_directed(popped, Direction::Outgoing) {
            if !visited.contains(node) {
                to_visit.insert(node);
            }
        }
    }

    // Visited includes the bag we started at, sub that out.
    let parent_bag_count = visited.len() - 1;

    Ok(parent_bag_count.to_string())
}

fn count_bag_contents(graph: &GraphMap<&str, u8, Directed>, root: &str) -> usize {
    graph.neighbors_directed(root, Direction::Incoming)
        .map(|node| {
            let bag_count = *graph.edge_weight(node, root).unwrap() as usize;
            bag_count * (count_bag_contents(graph, node) + 1)
        })
        .sum()
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let graph = bag_graph(input)?;
    let bag_size = count_bag_contents(&graph, "shiny gold");

    Ok(bag_size.to_string())
}