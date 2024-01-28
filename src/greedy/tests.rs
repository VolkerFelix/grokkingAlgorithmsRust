#[cfg(test)]

use crate::greedy::greedy::greedy;

#[test]
fn test_greedy() {
    use std::collections::{HashMap, HashSet};

    let mut radio_state_map: HashMap<&str, HashSet<&str>> = HashMap::new();
    radio_state_map.insert("KONE", &["ID", "NV", "UT"]);
    radio_state_map.insert("KTWO", &["WA", "ID", "MT"]);
    radio_state_map.insert("KTHREE", &["OR", "NV", "CA"]);
    radio_state_map.insert("KFOUR", &["NV", "UT"]);
    radio_state_map.insert("KFIVE", &["CA", "AZ"]);

    // 2^n total combinations possible
    // E.g. n= 3 = 8
    // {} + KONE + KTWO + KTHREE + (KONE, KTWO) + (KONE + KTHREE) + (KTWO, KTHREE) + (KONE, KTWO, KTHREE)
    // Exact solution takes O(2^n) --> bad
    // Approximation algo takes O(n^2) --> much better

    let mut needed_states: HashSet<&str> = HashSet::new();
    needed_states.insert("ID");
    needed_states.insert("NV");
    needed_states.insert("UT");
    needed_states.insert("WA");
    needed_states.insert("MT");
    needed_states.insert("OR");
    needed_states.insert("CA");
    needed_states.insert("AZ");

    greedy(&radio_state_map, &needed_states);

}