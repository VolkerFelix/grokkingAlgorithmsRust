#[cfg(test)]

use crate::graph::graph::{Graph, Node};
use crate::dijkstra::dijkstra::dijkstra;

#[test]
fn dijkstra_algo() {
    let d = Node::new("D", true);
    let mut c = Node::new("C", false);
    c.add_edge(Some(5), &d);
    let mut b = Node::new("B", false);
    b.add_edge(Some(20), &d);
    b.add_edge(Some(10), &c);
    let mut a = Node::new("A", false);
    a.add_edge(Some(1), &b);
    let mut start = Node::new("Start", false);
    start.add_edge(Some(5), &a);
    start.add_edge(Some(7), &b);

    let mut graph = Graph::default();
    graph.m_nodes.push(&start);
    graph.m_nodes.push(&a);
    graph.m_nodes.push(&b);
    graph.m_nodes.push(&c);
    graph.m_nodes.push(&d);

    let _test = dijkstra(&graph);
}

