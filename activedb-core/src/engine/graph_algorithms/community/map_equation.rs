use crate::engine::{
    graph_algorithms::compact_graph::CompactGraph,
    types::GraphError,
};
use rand::Rng;
use std::collections::HashMap;

/// Map Equation(Infomap) 파라미터
pub struct MapEquationConfig {
    pub max_iterations: usize,
}

impl Default for MapEquationConfig {
    fn default() -> Self {
        Self { max_iterations: 50 }
    }
}

/// x * log2(x) (x <= 0 이면 0)
#[inline]
fn plogp(x: f64) -> f64 {
    if x > 0.0 {
        x * x.log2()
    } else {
        0.0
    }
}

/// 무방향 이웃(나가는 + 들어오는) 목록.
fn undirected_neighbors(graph: &CompactGraph, idx: usize) -> Vec<usize> {
    let mut neighbors: Vec<usize> = graph.out_neighbors(idx).to_vec();
    neighbors.extend_from_slice(graph.in_neighbors(idx));
    neighbors
}

/// 2-level Map Equation(Infomap) 커뮤니티 탐지.
/// 무방향 랜덤 워크의 정상 분포(차수 기반)를 이용한 코드 길이를 Louvain 스타일
/// 지역 이동으로 최소화한다.
///
/// 코드 길이:
/// L = plogp(Σq_i) − 2·Σ plogp(q_i) − Σ_a plogp(p_a) + Σ_i plogp(q_i + p_i)
/// (q_i = 모듈 i의 탈출 확률, p_i = 모듈 i의 방문율 합, p_a = 노드 방문율)
///
/// 반환: 노드별 모듈(커뮤니티) ID.
pub fn map_equation(
    graph: &CompactGraph,
    config: &MapEquationConfig,
) -> Result<HashMap<u128, u64>, GraphError> {
    let n = graph.node_count();
    if n == 0 {
        return Ok(HashMap::new());
    }

    let neighbors: Vec<Vec<usize>> = (0..n).map(|i| undirected_neighbors(graph, i)).collect();
    let deg: Vec<f64> = neighbors.iter().map(|nb| nb.len() as f64).collect();
    let total_degree: f64 = deg.iter().sum();

    // 엣지가 없으면 각 노드가 독립 커뮤니티
    if total_degree == 0.0 {
        let mut result = HashMap::with_capacity(n);
        for idx in 0..n {
            result.insert(graph.to_node_id(idx), idx as u64);
        }
        return Ok(result);
    }

    let d = total_degree; // = 2m
    let p_node: Vec<f64> = deg.iter().map(|&k| k / d).collect();

    // 초기: 각 노드가 자기 자신의 모듈 (singleton)
    let mut assignment: Vec<usize> = (0..n).collect();
    let mut module_cut: Vec<f64> = deg.clone(); // singleton의 cut = deg
    let mut module_p: Vec<f64> = p_node.clone();
    let mut total_cut_real: f64 = deg.iter().sum();

    let mut rng = rand::rng();
    let eps = 1e-12;

    for _ in 0..config.max_iterations.max(1) {
        let mut moved = false;

        // 무작위 순서로 노드 방문
        let mut order: Vec<usize> = (0..n).collect();
        for i in (1..n).rev() {
            let j = rng.random_range(0..=i);
            order.swap(i, j);
        }

        for &node in &order {
            if neighbors[node].is_empty() {
                continue;
            }
            let s = assignment[node];
            let deg_node = deg[node];

            // 노드의 이웃이 속한 모듈별 개수
            let mut nb_mod_count: HashMap<usize, usize> = HashMap::new();
            for &nb in &neighbors[node] {
                *nb_mod_count.entry(assignment[nb]).or_insert(0) += 1;
            }

            // 1) 현재 모듈 S에서 노드 제거
            let node_to_s = *nb_mod_count.get(&s).unwrap_or(&0) as f64;
            let cut_s_new = module_cut[s] - deg_node + 2.0 * node_to_s;
            total_cut_real += cut_s_new - module_cut[s];
            module_cut[s] = cut_s_new;
            module_p[s] -= p_node[node];

            // 격리 상태 기준값 (노드가 singleton)
            let total_cut_iso = total_cut_real + deg_node;
            let q_node = deg_node / d;
            let total_q_iso = total_cut_iso / d;
            let plp_total_iso = plogp(total_q_iso);
            let plp_q_node = plogp(q_node);
            let plp_qp_node = plogp(q_node + p_node[node]);

            // 2) 후보 모듈 평가 (이웃 모듈 + 원래 모듈 S)
            let mut best_target = s;
            let mut best_delta = f64::INFINITY;

            let mut candidates: Vec<usize> = nb_mod_count.keys().copied().collect();
            if !candidates.contains(&s) {
                candidates.push(s);
            }

            for &t in &candidates {
                let node_to_t = *nb_mod_count.get(&t).unwrap_or(&0) as f64;
                let cut_t_old = module_cut[t];
                let cut_t_new = cut_t_old + deg_node - 2.0 * node_to_t;
                let p_t_old = module_p[t];
                let p_t_new = p_t_old + p_node[node];

                let q_t_old = cut_t_old / d;
                let q_t_new = cut_t_new / d;
                let total_cut_merge = total_cut_real + deg_node - 2.0 * node_to_t;
                let total_q_merge = total_cut_merge / d;

                let delta = (plogp(total_q_merge) - plp_total_iso)
                    - 2.0 * (plogp(q_t_new) - plp_q_node - plogp(q_t_old))
                    + (plogp(q_t_new + p_t_new) - plp_qp_node - plogp(q_t_old + p_t_old));

                if delta < best_delta {
                    best_delta = delta;
                    best_target = t;
                }
            }

            // 3) 최적 모듈에 노드 삽입
            let t = best_target;
            let node_to_t = *nb_mod_count.get(&t).unwrap_or(&0) as f64;
            let cut_t_new = module_cut[t] + deg_node - 2.0 * node_to_t;
            total_cut_real += cut_t_new - module_cut[t];
            module_cut[t] = cut_t_new;
            module_p[t] += p_node[node];
            assignment[node] = t;

            if t != s && best_delta < -eps {
                moved = true;
            }
        }

        if !moved {
            break;
        }
    }

    // 모듈 ID를 0부터 연속으로 재번호화
    let mut remap: HashMap<usize, u64> = HashMap::new();
    let mut next_id = 0u64;
    let mut result = HashMap::with_capacity(n);
    for idx in 0..n {
        let module = assignment[idx];
        let id = *remap.entry(module).or_insert_with(|| {
            let id = next_id;
            next_id += 1;
            id
        });
        result.insert(graph.to_node_id(idx), id);
    }

    Ok(result)
}
