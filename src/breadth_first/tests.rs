#[cfg(test)]

use crate::breadth_first::breadth_first::{breadth_first, Node, Graph};

#[test]
fn test_node_graph_creation_and_fmt() {
    let johannes = Node::new("Johannes", &[], true);
    let marcel = Node::new("Marcel", &[], false);
    let timos_friends = [&johannes, &marcel];
    let timo = Node::new("Timo", &timos_friends, false);
    let tobias = Node::new("Tobias", &[], false);
    let my_friends = [&timo, &tobias];
    let me = Node::new("Volker", &my_friends, false);

    let mut graph = Graph::default();
    graph.m_nodes.push(&johannes);
    graph.m_nodes.push(&marcel);
    graph.m_nodes.push(&timo);
    graph.m_nodes.push(&tobias);
    graph.m_nodes.push(&me);

    for node in graph.m_nodes {
        println!("Node content:\n{}", node);
    }
}

#[test]
fn breadth_first_algo_found() {
    let johannes = Node::new("Johannes", &[], true);
    let marcel = Node::new("Marcel", &[], false);
    let timos_friends = [&johannes, &marcel];
    let timo = Node::new("Timo", &timos_friends, false);
    let tobias = Node::new("Tobias", &[], false);
    let my_friends = [&timo, &tobias];
    let me = Node::new("Volker", &my_friends, false);

    let mut graph = Graph::default();
    graph.m_nodes.push(&johannes);
    graph.m_nodes.push(&marcel);
    graph.m_nodes.push(&timo);
    graph.m_nodes.push(&tobias);
    graph.m_nodes.push(&me);

    let found = breadth_first(&graph).unwrap();
    assert_eq!(found, &johannes);
}

#[test]
fn breadth_first_algo_not_found() {
    let johannes = Node::new("Johannes", &[], false);
    let marcel = Node::new("Marcel", &[], false);
    let timos_friends = [&johannes, &marcel];
    let timo = Node::new("Timo", &timos_friends, false);
    let tobias = Node::new("Tobias", &[], false);
    let my_friends = [&timo, &tobias];
    let me = Node::new("Volker", &my_friends, false);

    let mut graph = Graph::default();
    graph.m_nodes.push(&johannes);
    graph.m_nodes.push(&marcel);
    graph.m_nodes.push(&timo);
    graph.m_nodes.push(&tobias);
    graph.m_nodes.push(&me);

    let found = breadth_first(&graph);
    assert_eq!(found, None);
}

