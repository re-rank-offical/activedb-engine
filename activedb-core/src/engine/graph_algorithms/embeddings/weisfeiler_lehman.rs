use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use std::collections::HashMap;

/// Weisfeiler-Lehman 파라미터
pub struct WlConfig {
    /// WL 반복 횟수 (이웃 라벨 집계 단계 수)
    pub iterations: usize,
    /// 임베딩 차원 (feature hashing 버킷 수)
    pub dimension: usize,
}

impl Default for WlConfig {
    fn default() -> Self {
        Self {
            iterations: 3,
            dimension: 64,
        }
    }
}

/// FNV-1a 64비트 해시 (런 내 결정적).
fn fnv1a(data: &[u64]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for &v in data {
        hash ^= v;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn undirected_neighbors(graph: &CompactGraph, idx: usize) -> Vec<usize> {
    let mut neighbors: Vec<usize> = graph.out_neighbors(idx).to_vec();
    neighbors.extend_from_slice(graph.in_neighbors(idx));
    neighbors
}

fn l2_normalize(vec: &mut [f64]) {
    let norm: f64 = vec.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm > 1e-12 {
        for x in vec.iter_mut() {
            *x /= norm;
        }
    }
}

/// Weisfeiler-Lehman 구조적 노드 임베딩.
/// 초기 라벨(차수)에서 시작해 매 반복마다 이웃 라벨 멀티셋을 해시로 압축하고,
/// 각 단계의 라벨을 feature hashing으로 고정 차원 벡터에 누적한 뒤 L2 정규화.
pub fn weisfeiler_lehman(
    graph: &CompactGraph,
    config: &WlConfig,
) -> Result<HashMap<u128, Vec<f64>>, GraphError> {
    let n = graph.node_count();
    let dim = config.dimension.max(1);
    if n == 0 {
        return Ok(HashMap::new());
    }

    let neighbors: Vec<Vec<usize>> = (0..n).map(|i| undirected_neighbors(graph, i)).collect();

    // 초기 라벨 = 차수
    let mut labels: Vec<u64> = (0..n).map(|i| neighbors[i].len() as u64).collect();

    let mut embeddings: Vec<Vec<f64>> = vec![vec![0.0; dim]; n];

    // 초기 라벨을 임베딩에 반영
    for i in 0..n {
        let bucket = (labels[i] % dim as u64) as usize;
        embeddings[i][bucket] += 1.0;
    }

    // WL 반복
    for _ in 0..config.iterations.max(1) {
        let mut new_labels = vec![0u64; n];
        for i in 0..n {
            // 이웃 라벨을 정렬하여 멀티셋의 순서 불변성 확보
            let mut neighbor_labels: Vec<u64> = neighbors[i].iter().map(|&j| labels[j]).collect();
            neighbor_labels.sort_unstable();

            let mut data = Vec::with_capacity(neighbor_labels.len() + 1);
            data.push(labels[i]);
            data.extend_from_slice(&neighbor_labels);
            new_labels[i] = fnv1a(&data);
        }
        labels = new_labels;

        // 새 라벨을 feature hashing으로 누적
        for i in 0..n {
            let bucket = (labels[i] % dim as u64) as usize;
            embeddings[i][bucket] += 1.0;
        }
    }

    let mut result = HashMap::with_capacity(n);
    for i in 0..n {
        let mut emb = std::mem::take(&mut embeddings[i]);
        l2_normalize(&mut emb);
        result.insert(graph.to_node_id(i), emb);
    }
    Ok(result)
}
