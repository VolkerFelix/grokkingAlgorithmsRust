mod binary_search;
mod selection_sort;
mod quick_sort;
mod breadth_first;
mod graph;
mod dijkstra;
mod greedy;

use std::collections::{HashMap, HashSet};

use crate::graph::graph::{Graph, Node};
use crate::dijkstra::dijkstra::DijkstraResult;

/// Chapter 1
/// Use binary search to find an element in a sorted array.
/// Complexity: O(log(n))
pub fn find_element_in_sorted_array(f_sorted_array: &[i32], f_number: i32) -> Option<usize> {
    binary_search::binary_search::get_index_of_number(f_sorted_array, f_number)
}
/// Chapter 2
/// Use selection sort to sort an unsorted array.
/// Complexity: O(n^2)
pub fn get_sorted_array_slowly(f_unsorted_array: &[i32]) -> Vec<i32> {
    selection_sort::selection_sort::selection_sort(f_unsorted_array)
}
/// Chapter 4
/// Use quick sort to sort and unsorted array.
/// Complexity: O(n*log(n))
pub fn sort_quickly(f_array: &mut[i32]) {
    quick_sort::quick_sort::quick_sort(f_array)
}
/// Chapter 6
/// Use breadth-first to find a certain node starting from the root in an unweighted graph.
/// Define one node in your graph with "m_want_to_be_found=true"
/// Complexity: O(V+E)
pub fn find_node_in_graph<'a>(f_graph: &'a Graph) -> Option<&'a Node<'a>> {
    breadth_first::breadth_first::breadth_first(f_graph)
}
/// Chapter 7
/// Use Dijkstra to find the fastest way from root to finish node in a weighted graph.
/// Complexity: O(V^2)
pub fn find_fastest_way_to_finish<'a>(f_graph: &'a Graph) -> DijkstraResult {
    dijkstra::dijkstra::dijkstra(f_graph)
}
/// Chapter 8
/// Use greedy so solve NP-complete problems.
/// E.g. find the fewest radio stations needed to cover the requested states.
/// The solution is an approximation and likely not the optimal one.
/// Complexity: O(n^2) compared to optimal solution with O(2^n)
pub fn find_min_amount_of_radio_stations(f_radio_states_map: &HashMap<&str,&HashSet<&str>>, f_states_needed: &HashSet<&str>) -> Result<Vec<String>, &'static str> {
    greedy::greedy::greedy(f_radio_states_map, f_states_needed)
}