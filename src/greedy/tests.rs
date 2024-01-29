#[cfg(test)]

use crate::greedy::greedy::greedy;

#[test]
fn test_greedy() {
    use std::collections::{HashMap, HashSet};

    let mut radio_state_map: HashMap<&str, &HashSet<&str>> = HashMap::new();
    let mut kone_states = HashSet::new();
    kone_states.insert("ID");
    kone_states.insert("NV");
    kone_states.insert("UT");
    radio_state_map.insert("KONE", &kone_states);
    let mut ktwo_states = HashSet::new();
    ktwo_states.insert("WA");
    ktwo_states.insert("ID");
    ktwo_states.insert("MT");
    radio_state_map.insert("KTWO", &ktwo_states);
    let mut kthree_states = HashSet::new();
    kthree_states.insert("OR");
    kthree_states.insert("NV");
    kthree_states.insert("CA");
    radio_state_map.insert("KTHREE", &kthree_states);
    let mut kfour_states = HashSet::new();
    kfour_states.insert("NV");
    kfour_states.insert("UT");
    radio_state_map.insert("KFOUR", &kfour_states);
    let mut kfive_states = HashSet::new();
    kfive_states.insert("CA");
    kfive_states.insert("AZ");
    radio_state_map.insert("KFIVE", &kfive_states);

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

    let res = greedy(&radio_state_map, &needed_states);
    // We need 4 out of the 5 stations. The stations can be different each run.
    assert_eq!(4, res.unwrap().len());
}