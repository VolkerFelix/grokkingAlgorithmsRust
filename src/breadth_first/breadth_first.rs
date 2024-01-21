use std::{fmt, collections::VecDeque};

#[derive(Debug, PartialEq)]
pub struct Node<'a> {
    m_name: &'a str,
    m_connections: &'a[&'a Node<'a>],
    m_want_to_be_found: bool
}

impl<'a> Node<'a> {
    pub fn new(f_name: &'a str, f_connections: &'a [&'a Node<'a>], f_want_to_be_found: bool) -> Self {
        Node {
            m_name: f_name,
            m_connections: f_connections,
            m_want_to_be_found: f_want_to_be_found
        }
    }
}

#[derive(Default)]
pub struct Graph<'a> {
    pub m_nodes: Vec<&'a Node<'a>>,
}

impl<'a> fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = write!(f, "Name: {}\n", self.m_name);
        for connection in self.m_connections {
            ret = write!(f, "Friend: {}\n", connection.m_name);
        }
        ret
    }
}

pub fn breadth_first<'a>(f_graph: &'a Graph) -> Option<&'a Node<'a>>{
    let mut search_deque: VecDeque<&Node> = VecDeque::new();
    search_deque.push_back(f_graph.m_nodes.first().unwrap());
    let checked_nodes: Vec<Node> = Vec::new();
    while !search_deque.is_empty() {
        // Get new element from deque
        let node_to_be_checked = search_deque.pop_front().unwrap();
        // Did I come across this node already?
        if !was_node_already_checked(&node_to_be_checked, &checked_nodes) {
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

fn was_node_already_checked(f_node: &Node, f_checked_nodes: &Vec<Node>) -> bool {
    f_checked_nodes.contains(f_node)
}

