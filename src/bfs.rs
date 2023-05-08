use std::collections::{HashMap, HashSet, VecDeque};

pub fn bfs(start: i32, end: i32, graph: &HashMap<i32, HashSet<i32>>) -> Option<i32> {
    let mut dist: HashMap<i32, i32> = graph.keys().map(|v| (*v, -1)).collect();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    dist.insert(start, 0);

    while let Some(u) = queue.pop_front() {
        if u == end {
            return Some(dist[&u]);
        }
        for &v in graph.get(&u).unwrap() {
            if dist[&v] == -1 {
                dist.insert(v, dist[&u] + 1);
                queue.push_back(v);
            }
        }
    }

    None
}