use std::{fmt, collections::VecDeque};
use std::iter::Extend;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct Node<'a> {
    pub m_name: &'a str,
    pub m_connections: Vec<Edge<'a>>,
    pub m_want_to_be_found: Option<bool>,
    pub m_root: Option<bool>,
    pub m_finish: Option<bool>
}

impl<'a> Node<'a> {
    pub fn new(f_name: &'a str, f_want_to_be_found: Option<bool>, f_root: Option<bool>, f_finish: Option<bool>) -> Self {
        Node {
            m_name: f_name,
            m_connections: Vec::new(),
            m_want_to_be_found: f_want_to_be_found,
            m_root: f_root,
            m_finish: f_finish
        }
    }

    pub fn already_checked(&self, f_checked_nodes: &Vec<Node>) -> bool {
        f_checked_nodes.contains(self)
    }

    pub fn add_edge(&mut self, f_weight: Option<usize>, f_connects: &'a Node<'a>) {
        let new_edge = Edge::new(f_weight, f_connects);
        self.m_connections.push(new_edge);
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct Edge<'a> {
    pub m_weight: Option<usize>,
    pub m_connects: &'a Node<'a>,
}

impl<'a> Edge<'a> {
    fn new(f_weight: Option<usize>, f_dest: &'a Node<'a>) -> Self {
        Edge {
            m_weight: f_weight,
            m_connects: f_dest
        }
    }
}

#[derive(Default)]
pub struct Graph<'a> {
    pub m_nodes: Vec<&'a Node<'a>>,
    pub m_root_set: Option<bool>,
    pub m_finish_set: Option<bool>,
}

impl<'a> fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}\n", self.m_name)
    }
}

impl<'a> Extend<&'a Edge<'a>> for VecDeque<&Node<'a>> {
    fn extend<T: IntoIterator<Item=&'a Edge<'a>>>(&mut self, iter: T) {
        for element in iter {
            self.push_back(element.m_connects);
        }
    }
}