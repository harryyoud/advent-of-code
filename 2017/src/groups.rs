use std::collections::HashSet;
use std::hash::Hash;

use pathfinding::directed::dijkstra::dijkstra_all;

// We will visit each node in turn, calculate the group members,
// then remove them from the visit list
// When the visit list is empty, we have seen all the groups
pub fn count_groups<T: Eq + PartialEq + Hash + Clone>(mut to_visit: HashSet<T>, neighbours: impl Fn(&T) -> Vec<T>) -> usize {
    let mut group_count = 0;

    while let Some(x) = to_visit.iter().next().cloned() {
        group_count += 1;
        let connections = dijkstra_all(
            &x,
            |a| neighbours(a).into_iter().map(|b| (b, 1))
        );
        for point in connections.keys() {
            assert!(to_visit.remove(point));
        }
        to_visit.remove(&x);
    }

    group_count
}
