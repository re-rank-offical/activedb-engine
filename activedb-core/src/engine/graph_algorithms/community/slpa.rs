use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use rand::Rng;
use std::collections::HashMap;

/// SLPA(Speaker-Listener Label Propagation) 파라미터
pub struct SlpaConfig {
    /// 반복 횟수 (각 노드의 메모리에 라벨이 쌓이는 횟수)
    pub iterations: usize,
    /// 커뮤니티 소속 판정 임계값 r (메모리 내 라벨 빈도 비율)
    pub threshold: f64,
}

impl Default for SlpaConfig {
    fn default() -> Self {
        Self {
            iterations: 100,
            threshold: 0.1,
        }
    }
}

/// 무방향 이웃(나가는 + 들어오는)을 합쳐 반환.
fn undirected_neighbors(graph: &CompactGraph, idx: usize) -> Vec<usize> {
    let mut neighbors: Vec<usize> = graph.out_neighbors(idx).to_vec();
    neighbors.extend_from_slice(graph.in_neighbors(idx));
    neighbors
}

/// SLPA: 각 노드가 라벨 메모리를 유지하며 중복 커뮤니티를 탐지.
/// Speaker는 메모리 빈도에 비례해 라벨을 발화하고, Listener는 가장 많이 들은 라벨을 메모리에 추가.
/// 반환: 노드별 소속 커뮤니티 ID 목록 (중복 가능).
pub fn slpa(
    graph: &CompactGraph,
    config: &SlpaConfig,
) -> Result<HashMap<u128, Vec<u64>>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(HashMap::new());
    }

    let mut rng = rand::rng();

    // 무방향 이웃 사전 계산
    let neighbors: Vec<Vec<usize>> = (0..n).map(|i| undirected_neighbors(graph, i)).collect();

    // 각 노드의 라벨 메모리: 초기에는 자기 자신의 라벨 1개
    let mut memory: Vec<Vec<u64>> = (0..n).map(|i| vec![i as u64]).collect();

    let iterations = config.iterations.max(1);
    for _ in 0..iterations {
        // 순서 무작위화
        let mut order: Vec<usize> = (0..n).collect();
        for i in (1..n).rev() {
            let j = rng.random_range(0..=i);
            order.swap(i, j);
        }

        for &listener in &order {
            if neighbors[listener].is_empty() {
                continue;
            }

            // 각 이웃(speaker)이 메모리에서 라벨 하나를 무작위 발화
            let mut heard: HashMap<u64, usize> = HashMap::new();
            for &speaker in &neighbors[listener] {
                let mem = &memory[speaker];
                let label = mem[rng.random_range(0..mem.len())];
                *heard.entry(label).or_insert(0) += 1;
            }

            // 가장 많이 들은 라벨을 메모리에 추가 (동률은 사전순 최소로 결정)
            if let Some((&best_label, _)) =
                heard.iter().max_by_key(|&(label, &count)| (count, std::cmp::Reverse(*label)))
            {
                memory[listener].push(best_label);
            }
        }
    }

    // 후처리: 메모리 내 빈도가 임계값 이상인 라벨만 커뮤니티로 채택
    let threshold = config.threshold;
    let mut result = HashMap::with_capacity(n);
    for idx in 0..n {
        let mem = &memory[idx];
        let total = mem.len() as f64;
        let mut counts: HashMap<u64, usize> = HashMap::new();
        for &label in mem {
            *counts.entry(label).or_insert(0) += 1;
        }

        let mut communities: Vec<u64> = counts
            .into_iter()
            .filter(|&(_, c)| (c as f64 / total) >= threshold)
            .map(|(label, _)| label)
            .collect();

        // 임계값을 넘는 라벨이 없으면 최빈 라벨 1개로 대체
        if communities.is_empty() {
            if let Some(&label) = mem.iter().max_by_key(|&&l| {
                mem.iter().filter(|&&x| x == l).count()
            }) {
                communities.push(label);
            }
        }
        communities.sort_unstable();
        result.insert(graph.to_node_id(idx), communities);
    }

    Ok(result)
}
