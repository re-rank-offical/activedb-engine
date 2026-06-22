use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::HashSet;

/// Jaccard 링크 예측: 비연결 노드 쌍에 대해 Jaccard 계수를 계산.
/// score(a, b) = |N(a) ∩ N(b)| / |N(a) ∪ N(b)|
pub fn jaccard_prediction_all(
    graph: &CompactGraph,
    top_k: usize,
) -> Result<Vec<(u128, u128, f64)>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(Vec::new());
    }

    let neighbors: Vec<HashSet<usize>> = (0..n)
        .map(|i| graph.out_neighbors(i).iter().copied().collect())
        .collect();

    let mut scores: Vec<(u128, u128, f64)> = Vec::new();

    for a in 0..n {
        for b in (a + 1)..n {
            // 이미 연결된 쌍은 제외
            if neighbors[a].contains(&b) {
                continue;
            }
            let intersection = neighbors[a].intersection(&neighbors[b]).count();
            if intersection == 0 {
                continue;
            }
            let union = neighbors[a].len() + neighbors[b].len() - intersection;
            if union == 0 {
                continue;
            }
            let score = intersection as f64 / union as f64;
            scores.push((graph.to_node_id(a), graph.to_node_id(b), score));
        }
    }

    scores.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    scores.truncate(top_k);
    Ok(scores)
}
