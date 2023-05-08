#[test]
fn test_read_txt() {
    let adj_list = read_txt("facebook.txt");

    assert_eq!(adj_list[&1], vec![2, 3].into_iter().collect());
    assert_eq!(adj_list[&2], vec![1, 3].into_iter().collect());
    assert_eq!(adj_list[&3], vec![1, 2, 4].into_iter().collect());
    assert_eq!(adj_list[&4], vec![2].into_iter().collect());
}

#[test]
fn test_read_csv() {
    let adj_list = read_csv("lastfm_asia.csv");

    assert_eq!(adj_list[&1], vec![2, 3].into_iter().collect());
    assert_eq!(adj_list[&2], vec![1, 3].into_iter().collect());
    assert_eq!(adj_list[&3], vec![1, 2, 4].into_iter().collect());
    assert_eq!(adj_list[&4], vec![2].into_iter().collect());
}

#[test]
fn test_bfs() {
    let adj_list = read_txt("facebook.txt");
    let dist = bfs(&1, &adj_list);

    assert_eq!(dist[&1], 0);
    assert_eq!(dist[&2], 1);
    assert_eq!(dist[&3], 1);
    assert_eq!(dist[&4], 2);
}

#[test]
fn test_avg_distance() {
    let adj_list = read_txt("facebook.txt");
    let avg_dist = avg_distance(&adj_list);

    assert_eq!(avg_dist, 1.25);
}
