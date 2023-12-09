use std::collections::BTreeMap;
use std::error::Error;
use std::fs::read_to_string;
use std::str::Lines;

#[derive(Clone, Debug)]
struct Node {
    id: NodeId,
    left: String,
    right: String,
}

type NodeId = String;
type Instructions = String;

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_to_string("data/day08.txt").expect("file");
    let mut lines = contents.lines();
    let answer = answer(&mut lines)?;
    println!("Answer: {answer}");
    Ok(())
}

fn parse(lines: &mut Lines) -> Result<(Instructions, Vec<Node>), Box<dyn Error>> {
    let mut lines = lines.map(|line| line.trim());
    let instructions = lines.next().expect("instructions").to_owned();
    lines.next().expect("blank line");
    let nodes: Vec<Node> = lines
        .map(|line| Node {
            id: line[0..3].to_owned(),
            left: line[7..10].to_owned(),
            right: line[12..15].to_owned(),
        })
        .collect();
    nodes.iter().for_each(|node| {
        assert_eq!(node.id.len(), 3);
        assert_eq!(node.left.len(), 3);
        assert_eq!(node.right.len(), 3);
    });
    Ok((instructions, nodes))
}

fn answer(lines: &mut Lines) -> Result<u64, Box<dyn Error>> {
    let (instructions, nodes) = parse(lines)?;
    let node_map: BTreeMap<NodeId, Node> = nodes
        .iter()
        .map(|node| (node.id.clone(), node.clone()))
        .collect();

    // All nodes that start with A.
    let starting_paths: Vec<Node> = node_map
        .values()
        .filter_map(|node| node.id.ends_with('A').then_some(node.clone()))
        .collect();

    // Find each path separately. Assumes each path only has one node ending in 'Z'.
    let lengths: Vec<u64> = starting_paths
        .iter()
        .map(|starting_node| {
            let mut node = starting_node;
            let mut length = 0;
            for instruction in instructions.chars().cycle() {
                let next_node_id = match instruction {
                    'L' => &node.left,
                    'R' => &node.right,
                    _ => panic!("unknown instruction"),
                };
                node = node_map.get(next_node_id).expect("node");
                length += 1;
                if node.id.ends_with('Z') {
                    break;
                }
            }
            length as u64
        })
        .collect();

    fn greatest_common_divisor(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            greatest_common_divisor(b, a % b)
        }
    }

    fn least_common_multiple(lengths: &[u64]) -> u64 {
        if lengths.len() == 1 {
            lengths[0]
        } else {
            let a = lengths[0];
            let b = least_common_multiple(&lengths[1..]);
            a * b / greatest_common_divisor(a, b)
        }
    }

    Ok(least_common_multiple(&lengths))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

    #[test]
    fn example_answer() {
        let mut lines = EXAMPLE.lines();
        let answer = answer(&mut lines).unwrap();
        assert_eq!(answer, 6);
    }
}
