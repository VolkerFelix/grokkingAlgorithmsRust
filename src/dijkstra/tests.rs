#[cfg(test)]

use crate::graph::graph::{Graph, Node};
use crate::dijkstra::dijkstra::dijkstra;

#[test]
fn dijkstra_algo() {
    let echo = Node::new("Echo", None, None, Some(true));
    let mut delta = Node::new("Delta", None, None, None);
    delta.add_edge(Some(5), &echo);
    let mut charlie = Node::new("Charlie", None, None, None);
    charlie.add_edge(Some(20), &echo);
    charlie.add_edge(Some(10), &delta);
    let mut bravo = Node::new("Bravo", None, None, None);
    bravo.add_edge(Some(1), &charlie);
    let mut alpha = Node::new("Alpha", None, Some(true), None);
    alpha.add_edge(Some(5), &bravo);
    alpha.add_edge(Some(7), &charlie);

    let mut graph = Graph::default();
    graph.m_nodes.push(&alpha);
    graph.m_nodes.push(&bravo);
    graph.m_nodes.push(&charlie);
    graph.m_nodes.push(&delta);
    graph.m_nodes.push(&echo);

    let _test = dijkstra(&graph);
}

