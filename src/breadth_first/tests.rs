#[cfg(test)]

use crate::breadth_first::breadth_first;
use crate::breadth_first::breadth_first::Graph;

#[test]
fn test_node_graph_creation_and_fmt() {
    let timo = breadth_first::Node::new("Timo", &[], false);
    let tobias = breadth_first::Node::new("Tobias", &[], true);
    let friends = [&timo, &tobias];
    let me = breadth_first::Node::new("Volker", &friends, false);

    let mut graph = breadth_first::Graph::default();
    graph.m_nodes.push(&timo);
    graph.m_nodes.push(&tobias);
    graph.m_nodes.push(&me);

    for node in graph.m_nodes {
        println!("Node content:\n{}", node);
    }
}

