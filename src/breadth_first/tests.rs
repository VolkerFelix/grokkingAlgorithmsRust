#[cfg(test)]

use crate::breadth_first::breadth_first::breadth_first;
use crate::graph::graph::{Graph, Node};

#[test]
fn breadth_first_algo_found() {
    let echo = Node::new("Echo", Some(true), None, None);
    let delta = Node::new("Delta", Some(false), None, None);
    let mut charlie = Node::new("Charlie", Some(false), None, None);
    charlie.add_edge(None, &echo);
    charlie.add_edge(None, &delta);
    let bravo = Node::new("Bravo", Some(false), None, None);
    let mut alpha = Node::new("Alpha", Some(false), None, None);
    alpha.add_edge(None, &bravo);
    alpha.add_edge(None, &charlie);

    let mut graph = Graph::default();
    graph.m_nodes.push(echo);
    graph.m_nodes.push(delta);
    graph.m_nodes.push(charlie);
    graph.m_nodes.push(bravo);
    graph.m_nodes.push(alpha);

    let found = breadth_first(&graph).unwrap();
    assert_eq!(found, &echo);
}

#[test]
fn breadth_first_algo_not_found() {
    let echo = Node::new("Echo", Some(false), None, None);
    let delta = Node::new("Delta", Some(false), None, None);
    let mut charlie = Node::new("Charlie", Some(false), None, None);
    charlie.add_edge(None, &echo);
    charlie.add_edge(None, &delta);
    let bravo = Node::new("Bravo", Some(false), None, None);
    let mut alpha = Node::new("Alpha", Some(false), None, None);
    alpha.add_edge(None, &bravo);
    alpha.add_edge(None, &charlie);

    let mut graph = Graph::default();
    graph.m_nodes.push(echo);
    graph.m_nodes.push(delta);
    graph.m_nodes.push(charlie);
    graph.m_nodes.push(bravo);
    graph.m_nodes.push(alpha);

    let found = breadth_first(&graph);
    assert_eq!(found, None);
}

