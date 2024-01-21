use std::collections::VecDeque;
use crate::graph::graph::{Graph, Node};

pub fn breadth_first<'a>(f_graph: &'a Graph) -> Option<&'a Node<'a>>{
    let mut search_deque: VecDeque<&Node> = VecDeque::new();
    search_deque.push_back(f_graph.m_nodes.first().unwrap());
    let checked_nodes: Vec<Node> = Vec::new();
    while !search_deque.is_empty() {
        // Get new element from deque
        let node_to_be_checked = search_deque.pop_front().unwrap();
        // Did I come across this node already?
        if !node_to_be_checked.already_checked(&checked_nodes) {
            if node_to_be_checked.m_want_to_be_found == true {
                return Some(node_to_be_checked);
            } else {
                // Add all connected nodes to the deque
                search_deque.extend(node_to_be_checked.m_connections.iter());
            }
        }
    }
    None
}

