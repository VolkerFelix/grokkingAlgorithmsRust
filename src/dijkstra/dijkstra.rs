use std::collections::HashMap;
use crate::graph::graph::{Graph, Node};

pub type DijkstraResult = Result<String, &'static str>;

pub fn dijkstra<'a>(f_graph: &'a Graph) -> DijkstraResult {
    let mut costs: HashMap<&Node, usize> = HashMap::new();
    let mut parents: HashMap<&Node, &Node> = HashMap::new();
    let mut already_processed: Vec<&Node> = Vec::new();

    // Init
    let start = f_graph.m_nodes.first().unwrap();
    for edge in &start.m_connections {
        costs.insert(edge.m_connects, edge.m_weight.unwrap());
        parents.insert(edge.m_connects, start);
    }

    let mut lowest_weight_node = find_lowest_weight_node(&costs, &already_processed);

    while lowest_weight_node.is_some() {
        let current_node = lowest_weight_node.unwrap();
        // Get all neighbours
        for neighbour in &current_node.m_connections {
            // Get cost to get to current node
            let current_node_cost = costs.get(current_node).unwrap();
            // Calculate new total cost to get to neighbour
            let new_cost = current_node_cost + neighbour.m_weight.unwrap();
            let neighbour_node = neighbour.m_connects;
            // Has this node been discovered before?
            // If not, insert to costs and parents.
            if !costs.contains_key(neighbour_node) {
                costs.insert(neighbour_node, new_cost);
                parents.insert(neighbour_node, current_node);
            }
            // Compare with existing costs
            if costs.get(neighbour_node).unwrap() > &new_cost {
                // Found a cheaper way! --> Update costs and parent
                costs.insert(neighbour_node, new_cost);
                parents.insert(neighbour_node, current_node);
            }
        }
        already_processed.push(current_node);
        lowest_weight_node = find_lowest_weight_node(&costs, &already_processed);
    }

    create_shortest_path(&parents)
}

fn find_lowest_weight_node<'a>(f_node_map: &HashMap<&'a Node, usize>, f_already_processed: &Vec<&Node>) -> Option<&'a Node<'a>> {
    let mut lowest_cost = usize::MAX;
    let mut lowest_cost_node = None;
    for (node, weight) in f_node_map {
        if weight < &lowest_cost && !f_already_processed.contains(&node) {
            lowest_cost = *weight;
            lowest_cost_node = Some(*node);
        }
    }
    lowest_cost_node
}

fn create_shortest_path(f_parents: &HashMap<&Node, &Node>) -> DijkstraResult {
    // Find finish node
    let mut finish_node: Option<&Node> = None;
    for (child, parent) in f_parents {
        match child.m_finish {
            Some(finish) => {
                if finish {
                    finish_node = Some(child);
                    break;
                }
            },
            None => ()
        };
        match parent.m_finish {
            Some(finish) => {
                if finish {
                    finish_node = Some(parent);
                    break;
                }
            },
            None => ()
        }
    }
    if finish_node.is_none() {
        return Err("No finish node found!");
    }
    // Now loop backward until root node is found
    let mut child = finish_node.unwrap();
    let mut shortest_path = child.m_name.to_string();
    loop {
        let parent = f_parents.get(child).unwrap();
        shortest_path.push_str(parent.m_name);
        match parent.m_root {
            Some(root) => {
                if root {
                    break;
                }
            },
            None => ()
        }
        child = parent;

    }
    Ok(shortest_path)
}