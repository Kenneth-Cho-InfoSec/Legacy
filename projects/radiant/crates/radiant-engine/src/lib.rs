//! UI-independent scheduling, transport, mixing, and deterministic rendering.

mod automation;
mod delay;
mod live;
mod mixer;
mod plan;
mod render;
mod smoothing;
mod timeline;
mod transport;

use radiant_domain::{ChannelId, MediaId, MixerInsertId, PatternId, ValidationIssue};
use thiserror::Error;

pub use automation::{AutomationMode, SamplePoint, apply_gesture, evaluate as evaluate_automation};
pub use delay::DelayLine;
pub use live::{
    LiveChannelConfig, LiveGraphPublisher, LiveGraphRuntime, LiveGraphTopology, LiveMixerGraph,
    LiveRoute,
};
pub use mixer::{ChannelStrip, MeterSnapshot, StereoMeter};
pub use plan::{MediaPool, RenderPlan};
pub use render::{
    DitherMode, ExportMetadata, RenderOptions, bounce, export_wav, export_wav_with_metadata,
    freeze, render_offline, render_stems,
};
pub use smoothing::SmoothedValue;
pub use timeline::{MeterPoint, MusicalPosition, MusicalTimeline};
pub use transport::{Transport, TransportCommand, TransportState};

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("session validation failed: {0:?}")]
    InvalidSession(Vec<ValidationIssue>),
    #[error("invalid tempo or meter timeline")]
    InvalidTimeline,
    #[error("session has no master mixer insert")]
    MissingMaster,
    #[error("missing mixer insert {0:?}")]
    MissingMixerInsert(MixerInsertId),
    #[error("missing channel {0:?}")]
    MissingChannel(ChannelId),
    #[error("missing pattern {0:?}")]
    MissingPattern(PatternId),
    #[error("missing media {0:?}")]
    MissingMedia(MediaId),
    #[error("render block size must be greater than zero")]
    InvalidBlockSize,
    #[error("render length is too large for this platform")]
    RenderTooLong,
    #[error(transparent)]
    Graph(#[from] radiant_graph::GraphError),
    #[error(transparent)]
    Media(#[from] radiant_media::MediaError),
    #[error("metadata export failed: {0}")]
    MetadataExport(#[from] std::io::Error),
    #[error("metadata serialization failed: {0}")]
    MetadataSerialization(#[from] serde_json::Error),
}
