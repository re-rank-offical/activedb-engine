use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use rand::Rng;
use std::collections::BinaryHeap;

/// Influence Maximization 파라미터
pub struct InfluenceConfig {
    /// 선택할 시드 노드 수
    pub k: usize,
    /// Independent Cascade 전파 확률
    pub propagation_prob: f64,
    /// 영향력 추정용 몬테카를로 시뮬레이션 횟수
    pub mc_simulations: usize,
}

impl Default for InfluenceConfig {
    fn default() -> Self {
        Self {
            k: 5,
            propagation_prob: 0.1,
            mc_simulations: 100,
        }
    }
}

/// 선택된 시드 1개 결과: (노드 ID, 한계 영향력 증가분)
pub struct SeedResult {
    pub node_id: u128,
    pub marginal_gain: f64,
}

/// CELF 우선순위 큐용 엔트리
struct CelfEntry {
    idx: usize,
    gain: f64,
    /// 이 gain이 계산된 시점의 시드 집합 크기
    last_updated: usize,
}

impl PartialEq for CelfEntry {
    fn eq(&self, other: &Self) -> bool {
        self.gain == other.gain
    }
}
impl Eq for CelfEntry {}
impl PartialOrd for CelfEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CelfEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.gain
            .partial_cmp(&other.gain)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

/// Independent Cascade 모델로 시드 집합의 기대 전파 크기를 추정.
/// 단일 시뮬레이션: 활성 노드가 비활성 이웃을 확률 p로 활성화.
fn ic_spread_once(
    graph: &CompactGraph,
    seeds: &[usize],
    p: f64,
    active: &mut [bool],
    rng: &mut impl Rng,
) -> usize {
    for a in active.iter_mut() {
        *a = false;
    }
    let mut frontier: Vec<usize> = Vec::new();
    for &s in seeds {
        if !active[s] {
            active[s] = true;
            frontier.push(s);
        }
    }
    let mut activated = frontier.len();

    while let Some(node) = frontier.pop() {
        for &neighbor in graph.out_neighbors(node) {
            if !active[neighbor] && rng.random::<f64>() < p {
                active[neighbor] = true;
                activated += 1;
                frontier.push(neighbor);
            }
        }
    }
    activated
}

/// 시드 집합의 평균 전파 크기 (mc_simulations회 평균).
fn estimate_spread(
    graph: &CompactGraph,
    seeds: &[usize],
    p: f64,
    mc: usize,
    active: &mut [bool],
    rng: &mut impl Rng,
) -> f64 {
    if seeds.is_empty() {
        return 0.0;
    }
    let mut total = 0usize;
    for _ in 0..mc {
        total += ic_spread_once(graph, seeds, p, active, rng);
    }
    total as f64 / mc as f64
}

/// CELF(Cost-Effective Lazy Forward) 최적화 기반 Influence Maximization.
/// Greedy의 한계 영향력 단조 감소 성질을 이용해 재계산을 지연시킨다.
/// 반환: 선택 순서대로 시드와 한계 영향력, 그리고 총 기대 전파 크기.
pub fn influence_maximization(
    graph: &CompactGraph,
    config: &InfluenceConfig,
) -> Result<(Vec<SeedResult>, f64), GraphError> {
    let n = graph.node_count();
    if n == 0 || config.k == 0 {
        return Ok((Vec::new(), 0.0));
    }
    let k = config.k.min(n);
    let p = config.propagation_prob;
    let mc = config.mc_simulations.max(1);

    let mut rng = rand::rng();
    let mut active = vec![false; n];

    // 1단계: 각 노드 단독의 한계 영향력 계산 후 최대 힙 구성
    let mut heap: BinaryHeap<CelfEntry> = BinaryHeap::with_capacity(n);
    for idx in 0..n {
        let gain = estimate_spread(graph, &[idx], p, mc, &mut active, &mut rng);
        heap.push(CelfEntry {
            idx,
            gain,
            last_updated: 0,
        });
    }

    let mut seeds: Vec<usize> = Vec::with_capacity(k);
    let mut results: Vec<SeedResult> = Vec::with_capacity(k);
    let mut current_spread = 0.0;

    // 2단계: CELF lazy greedy
    while results.len() < k {
        let Some(top) = heap.pop() else { break };

        if top.last_updated == seeds.len() {
            // gain이 최신 상태 → 채택
            seeds.push(top.idx);
            current_spread += top.gain;
            results.push(SeedResult {
                node_id: graph.to_node_id(top.idx),
                marginal_gain: top.gain,
            });
        } else {
            // 오래된 gain → 재계산 후 다시 삽입
            seeds.push(top.idx);
            let new_spread = estimate_spread(graph, &seeds, p, mc, &mut active, &mut rng);
            seeds.pop();
            heap.push(CelfEntry {
                idx: top.idx,
                gain: new_spread - current_spread,
                last_updated: seeds.len(),
            });
        }
    }

    Ok((results, current_spread))
}
