//! Deterministic, UI-independent directed audio-graph compilation.

use std::collections::BTreeSet;

use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompiledGraph {
    order: Vec<usize>,
    path_latency: Vec<u32>,
}

impl CompiledGraph {
    pub fn compile(
        node_count: usize,
        edges: &[(usize, usize)],
        node_latency: &[u32],
    ) -> Result<Self, GraphError> {
        if node_latency.len() != node_count {
            return Err(GraphError::LatencyCount {
                nodes: node_count,
                latencies: node_latency.len(),
            });
        }
        let mut incoming = vec![0_usize; node_count];
        let mut outgoing = vec![Vec::new(); node_count];
        for &(source, destination) in edges {
            if source >= node_count || destination >= node_count {
                return Err(GraphError::NodeOutOfRange {
                    source_node: source,
                    destination,
                    node_count,
                });
            }
            if source == destination {
                return Err(GraphError::Cycle);
            }
            if !outgoing[source].contains(&destination) {
                outgoing[source].push(destination);
                incoming[destination] += 1;
            }
        }
        for destinations in &mut outgoing {
            destinations.sort_unstable();
            destinations.dedup();
        }
        let mut ready: BTreeSet<_> = incoming
            .iter()
            .enumerate()
            .filter_map(|(index, count)| (*count == 0).then_some(index))
            .collect();
        let mut order = Vec::with_capacity(node_count);
        while let Some(node) = ready.pop_first() {
            order.push(node);
            for &destination in &outgoing[node] {
                incoming[destination] -= 1;
                if incoming[destination] == 0 {
                    ready.insert(destination);
                }
            }
        }
        if order.len() != node_count {
            return Err(GraphError::Cycle);
        }

        let mut path_latency = node_latency.to_vec();
        for &source in &order {
            for &destination in &outgoing[source] {
                path_latency[destination] = path_latency[destination]
                    .max(path_latency[source].saturating_add(node_latency[destination]));
            }
        }
        Ok(Self {
            order,
            path_latency,
        })
    }

    #[must_use]
    pub fn order(&self) -> &[usize] {
        &self.order
    }

    #[must_use]
    pub fn path_latency(&self, node: usize) -> Option<u32> {
        self.path_latency.get(node).copied()
    }

    #[must_use]
    pub fn maximum_latency(&self) -> u32 {
        self.path_latency.iter().copied().max().unwrap_or(0)
    }

    /// Delay required to align a node with the longest path in the graph.
    #[must_use]
    pub fn compensation_delay(&self, node: usize) -> Option<u32> {
        let maximum = self.maximum_latency();
        self.path_latency
            .get(node)
            .map(|latency| maximum.saturating_sub(*latency))
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum GraphError {
    #[error("audio graph contains a routing cycle")]
    Cycle,
    #[error("edge {source_node}->{destination} is outside a graph with {node_count} nodes")]
    NodeOutOfRange {
        source_node: usize,
        destination: usize,
        node_count: usize,
    },
    #[error("graph has {nodes} nodes but {latencies} latency entries")]
    LatencyCount { nodes: usize, latencies: usize },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering_is_stable_and_sources_precede_destinations() {
        let graph =
            CompiledGraph::compile(5, &[(2, 4), (0, 3), (3, 4)], &[0; 5]).expect("compile graph");
        assert_eq!(graph.order(), &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn cycles_are_rejected() {
        assert_eq!(
            CompiledGraph::compile(3, &[(0, 1), (1, 2), (2, 0)], &[0; 3]),
            Err(GraphError::Cycle)
        );
    }

    #[test]
    fn latency_follows_the_longest_upstream_path() {
        let graph = CompiledGraph::compile(4, &[(0, 2), (1, 2), (2, 3)], &[4, 10, 8, 2])
            .expect("compile graph");
        assert_eq!(graph.path_latency(2), Some(18));
        assert_eq!(graph.path_latency(3), Some(20));
        assert_eq!(graph.maximum_latency(), 20);
        assert_eq!(graph.compensation_delay(0), Some(16));
        assert_eq!(graph.compensation_delay(3), Some(0));
    }
}
