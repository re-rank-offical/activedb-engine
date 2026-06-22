use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use rand::Rng;
use std::collections::{HashMap, HashSet};

/// Node2Vec 파라미터
pub struct Node2VecConfig {
    pub dimension: usize,
    pub walk_length: usize,
    pub num_walks: usize,
    pub window: usize,
    /// return 파라미터 (작을수록 되돌아오기 선호)
    pub p: f64,
    /// in-out 파라미터 (작을수록 멀리 탐색)
    pub q: f64,
}

impl Default for Node2VecConfig {
    fn default() -> Self {
        Self {
            dimension: 64,
            walk_length: 20,
            num_walks: 10,
            window: 5,
            p: 1.0,
            q: 1.0,
        }
    }
}

fn undirected_neighbor_list(graph: &CompactGraph, idx: usize) -> Vec<usize> {
    let mut set: HashSet<usize> = graph.out_neighbors(idx).iter().copied().collect();
    set.extend(graph.in_neighbors(idx).iter().copied());
    set.remove(&idx);
    set.into_iter().collect()
}

fn l2_normalize(vec: &mut [f64]) {
    let norm: f64 = vec.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm > 1e-12 {
        for x in vec.iter_mut() {
            *x /= norm;
        }
    }
}

/// 편향 랜덤 워크 1회 생성 (2차 마르코프, p/q 편향).
fn biased_walk(
    start: usize,
    length: usize,
    adj: &[Vec<usize>],
    adj_set: &[HashSet<usize>],
    p: f64,
    q: f64,
    rng: &mut impl Rng,
) -> Vec<usize> {
    let mut walk = Vec::with_capacity(length);
    walk.push(start);
    if adj[start].is_empty() {
        return walk;
    }
    // 첫 스텝: 균등 선택
    walk.push(adj[start][rng.random_range(0..adj[start].len())]);

    while walk.len() < length {
        let cur = *walk.last().unwrap();
        let prev = walk[walk.len() - 2];
        let candidates = &adj[cur];
        if candidates.is_empty() {
            break;
        }

        // 비정규화 가중치 계산
        let mut weights = Vec::with_capacity(candidates.len());
        let mut total = 0.0;
        for &x in candidates {
            let w = if x == prev {
                1.0 / p
            } else if adj_set[prev].contains(&x) {
                1.0
            } else {
                1.0 / q
            };
            weights.push(w);
            total += w;
        }

        // 누적 분포에서 샘플링
        let target = rng.random::<f64>() * total;
        let mut acc = 0.0;
        let mut next = candidates[candidates.len() - 1];
        for (i, &w) in weights.iter().enumerate() {
            acc += w;
            if acc >= target {
                next = candidates[i];
                break;
            }
        }
        walk.push(next);
    }
    walk
}

/// Node2Vec 노드 임베딩 (DeepWalk 스타일 간소화).
/// 편향 랜덤 워크 → 윈도우 내 동시출현(co-occurrence) → PPMI 행렬 →
/// 희소 랜덤 투영으로 차원 축소 → L2 정규화.
pub fn node2vec(
    graph: &CompactGraph,
    config: &Node2VecConfig,
) -> Result<HashMap<u128, Vec<f64>>, GraphError> {
    let n = graph.node_count();
    let dim = config.dimension.max(1);
    if n == 0 {
        return Ok(HashMap::new());
    }

    let adj: Vec<Vec<usize>> = (0..n).map(|i| undirected_neighbor_list(graph, i)).collect();
    let adj_set: Vec<HashSet<usize>> = adj.iter().map(|v| v.iter().copied().collect()).collect();

    let mut rng = rand::rng();
    let window = config.window.max(1);

    // 동시출현 횟수 (노드별 희소 행)
    let mut cooc: Vec<HashMap<usize, f64>> = vec![HashMap::new(); n];

    for _ in 0..config.num_walks.max(1) {
        for start in 0..n {
            if adj[start].is_empty() {
                continue;
            }
            let walk = biased_walk(
                start,
                config.walk_length.max(2),
                &adj,
                &adj_set,
                config.p,
                config.q,
                &mut rng,
            );
            // 윈도우 내 (center, context) 쌍 집계 (양방향)
            for i in 0..walk.len() {
                let lo = i.saturating_sub(window);
                let hi = (i + window + 1).min(walk.len());
                for j in lo..hi {
                    if j == i {
                        continue;
                    }
                    *cooc[walk[i]].entry(walk[j]).or_insert(0.0) += 1.0;
                }
            }
        }
    }

    // PPMI 계산을 위한 합계
    let row_sum: Vec<f64> = cooc.iter().map(|r| r.values().sum()).collect();
    let total: f64 = row_sum.iter().sum();
    // 무방향 동시출현이므로 열 합 = 행 합
    let col_sum = &row_sum;

    if total <= 0.0 {
        // 워크가 전혀 없으면 0 임베딩
        let mut result = HashMap::with_capacity(n);
        for i in 0..n {
            result.insert(graph.to_node_id(i), vec![0.0; dim]);
        }
        return Ok(result);
    }

    // 희소 랜덤 투영 행렬 R (n × dim)
    let s = (n as f64).sqrt().max(1.0);
    let value = s.sqrt();
    let prob = 1.0 / (2.0 * s);
    let projection: Vec<Vec<f64>> = (0..n)
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

    // 임베딩[i] = Σ_j PPMI(i,j) * R[j]
    let mut result = HashMap::with_capacity(n);
    for i in 0..n {
        let mut emb = vec![0.0f64; dim];
        if row_sum[i] > 0.0 {
            for (&j, &count) in &cooc[i] {
                if col_sum[j] <= 0.0 {
                    continue;
                }
                let pmi = ((count * total) / (row_sum[i] * col_sum[j])).log2();
                let ppmi = pmi.max(0.0);
                if ppmi > 0.0 {
                    for d in 0..dim {
                        emb[d] += ppmi * projection[j][d];
                    }
                }
            }
        }
        l2_normalize(&mut emb);
        result.insert(graph.to_node_id(i), emb);
    }

    Ok(result)
}
