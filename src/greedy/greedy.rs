use std::collections::{HashMap, HashSet};

/// Greedy approximation algo
/// 1. Pick radio station that covers the most states that haven't been covered yet. Overlaps are ok.
/// 2. Repeat until all states are covered
pub fn greedy(f_radio_states_map: &HashMap<&str,&HashSet<&str>>, f_states_needed: &HashSet<&str>) -> Result<Vec<String>, &'static str> {
    let mut still_needed_states = f_states_needed.clone();
    let mut final_stations = Vec::new();

    while !still_needed_states.is_empty() {
        // Set of all states this station covers that haven't been covered yet.
        let mut states_covered = HashSet::new();
        let mut best_station = None;
        for (station, states) in f_radio_states_map.into_iter() {
            let potential_coverage: HashSet<&str> = still_needed_states.intersection(states).cloned().collect();
            if potential_coverage.len() > states_covered.len() {
                states_covered = potential_coverage;
                best_station = Some(station);
            }
        }
        if best_station.is_none() {
            return Err("Could not terminate - No best station found.");
        }
        final_stations.push(best_station.unwrap().to_string());
        still_needed_states = still_needed_states.difference(&states_covered).cloned().collect();
    }
    Ok(final_stations)
}