use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use rayon::prelude::*;
use std::collections::HashMap;

/// Article Rank 파라미터
pub struct ArticleRankConfig {
    pub damping: f64,
    pub max_iterations: usize,
    pub tolerance: f64,
}

impl Default for ArticleRankConfig {
    fn default() -> Self {
        Self {
            damping: 0.85,
            max_iterations: 20,
            tolerance: 1e-6,
        }
    }
}

/// Article Rank: PageRank 변형.
/// 분모에 평균 나가는 차수를 더해 저차수 노드의 영향력을 낮춘다.
/// AR(v) = (1-d)/N + d * Σ_{u∈In(v)} AR(u) / (outDeg(u) + avgOutDeg)
pub fn article_rank(
    graph: &CompactGraph,
    config: &ArticleRankConfig,
) -> Result<HashMap<u128, f64>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(HashMap::new());
    }

    let d = config.damping;
    let base = (1.0 - d) / n as f64;
    let avg_out_degree = graph.edge_count() as f64 / n as f64;

    let mut scores = vec![1.0 / n as f64; n];
    let mut new_scores = vec![0.0f64; n];

    for _ in 0..config.max_iterations {
        new_scores
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, new_score)| {
                let mut sum = 0.0;
                for &src in graph.in_neighbors(i) {
                    let denom = graph.out_degree(src) as f64 + avg_out_degree;
                    if denom > 0.0 {
                        sum += scores[src] / denom;
                    }
                }
                *new_score = base + d * sum;
            });

        let diff: f64 = scores
            .iter()
            .zip(new_scores.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();

        std::mem::swap(&mut scores, &mut new_scores);

        if diff < config.tolerance {
            break;
        }
    }

    let mut result = HashMap::with_capacity(n);
    for (idx, &score) in scores.iter().enumerate() {
        result.insert(graph.to_node_id(idx), score);
    }
    Ok(result)
}
