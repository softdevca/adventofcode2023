use std::collections::BTreeMap;
use std::error::Error;
use std::fs::read_to_string;
use std::io::Error as IoError;
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
    let nodes = lines
        .map(|line| Node {
            id: line[0..3].to_owned(),
            left: line[7..10].to_owned(),
            right: line[12..15].to_owned(),
        })
        .collect();
    Ok((instructions, nodes))
}

fn answer(lines: &mut Lines) -> Result<usize, Box<dyn Error>> {
    let (instructions, nodes) = parse(lines)?;
    let node_map: BTreeMap<NodeId, Node> = nodes
        .iter()
        .map(|node| (node.id.clone(), node.clone()))
        .collect();
    let mut node = node_map.get("AAA").expect("first node, 'AAA'");

    let mut path = Vec::new();
    path.push(node);

    for instruction in instructions.chars().cycle() {
        let next_node_id = match instruction {
            'L' => &node.left,
            'R' => &node.right,
            _ => return Err(IoError::other(format!("unknown instruction '{instruction}'")).into()),
        };
        if next_node_id == "ZZZ" {
            break;
        }

        node = node_map.get(next_node_id).expect("node");
        path.push(node);
    }
    Ok(path.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &'static str = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn example1_answer() {
        let mut lines = EXAMPLE1.lines();
        let answer = answer(&mut lines).unwrap();
        assert_eq!(answer, 2);
    }

    const EXAMPLE2: &'static str = " LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn example2_answer() {
        let mut lines = crate::tests::EXAMPLE2.lines();
        let answer = answer(&mut lines).unwrap();
        assert_eq!(answer, 6);
    }
}
