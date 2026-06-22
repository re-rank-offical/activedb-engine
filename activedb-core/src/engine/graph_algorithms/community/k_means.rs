use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use rand::Rng;
use std::collections::{HashMap, HashSet};

/// 그래프 K-Means 파라미터
pub struct KMeansConfig {
    /// 클러스터 수
    pub k: usize,
    /// Lloyd 반복 횟수
    pub max_iterations: usize,
}

impl Default for KMeansConfig {
    fn default() -> Self {
        Self {
            k: 4,
            max_iterations: 50,
        }
    }
}

/// 노드별 구조적 특징 벡터를 만든다.
/// [전체 차수, 나가는 차수, 들어오는 차수, 지역 클러스터링 계수]
fn build_features(graph: &CompactGraph) -> Vec<[f64; 4]> {
    let n = graph.node_count();

    // 무방향 이웃 집합
    let neighbor_sets: Vec<HashSet<usize>> = (0..n)
        .map(|i| {
            let mut s: HashSet<usize> = graph.out_neighbors(i).iter().copied().collect();
            s.extend(graph.in_neighbors(i).iter().copied());
            s.remove(&i);
            s
        })
        .collect();

    (0..n)
        .map(|i| {
            let total = neighbor_sets[i].len() as f64;
            let out_d = graph.out_degree(i) as f64;
            let in_d = graph.in_degree(i) as f64;

            // 지역 클러스터링 계수
            let neighbors: Vec<usize> = neighbor_sets[i].iter().copied().collect();
            let kk = neighbors.len();
            let cc = if kk < 2 {
                0.0
            } else {
                let mut links = 0usize;
                for a in 0..kk {
                    for b in (a + 1)..kk {
                        if neighbor_sets[neighbors[a]].contains(&neighbors[b]) {
                            links += 1;
                        }
                    }
                }
                2.0 * links as f64 / (kk as f64 * (kk as f64 - 1.0))
            };

            [total, out_d, in_d, cc]
        })
        .collect()
}

/// 특징을 z-score로 표준화 (열 단위).
fn standardize(features: &mut [[f64; 4]]) {
    let n = features.len();
    if n == 0 {
        return;
    }
    for col in 0..4 {
        let mean: f64 = features.iter().map(|f| f[col]).sum::<f64>() / n as f64;
        let var: f64 = features.iter().map(|f| (f[col] - mean).powi(2)).sum::<f64>() / n as f64;
        let std = var.sqrt();
        if std > 1e-12 {
            for f in features.iter_mut() {
                f[col] = (f[col] - mean) / std;
            }
        } else {
            for f in features.iter_mut() {
                f[col] = 0.0;
            }
        }
    }
}

#[inline]
fn sq_dist(a: &[f64; 4], b: &[f64; 4]) -> f64 {
    (0..4).map(|i| (a[i] - b[i]).powi(2)).sum()
}

/// 구조적 특징 기반 K-Means 노드 클러스터링.
/// 특징: [전체 차수, 나가는 차수, 들어오는 차수, 지역 클러스터링 계수] → z-score 표준화 →
/// k-means++ 초기화 → Lloyd 반복.
/// 반환: 노드별 클러스터 ID.
pub fn k_means(
    graph: &CompactGraph,
    config: &KMeansConfig,
) -> Result<HashMap<u128, u64>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(HashMap::new());
    }
    let k = config.k.clamp(1, n);

    let mut features = build_features(graph);
    standardize(&mut features);

    let mut rng = rand::rng();

    // k-means++ 초기 중심 선택
    let mut centroids: Vec<[f64; 4]> = Vec::with_capacity(k);
    centroids.push(features[rng.random_range(0..n)]);
    while centroids.len() < k {
        let dists: Vec<f64> = features
            .iter()
            .map(|f| {
                centroids
                    .iter()
                    .map(|c| sq_dist(f, c))
                    .fold(f64::INFINITY, f64::min)
            })
            .collect();
        let sum: f64 = dists.iter().sum();
        if sum <= 1e-12 {
            // 모든 점이 동일 → 임의 선택
            centroids.push(features[rng.random_range(0..n)]);
            continue;
        }
        let target = rng.random::<f64>() * sum;
        let mut acc = 0.0;
        let mut chosen = n - 1;
        for (i, &dd) in dists.iter().enumerate() {
            acc += dd;
            if acc >= target {
                chosen = i;
                break;
            }
        }
        centroids.push(features[chosen]);
    }

    // Lloyd 반복
    let mut assignment = vec![0usize; n];
    for _ in 0..config.max_iterations.max(1) {
        let mut changed = false;

        // 할당 단계
        for i in 0..n {
            let mut best = 0;
            let mut best_dist = f64::INFINITY;
            for (c, centroid) in centroids.iter().enumerate() {
                let dd = sq_dist(&features[i], centroid);
                if dd < best_dist {
                    best_dist = dd;
                    best = c;
                }
            }
            if assignment[i] != best {
                assignment[i] = best;
                changed = true;
            }
        }

        // 갱신 단계
        let mut sums = vec![[0.0f64; 4]; k];
        let mut counts = vec![0usize; k];
        for i in 0..n {
            let c = assignment[i];
            counts[c] += 1;
            for j in 0..4 {
                sums[c][j] += features[i][j];
            }
        }
        for c in 0..k {
            if counts[c] > 0 {
                for j in 0..4 {
                    centroids[c][j] = sums[c][j] / counts[c] as f64;
                }
            }
        }

        if !changed {
            break;
        }
    }

    let mut result = HashMap::with_capacity(n);
    for i in 0..n {
        result.insert(graph.to_node_id(i), assignment[i] as u64);
    }
    Ok(result)
}
