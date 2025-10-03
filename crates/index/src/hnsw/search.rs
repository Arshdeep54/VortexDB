use std::collections::HashSet;

use defs::PointId;

use crate::distance;

use super::index::HnswIndex;

impl HnswIndex {
    /// Greedy search within a fixed layer
    /// - start from `ep` and evaluate neighbors at `level`
    /// - move to a neighbor only if it strictly improves distance
    /// - stop when no improvement and return the last id
    pub(super) fn greedy_search_layer(&self, ep: PointId, level: usize, query: &[f32]) -> PointId {
        let mut current = ep;
        loop {
            let cur_vec = self.get_vec(current);
            let mut best_score = distance(query.to_vec(), cur_vec.to_vec(), self.similarity);
            let mut best_id = current;

            let empty: &[PointId] = &[];
            let neighbors = self
                .index
                .nodes
                .get(&current)
                .and_then(|n| n.neighbors.get(level))
                .map(|v| v.as_slice())
                .unwrap_or(empty);

            for &n in neighbors {
                if n == current {
                    continue;
                }
                // Skip deleted neighbors
                if let Some(nn) = self.index.nodes.get(&n) {
                    if nn.deleted {
                        continue;
                    }
                }
                let n_vec = self.get_vec(n);
                let score = distance(query.to_vec(), n_vec.to_vec(), self.similarity);
                if score < best_score {
                    best_score = score;
                    best_id = n;
                }
            }

            if best_id == current {
                break;
            }
            current = best_id;
        }
        current
    }

    /// Best-first (ef) search used during insertion on a given layer
    /// - maintain candidate queue and working set up to `ef_construction`
    /// - expand the closest candidate; skip deleted nodes
    /// - early-exit if the best candidate is worse than the worst in W when full
    /// - return W as (id, distance) sorted by ascending distance
    pub(super) fn search_layer_for_insert(
        &self,
        ep: PointId,
        level: usize,
        query: &[f32],
        ef_construction: usize,
    ) -> Vec<(PointId, f32)> {
        let mut visited: HashSet<PointId> = HashSet::new();
        let mut candidates: Vec<(f32, PointId)> = Vec::new();
        let mut w: Vec<(f32, PointId)> = Vec::new();

        // Seed with a non-deleted entry point
        let seed = if let Some(node) = self.index.nodes.get(&ep) {
            if node.deleted {
                self.index
                    .nodes
                    .iter()
                    .filter(|(_, n)| !n.deleted && n.neighbors.len() > level)
                    .max_by(|a, b| a.1.level.cmp(&b.1.level).then_with(|| a.0.cmp(b.0)))
                    .map(|(id, _)| *id)
                    .unwrap_or(ep)
            } else {
                ep
            }
        } else {
            ep
        };
        let ep_score = distance(query.to_vec(), self.get_vec(seed).to_vec(), self.similarity);
        candidates.push((ep_score, seed));
        w.push((ep_score, seed));
        visited.insert(seed);

        while !candidates.is_empty() {
            let (best_idx, (best_score, best_id)) = candidates
                .iter()
                .enumerate()
                .min_by(|a, b| a.1 .0.partial_cmp(&b.1 .0).unwrap())
                .map(|(i, v)| (i, *v))
                .unwrap();
            candidates.swap_remove(best_idx);

            if w.len() >= ef_construction {
                if let Some((_, (worst_score, _))) = w
                    .iter()
                    .enumerate()
                    .max_by(|a, b| a.1 .0.partial_cmp(&b.1 .0).unwrap())
                {
                    if best_score > *worst_score {
                        break;
                    }
                }
            }

            let empty: &[PointId] = &[];
            let neighbors = self
                .index
                .nodes
                .get(&best_id)
                .and_then(|n| n.neighbors.get(level))
                .map(|v| v.as_slice())
                .unwrap_or(empty);
            for &n in neighbors {
                if visited.contains(&n) {
                    continue;
                }
                // Skip deleted neighbors
                if let Some(nn) = self.index.nodes.get(&n) {
                    if nn.deleted {
                        continue;
                    }
                }
                visited.insert(n);
                let score = distance(query.to_vec(), self.get_vec(n).to_vec(), self.similarity);
                candidates.push((score, n));
                if w.len() < ef_construction {
                    w.push((score, n));
                } else if let Some((worst_idx, (worst_score, _))) = w
                    .iter()
                    .enumerate()
                    .max_by(|a, b| a.1 .0.partial_cmp(&b.1 .0).unwrap())
                {
                    if score < *worst_score {
                        w[worst_idx] = (score, n);
                    }
                }
            }
        }

        let mut out: Vec<(PointId, f32)> = w.into_iter().map(|(s, id)| (id, s)).collect();
        out.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        out
    }

    /// Diversity-based neighbor selection (heuristic pruning)
    /// - sort candidates by distance-to-new ascending
    /// - accept a candidate unless it is dominated by an accepted one
    /// - return up to `m` ids
    pub(super) fn select_neighbors_heuristic(
        &self,
        candidates: &[(PointId, f32)],
        m: usize,
    ) -> Vec<PointId> {
        let mut sorted = candidates.to_vec();
        sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let mut result: Vec<PointId> = Vec::with_capacity(m);

        'outer: for &(cand_id, cand_dist_to_q) in &sorted {
            let cand_vec = self.get_vec(cand_id);
            for &r_id in &result {
                let r_vec = self.get_vec(r_id);
                let cand_to_r = distance(cand_vec.to_vec(), r_vec.to_vec(), self.similarity);
                if cand_to_r < cand_dist_to_q {
                    continue 'outer;
                }
            }
            result.push(cand_id);
            if result.len() >= m {
                break;
            }
        }

        result
    }

    /// Connect new node `p` with `neighbors` on `level`
    /// - ensure level storage exists
    /// - merge and prune neighbor lists for `p` and each neighbor (cap by `m`/`M0`)
    /// - skip linking into deleted nodes
    pub(super) fn connect_bidirectional(
        &mut self,
        p: PointId,
        neighbors: &[PointId],
        level: usize,
        m: usize,
    ) {
        let node = self.index.nodes.get_mut(&p).expect("node must exist");
        if node.neighbors.len() <= level {
            node.neighbors.resize(level + 1, Vec::new());
        }

        let mut combined_p: Vec<PointId> = {
            let node = self.index.nodes.get(&p).unwrap();
            node.neighbors[level].clone()
        };
        for &n in neighbors {
            if n != p && !combined_p.contains(&n) {
                combined_p.push(n);
            }
        }
        let p_vec = self.get_vec(p).to_vec();
        let mut scored_p: Vec<(PointId, f32)> = combined_p
            .into_iter()
            .map(|nid| {
                let d = distance(p_vec.clone(), self.get_vec(nid).to_vec(), self.similarity);
                (nid, d)
            })
            .collect();
        scored_p.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        scored_p.truncate(m);
        let new_p_list: Vec<PointId> = scored_p.into_iter().map(|(nid, _)| nid).collect();
        {
            let node = self.index.nodes.get_mut(&p).unwrap();
            node.neighbors[level] = new_p_list;
        }

        for &n in neighbors {
            if n == p {
                continue;
            }
            if let Some(nn) = self.index.nodes.get(&n) {
                if nn.deleted {
                    continue;
                }
            }
            {
                let node = self.index.nodes.get_mut(&n).expect("neighbor must exist");
                if node.neighbors.len() <= level {
                    node.neighbors.resize(level + 1, Vec::new());
                }
            }
            let cap = if level == 0 {
                self.max_connections_0
            } else {
                self.max_connections
            };

            let mut combined_n: Vec<PointId> = {
                let node = self.index.nodes.get(&n).unwrap();
                node.neighbors[level].clone()
            };
            if !combined_n.contains(&p) {
                combined_n.push(p);
            }

            let n_vec = self.get_vec(n).to_vec();
            let mut scored_n: Vec<(PointId, f32)> = combined_n
                .into_iter()
                .map(|nid| {
                    let d = distance(n_vec.clone(), self.get_vec(nid).to_vec(), self.similarity);
                    (nid, d)
                })
                .collect();
            scored_n.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            scored_n.truncate(cap);
            let new_n_list: Vec<PointId> = scored_n.into_iter().map(|(nid, _)| nid).collect();
            {
                let node = self.index.nodes.get_mut(&n).unwrap();
                node.neighbors[level] = new_n_list;
            }
        }
    }
}
