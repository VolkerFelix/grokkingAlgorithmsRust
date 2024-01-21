use std::{collections::HashMap, fmt::LowerExp};
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
    let lowest_weight = temp_costs.iter().min().unwrap();
    let neighbors_of_lowest_weight = &lowest_weight.0.m_connections;

    for neighbor in neighbors_of_lowest_weight {
        // Calculate new cost from start
        let new_cost = lowest_weight.1 + neighbor.m_weight.unwrap();
        // Compare the costs; If lower then update them
        if costs.get(neighbor.m_connects).unwrap() > &new_cost {
            // Found a better way!
            costs.insert(&neighbor.m_connects, new_cost);
            parents.insert(&neighbor.m_connects, lowest_weight.0);
        }
    }
    already_processed.push(lowest_weight.0);

    None
}

fn find_lowest_weight_node(f_node_map: HashMap<&Node, usize>, f_aready_processed: &Vec<&Node>) -> Option<&Node> {
    let lowest_weight_node = f_node_map.iter().min();
    match lowest_weight_node {
        None => return None,
        Some(node) => {
            // Already processed?
            // TODO: Continue here
        }
    }
}