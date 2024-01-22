use std::collections::HashMap;
use crate::graph::graph::{Graph, Node};

pub fn dijkstra<'a, 'b>(f_graph: &'a Graph) -> Option<Graph<'b>> {
    let mut costs: HashMap<&Node, usize> = HashMap::new();
    let mut parents: HashMap<&Node, &Node> = HashMap::new();
    let mut already_processed: Vec<&Node> = Vec::new();

    // Init
    let start = f_graph.m_nodes.first().unwrap();
    for edge in &start.m_connections {
        costs.insert(edge.m_connects, edge.m_weight.unwrap());
        parents.insert(&start, edge.m_connects);
    }
    let temp_costs = costs.clone();

    let mut lowest_weight_node = find_lowest_weight_node(&costs, &already_processed);

    while lowest_weight_node.is_some() {
        let current_node = lowest_weight_node.unwrap();
        // Get all neighbours
        for neighbour in &current_node.m_connections {
            // Get cost to get to current node
            let current_node_cost = costs.get(current_node).unwrap();
            // Calculate new total cost to get to this neighbour
            let new_cost = current_node_cost + neighbour.m_weight.unwrap();
            // Compare with existing costs
            if costs.get(neighbour.m_connects).unwrap() > &new_cost {
                // Found a cheaper way! --> Update costs and parent
                costs.insert(neighbour.m_connects, new_cost);
                parents.insert(neighbour.m_connects, current_node);
            }
        }
        already_processed.push(current_node);
        lowest_weight_node = find_lowest_weight_node(&costs, &already_processed);
    }
    for (child, parent) in parents.iter() {
        println!("Final result: ");
        println!("Child: {}, Parent: {}", child.m_name, parent.m_name);
    }
    None
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