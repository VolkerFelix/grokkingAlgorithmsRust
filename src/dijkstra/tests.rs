#[cfg(test)]

use crate::graph::graph::{Graph, Node};
use crate::dijkstra::dijkstra::dijkstra;

#[test]
fn dijkstra_algo() {
    let johannes = Node::new("Johannes", true);
    let marcel = Node::new("Marcel", false);
    let mut timo = Node::new("Timo", false);
    timo.add_edge(Some(10), &johannes);
    timo.add_edge(Some(20), &marcel);
    let tobias = Node::new("Tobias", false);
    let mut me = Node::new("Volker", false);
    me.add_edge(Some(5), &tobias);
    me.add_edge(Some(7), &timo);

    let mut graph = Graph::default();
    graph.m_nodes.push(&me);
    graph.m_nodes.push(&timo);
    graph.m_nodes.push(&tobias);
    graph.m_nodes.push(&johannes);
    graph.m_nodes.push(&marcel);

    let test = dijkstra(&graph);
}

