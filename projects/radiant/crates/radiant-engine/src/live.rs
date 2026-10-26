//! Callback-safe live mixer graph.
//!
//! Graph construction happens on the control thread. `process_block` only
//! mutates preallocated channel strips and scratch buffers, making it suitable
//! for PipeWire/JACK data callbacks.

use arc_swap::ArcSwap;
use radiant_graph::{CompiledGraph, GraphError};
use std::sync::Arc;

use crate::ChannelStrip;

#[derive(Clone, Copy, Debug)]
pub struct LiveChannelConfig {
    pub gain: f32,
    pub pan: f32,
    pub muted: bool,
    pub solo: bool,
    pub phase_inverted: bool,
    pub stereo_width: f32,
    pub compensation_delay: usize,
}

impl Default for LiveChannelConfig {
    fn default() -> Self {
        Self {
            gain: 1.0,
            pan: 0.0,
            muted: false,
            solo: false,
            phase_inverted: false,
            stereo_width: 1.0,
            compensation_delay: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LiveRoute {
    pub source: usize,
    pub destination: usize,
    pub gain: f32,
    pub pre_fader: bool,
    pub sidechain: bool,
}

/// Immutable control-thread graph description suitable for atomic publication.
#[derive(Clone, Debug)]
pub struct LiveGraphTopology {
    graph: CompiledGraph,
    master: usize,
    configs: Vec<LiveChannelConfig>,
    routes: Vec<Vec<LiveRoute>>,
    solo_path: Vec<bool>,
}

impl LiveGraphTopology {
    pub fn compile(
        configs: &[LiveChannelConfig],
        routes: &[LiveRoute],
        master: usize,
    ) -> Result<Self, GraphError> {
        if master >= configs.len() {
            return Err(GraphError::NodeOutOfRange {
                source_node: master,
                destination: master,
                node_count: configs.len(),
            });
        }
        let edges: Vec<_> = routes
            .iter()
            .map(|route| (route.source, route.destination))
            .collect();
        let latencies: Vec<_> = configs
            .iter()
            .map(|config| u32::try_from(config.compensation_delay).unwrap_or(u32::MAX))
            .collect();
        let graph = CompiledGraph::compile(configs.len(), &edges, &latencies)?;
        let mut effective_configs = configs.to_vec();
        for (index, config) in effective_configs.iter_mut().enumerate() {
            config.compensation_delay =
                usize::try_from(graph.compensation_delay(index).unwrap_or(0)).unwrap_or(usize::MAX);
        }
        let mut grouped = vec![Vec::new(); configs.len()];
        for route in routes {
            if route.source >= configs.len() || route.destination >= configs.len() {
                return Err(GraphError::NodeOutOfRange {
                    source_node: route.source,
                    destination: route.destination,
                    node_count: configs.len(),
                });
            }
            grouped[route.source].push(*route);
        }
        let mut solo_path = vec![false; configs.len()];
        resolve_solo_path(configs, &grouped, &mut solo_path);
        Ok(Self {
            graph,
            master,
            configs: effective_configs,
            routes: grouped,
            solo_path,
        })
    }
}

/// Stateful real-time renderer consuming atomically published immutable
/// topologies. All processing buffers and channel state are allocated once.
pub struct LiveGraphRuntime {
    topology: Arc<ArcSwap<LiveGraphTopology>>,
    channels: Vec<ChannelStrip>,
    scratch: Vec<[f32; 2]>,
    sidechain: Vec<f32>,
    overrides: Vec<Option<LiveChannelConfig>>,
}

const LIVE_DELAY_CAPACITY: usize = 262_145;

#[derive(Clone)]
pub struct LiveGraphPublisher {
    topology: Arc<ArcSwap<LiveGraphTopology>>,
}

impl LiveGraphRuntime {
    #[must_use]
    pub fn new(topology: LiveGraphTopology, capacity: usize) -> Self {
        let capacity = capacity.max(topology.configs.len());
        Self {
            topology: Arc::new(ArcSwap::from_pointee(topology)),
            channels: (0..capacity)
                .map(|_| {
                    let mut channel = ChannelStrip::new(1.0, 0.0, 64);
                    channel.prepare_delay_capacity(LIVE_DELAY_CAPACITY);
                    channel
                })
                .collect(),
            scratch: vec![[0.0; 2]; capacity],
            sidechain: vec![0.0; capacity],
            overrides: vec![None; capacity],
        }
    }

    #[must_use]
    pub fn publisher(&self) -> LiveGraphPublisher {
        LiveGraphPublisher {
            topology: self.topology.clone(),
        }
    }

    pub fn publish(&self, topology: LiveGraphTopology) {
        self.topology.store(Arc::new(topology));
    }

    pub fn set_channel_config(&mut self, index: usize, config: LiveChannelConfig) -> bool {
        let Some(slot) = self.overrides.get_mut(index) else {
            return false;
        };
        *slot = Some(config);
        true
    }

    pub fn process_block(&mut self, inputs: &[&[[f32; 2]]], output: &mut [[f32; 2]]) {
        let topology = self.topology.load();
        let count = topology.configs.len().min(self.channels.len());
        if topology.configs.len() > self.channels.len() {
            output.fill([0.0; 2]);
            return;
        }
        for (frame, destination) in output.iter_mut().enumerate() {
            self.scratch[..count].fill([0.0; 2]);
            self.sidechain[..count].fill(0.0);
            for (index, input) in inputs.iter().enumerate().take(count) {
                self.scratch[index] = input.get(frame).copied().unwrap_or([0.0; 2]);
            }
            for &index in topology.graph.order() {
                if index >= count {
                    continue;
                }
                let config = self.overrides[index].unwrap_or(topology.configs[index]);
                if self.channels[index].delay_frames() != config.compensation_delay {
                    let _ = self.channels[index].set_delay_bounded(config.compensation_delay);
                }
                let solo_muted = topology.configs.iter().any(|item| item.solo)
                    && !topology.solo_path[index]
                    && index != topology.master;
                self.channels[index].set_targets(
                    if config.muted || solo_muted {
                        0.0
                    } else {
                        config.gain
                    },
                    config.pan,
                    64,
                );
                let mut processed = self.channels[index].process(self.scratch[index]);
                let duck = 1.0 / (1.0 + self.sidechain[index] * 2.0);
                processed[0] *= duck;
                processed[1] *= duck;
                self.scratch[index] = processed;
                for route in &topology.routes[index] {
                    if route.sidechain {
                        self.sidechain[route.destination] +=
                            (processed[0].abs() + processed[1].abs()) * 0.5 * route.gain.abs();
                    } else {
                        let sent = if route.pre_fader {
                            self.scratch[index]
                        } else {
                            processed
                        };
                        self.scratch[route.destination][0] += sent[0] * route.gain;
                        self.scratch[route.destination][1] += sent[1] * route.gain;
                    }
                }
            }
            *destination = self.scratch[topology.master];
        }
    }
}

impl LiveGraphPublisher {
    pub fn publish(&self, topology: LiveGraphTopology) {
        self.topology.store(Arc::new(topology));
    }
}

#[derive(Debug)]
pub struct LiveMixerGraph {
    graph: CompiledGraph,
    master: usize,
    channels: Vec<ChannelStrip>,
    configs: Vec<LiveChannelConfig>,
    routes: Vec<Vec<LiveRoute>>,
    scratch: Vec<[f32; 2]>,
    sidechain: Vec<f32>,
    solo_path: Vec<bool>,
}

impl LiveMixerGraph {
    pub fn new(
        configs: &[LiveChannelConfig],
        routes: &[LiveRoute],
        master: usize,
    ) -> Result<Self, GraphError> {
        if master >= configs.len() {
            return Err(GraphError::NodeOutOfRange {
                source_node: master,
                destination: master,
                node_count: configs.len(),
            });
        }
        let edges: Vec<_> = routes
            .iter()
            .map(|route| (route.source, route.destination))
            .collect();
        let latencies: Vec<_> = configs
            .iter()
            .map(|config| u32::try_from(config.compensation_delay).unwrap_or(u32::MAX))
            .collect();
        let graph = CompiledGraph::compile(configs.len(), &edges, &latencies)?;
        let mut channels = Vec::with_capacity(configs.len());
        for (index, config) in configs.iter().enumerate() {
            let mut strip = ChannelStrip::new(config.gain, config.pan, 64);
            strip.phase_inverted = config.phase_inverted;
            strip.stereo_width = config.stereo_width;
            strip.set_delay(
                usize::try_from(graph.compensation_delay(index).unwrap_or(0)).unwrap_or(usize::MAX),
            );
            channels.push(strip);
        }
        let mut grouped = vec![Vec::new(); configs.len()];
        for route in routes {
            if route.source >= configs.len() || route.destination >= configs.len() {
                return Err(GraphError::NodeOutOfRange {
                    source_node: route.source,
                    destination: route.destination,
                    node_count: configs.len(),
                });
            }
            grouped[route.source].push(*route);
        }
        let mut solo_path = vec![false; configs.len()];
        resolve_solo_path(configs, &grouped, &mut solo_path);
        Ok(Self {
            graph,
            master,
            channels,
            configs: configs.to_vec(),
            routes: grouped,
            scratch: vec![[0.0; 2]; configs.len()],
            sidechain: vec![0.0; configs.len()],
            solo_path,
        })
    }

    /// Update one channel from a control message. The operation is bounded
    /// and allocation-free so it may be applied at a callback command boundary.
    pub fn set_channel_config(&mut self, index: usize, config: LiveChannelConfig) -> bool {
        let Some(previous) = self.configs.get_mut(index) else {
            return false;
        };
        *previous = config;
        let Some(strip) = self.channels.get_mut(index) else {
            return false;
        };
        strip.phase_inverted = config.phase_inverted;
        strip.stereo_width = config.stereo_width;
        // Delay-line topology is fixed when the graph is built; a latency
        // change requires a control-thread graph rebuild.
        resolve_solo_path(&self.configs, &self.routes, &mut self.solo_path);
        true
    }

    #[must_use]
    pub fn channel_config(&self, index: usize) -> Option<LiveChannelConfig> {
        self.configs.get(index).copied()
    }

    /// Replace an existing route slot without changing graph topology. Route
    /// slots are allocated during graph construction; callers can stage a
    /// new gain/tap mode at a callback boundary without allocating.
    pub fn set_route_slot(&mut self, source: usize, slot: usize, route: LiveRoute) -> bool {
        if route.source != source || route.destination >= self.configs.len() {
            return false;
        }
        let Some(existing) = self
            .routes
            .get_mut(source)
            .and_then(|routes| routes.get_mut(slot))
        else {
            return false;
        };
        *existing = route;
        true
    }

    #[must_use]
    pub fn graph(&self) -> &CompiledGraph {
        &self.graph
    }

    /// Process interleaved stereo frames for every connected input bus.
    /// Inputs are copied into preallocated graph scratch; no callback
    /// allocation occurs.
    pub fn process_block(&mut self, inputs: &[&[[f32; 2]]], output: &mut [[f32; 2]]) {
        for (frame, destination) in output.iter_mut().enumerate() {
            self.scratch.fill([0.0; 2]);
            self.sidechain.fill(0.0);
            for (index, input) in inputs.iter().enumerate().take(self.scratch.len()) {
                self.scratch[index] = input.get(frame).copied().unwrap_or([0.0; 2]);
            }
            for &index in self.graph.order() {
                let dry = self.scratch[index];
                let config = self.configs[index];
                let solo_muted = self.configs.iter().any(|item| item.solo)
                    && !self.solo_path[index]
                    && index != self.master;
                self.channels[index].set_targets(
                    if config.muted || solo_muted {
                        0.0
                    } else {
                        config.gain
                    },
                    config.pan,
                    64,
                );
                let duck = 1.0 / (1.0 + self.sidechain[index] * 2.0);
                let mut processed = self.channels[index].process(dry);
                processed[0] *= duck;
                processed[1] *= duck;
                self.scratch[index] = processed;
                for route in &self.routes[index] {
                    if route.sidechain {
                        self.sidechain[route.destination] +=
                            (processed[0].abs() + processed[1].abs()) * 0.5 * route.gain.abs();
                    } else {
                        let sent = if route.pre_fader { dry } else { processed };
                        self.scratch[route.destination][0] += sent[0] * route.gain;
                        self.scratch[route.destination][1] += sent[1] * route.gain;
                    }
                }
            }
            *destination = self.scratch[self.master];
        }
    }
}

fn resolve_solo_path(configs: &[LiveChannelConfig], routes: &[Vec<LiveRoute>], path: &mut [bool]) {
    for (slot, config) in path.iter_mut().zip(configs) {
        *slot = config.solo;
    }
    let mut changed = true;
    while changed {
        changed = false;
        for (source, source_routes) in routes.iter().enumerate() {
            if !path[source] {
                continue;
            }
            for route in source_routes {
                if !route.sidechain && !path[route.destination] {
                    path[route.destination] = true;
                    changed = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impulse_reaches_master_through_post_route() {
        let configs = [LiveChannelConfig::default(); 2];
        let routes = [LiveRoute {
            source: 0,
            destination: 1,
            gain: 1.0,
            pre_fader: false,
            sidechain: false,
        }];
        let mut graph = LiveMixerGraph::new(&configs, &routes, 1).expect("graph");
        let inputs = [[1.0, 1.0]];
        let inputs = [&inputs[..]];
        let mut output = [[0.0, 0.0]];
        graph.process_block(&inputs, &mut output);
        assert!(
            output[0]
                .iter()
                .all(|sample| (*sample - 1.0).abs() < f32::EPSILON)
        );
    }

    #[test]
    fn cycles_are_rejected_before_live_processing() {
        let configs = [LiveChannelConfig::default(); 2];
        let routes = [
            LiveRoute {
                source: 0,
                destination: 1,
                gain: 1.0,
                pre_fader: false,
                sidechain: false,
            },
            LiveRoute {
                source: 1,
                destination: 0,
                gain: 1.0,
                pre_fader: false,
                sidechain: false,
            },
        ];
        assert_eq!(
            LiveMixerGraph::new(&configs, &routes, 1).unwrap_err(),
            GraphError::Cycle
        );
    }

    fn route_graph(configs: &[LiveChannelConfig], routes: &[LiveRoute]) -> LiveMixerGraph {
        LiveMixerGraph::new(configs, routes, configs.len() - 1).expect("graph")
    }

    #[test]
    fn pre_and_post_fader_taps_are_sample_exact() {
        let mut config = [LiveChannelConfig::default(); 2];
        config[0].gain = 0.5;
        let pre_route = [LiveRoute {
            source: 0,
            destination: 1,
            gain: 1.0,
            pre_fader: true,
            sidechain: false,
        }];
        let mut pre = route_graph(&config, &pre_route);
        let source = [[1.0, 1.0]];
        let inputs = [&source[..]];
        let mut output = [[0.0, 0.0]];
        pre.process_block(&inputs, &mut output);
        assert!((output[0][0] - 1.0).abs() < f32::EPSILON);

        let post_route = [LiveRoute {
            pre_fader: false,
            ..pre_route[0]
        }];
        let mut post = route_graph(&config, &post_route);
        let mut output = [[0.0, 0.0]];
        post.process_block(&inputs, &mut output);
        assert!((output[0][0] - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn sidechain_ducks_without_becoming_audible() {
        let configs = [LiveChannelConfig::default(); 2];
        let routes = [LiveRoute {
            source: 0,
            destination: 1,
            gain: 1.0,
            pre_fader: false,
            sidechain: true,
        }];
        let mut graph = route_graph(&configs, &routes);
        let trigger = [[1.0, 1.0]];
        let program = [[1.0, 1.0]];
        let inputs = [&trigger[..], &program[..]];
        let mut output = [[0.0, 0.0]];
        graph.process_block(&inputs, &mut output);
        assert!(output[0][0] < 1.0);
        assert!(output[0][0] > 0.0);
    }

    #[test]
    fn compensation_delays_the_shorter_path() {
        let mut configs = [LiveChannelConfig::default(); 3];
        configs[0].compensation_delay = 2;
        let routes = [
            LiveRoute {
                source: 0,
                destination: 2,
                gain: 1.0,
                pre_fader: false,
                sidechain: false,
            },
            LiveRoute {
                source: 1,
                destination: 2,
                gain: 1.0,
                pre_fader: false,
                sidechain: false,
            },
        ];
        let mut graph = LiveMixerGraph::new(&configs, &routes, 2).expect("graph");
        let silent = [[0.0, 0.0]; 3];
        let impulse = [[1.0, 1.0], [0.0, 0.0], [0.0, 0.0]];
        let inputs = [&silent[..], &impulse[..], &silent[..]];
        let mut output = [[0.0, 0.0]; 3];
        graph.process_block(&inputs, &mut output);
        assert!(output[0][0].abs() < f32::EPSILON);
        assert!(output[1][0].abs() < f32::EPSILON);
        assert!((output[2][0] - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn mute_and_solo_are_respected_at_the_output() {
        let mut configs = [LiveChannelConfig::default(); 3];
        configs[1].muted = true;
        configs[1].gain = 0.0;
        configs[0].solo = true;
        let routes = [
            LiveRoute {
                source: 0,
                destination: 2,
                gain: 1.0,
                pre_fader: false,
                sidechain: false,
            },
            LiveRoute {
                source: 1,
                destination: 2,
                gain: 1.0,
                pre_fader: false,
                sidechain: false,
            },
        ];
        let mut graph = LiveMixerGraph::new(&configs, &routes, 2).expect("graph");
        let first = [[0.25, 0.25]];
        let second = [[0.75, 0.75]];
        let inputs = [&first[..], &second[..], &[][..]];
        let mut output = [[0.0, 0.0]];
        graph.process_block(&inputs, &mut output);
        assert!((output[0][0] - 0.25).abs() < f32::EPSILON);
    }

    #[test]
    fn existing_route_slot_can_change_without_reallocation() {
        let configs = [LiveChannelConfig::default(); 2];
        let routes = [LiveRoute {
            source: 0,
            destination: 1,
            gain: 1.0,
            pre_fader: false,
            sidechain: false,
        }];
        let mut graph = route_graph(&configs, &routes);
        assert!(graph.set_route_slot(
            0,
            0,
            LiveRoute {
                gain: 0.25,
                ..routes[0]
            }
        ));
        let source = [[1.0, 1.0]];
        let inputs = [&source[..]];
        let mut output = [[0.0, 0.0]];
        graph.process_block(&inputs, &mut output);
        assert!((output[0][0] - 0.25).abs() < f32::EPSILON);
    }
}
