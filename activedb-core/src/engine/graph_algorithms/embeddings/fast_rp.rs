use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use rand::Rng;
use std::collections::HashMap;

/// FastRP 파라미터
pub struct FastRpConfig {
    /// 임베딩 차원
    pub dimension: usize,
    /// 전파 반복 횟수 (이웃 집계 단계 수)
    pub iterations: usize,
}

impl Default for FastRpConfig {
    fn default() -> Self {
        Self {
            dimension: 64,
            iterations: 3,
        }
    }
}

/// 무방향 이웃 목록.
fn undirected_neighbors(graph: &CompactGraph, idx: usize) -> Vec<usize> {
    let mut neighbors: Vec<usize> = graph.out_neighbors(idx).to_vec();
    neighbors.extend_from_slice(graph.in_neighbors(idx));
    neighbors
}

/// 행 L2 정규화.
fn l2_normalize(vec: &mut [f64]) {
    let norm: f64 = vec.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm > 1e-12 {
        for x in vec.iter_mut() {
            *x /= norm;
        }
    }
}

/// FastRP(Fast Random Projection) 노드 임베딩.
/// 1) 희소 랜덤 투영으로 초기 임베딩 생성
/// 2) 이웃 평균 집계를 반복하며 각 단계 결과를 누적
/// 3) L2 정규화
/// 반환: 노드별 임베딩 벡터.
pub fn fast_rp(
    graph: &CompactGraph,
    config: &FastRpConfig,
) -> Result<HashMap<u128, Vec<f64>>, GraphError> {
    let n = graph.node_count();
    let dim = config.dimension.max(1);
    if n == 0 {
        return Ok(HashMap::new());
    }

    let mut rng = rand::rng();

    // 희소 랜덤 투영 행렬: 각 성분 +√s (1/2s 확률), -√s (1/2s 확률), 0 (1-1/s 확률)
    let s = (n as f64).sqrt().max(1.0);
    let value = s.sqrt();
    let prob = 1.0 / (2.0 * s);

    let mut current: Vec<Vec<f64>> = (0..n)
        .map(|_| {
            (0..dim)
                .map(|_| {
                    let r = rng.random::<f64>();
                    if r < prob {
                        value
                    } else if r < 2.0 * prob {
                        -value
                    } else {
                        0.0
                    }
                })
                .collect()
        })
        .collect();

    let neighbors: Vec<Vec<usize>> = (0..n).map(|i| undirected_neighbors(graph, i)).collect();

    // 누적 임베딩 (초기 투영 포함)
    let mut accumulated: Vec<Vec<f64>> = current.clone();

    // 이웃 평균 집계 반복
    for _ in 0..config.iterations.max(1) {
        let mut next: Vec<Vec<f64>> = vec![vec![0.0; dim]; n];
        for i in 0..n {
            let nb = &neighbors[i];
            if nb.is_empty() {
                continue;
            }
            let inv = 1.0 / nb.len() as f64;
            for &j in nb {
                for d in 0..dim {
                    next[i][d] += current[j][d] * inv;
                }
            }
        }
        for i in 0..n {
            for d in 0..dim {
                accumulated[i][d] += next[i][d];
            }
        }
        current = next;
    }

    let mut result = HashMap::with_capacity(n);
    for i in 0..n {
        let mut emb = std::mem::take(&mut accumulated[i]);
        l2_normalize(&mut emb);
        result.insert(graph.to_node_id(i), emb);
    }
    Ok(result)
}
