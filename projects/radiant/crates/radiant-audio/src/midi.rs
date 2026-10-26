use std::ffi::CString;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64, Ordering};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use alsa::Direction;
use alsa::seq::{Addr, EvNote, EventType, PortCap, PortSubscribe, PortType, Seq};

use crate::realtime::RealtimeCounters;
use crate::{MidiIngress, MidiMessage, MidiMessageKind};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MidiPortStatus {
    pub online: bool,
    pub client_id: i32,
    pub port_id: i32,
    pub topology_generation: u64,
    pub failures: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimestampMapper {
    sample_rate: u32,
}

impl TimestampMapper {
    #[must_use]
    pub const fn new(sample_rate: u32) -> Self {
        Self { sample_rate }
    }

    #[must_use]
    pub fn to_audio_frame(self, callback_frame: u64, callback_nanos: u64, event_nanos: u64) -> u64 {
        let nanoseconds = event_nanos.abs_diff(callback_nanos);
        let frames = u128::from(nanoseconds).saturating_mul(u128::from(self.sample_rate))
            / 1_000_000_000_u128;
        let frames = u64::try_from(frames).unwrap_or(u64::MAX);
        if event_nanos >= callback_nanos {
            callback_frame.saturating_add(frames)
        } else {
            callback_frame.saturating_sub(frames)
        }
    }
}

#[derive(Debug, Default)]
struct MidiStatusAtomics {
    online: AtomicBool,
    client_id: AtomicI32,
    port_id: AtomicI32,
    topology_generation: AtomicU64,
    failures: AtomicU64,
}

pub struct AlsaMidiInput {
    stop: Arc<AtomicBool>,
    status: Arc<MidiStatusAtomics>,
    thread: Option<JoinHandle<()>>,
}

impl AlsaMidiInput {
    pub(crate) fn start(
        mut ingress: MidiIngress,
        counters: Arc<RealtimeCounters>,
        sample_rate: u32,
        clock_origin: Instant,
    ) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let status = Arc::new(MidiStatusAtomics::default());
        let thread_stop = stop.clone();
        let thread_status = status.clone();
        let thread = thread::Builder::new()
            .name("radiant-alsa-midi".into())
            .spawn(move || {
                let mapper = TimestampMapper::new(sample_rate);
                while !thread_stop.load(Ordering::Acquire) {
                    if run_sequencer(
                        &thread_stop,
                        &thread_status,
                        &counters,
                        &mut ingress,
                        mapper,
                        clock_origin,
                    )
                    .is_ok()
                    {
                        break;
                    }
                    thread_status.online.store(false, Ordering::Release);
                    thread_status.failures.fetch_add(1, Ordering::Relaxed);
                    thread::sleep(Duration::from_millis(500));
                }
                thread_status.online.store(false, Ordering::Release);
            })
            .ok();
        Self {
            stop,
            status,
            thread,
        }
    }

    #[must_use]
    pub fn status(&self) -> MidiPortStatus {
        MidiPortStatus {
            online: self.status.online.load(Ordering::Acquire),
            client_id: self.status.client_id.load(Ordering::Relaxed),
            port_id: self.status.port_id.load(Ordering::Relaxed),
            topology_generation: self.status.topology_generation.load(Ordering::Relaxed),
            failures: self.status.failures.load(Ordering::Relaxed),
        }
    }
}

impl Drop for AlsaMidiInput {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn run_sequencer(
    stop: &AtomicBool,
    status: &MidiStatusAtomics,
    counters: &RealtimeCounters,
    ingress: &mut MidiIngress,
    mapper: TimestampMapper,
    clock_origin: Instant,
) -> Result<(), ()> {
    let seq = Seq::open(None, Some(Direction::Capture), true).map_err(|_| ())?;
    let client_name = CString::new("Radiant").map_err(|_| ())?;
    seq.set_client_name(&client_name).map_err(|_| ())?;
    let port_name = CString::new("Radiant MIDI In").map_err(|_| ())?;
    let port = seq
        .create_simple_port(
            &port_name,
            PortCap::WRITE | PortCap::SUBS_WRITE,
            PortType::MIDI_GENERIC | PortType::APPLICATION,
        )
        .map_err(|_| ())?;
    let client = seq.client_id().map_err(|_| ())?;
    subscribe_to_announcements(&seq, client, port);
    status.client_id.store(client, Ordering::Relaxed);
    status.port_id.store(port, Ordering::Relaxed);
    status.online.store(true, Ordering::Release);

    let mut input = seq.input();
    while !stop.load(Ordering::Acquire) {
        if input.event_input_pending(true).map_err(|_| ())? == 0 {
            thread::sleep(Duration::from_millis(1));
            continue;
        }
        while input.event_input_pending(false).map_err(|_| ())? > 0 {
            let event = input.event_input().map_err(|_| ())?;
            match event.get_type() {
                EventType::Note | EventType::Noteon | EventType::Noteoff => {
                    if let Some(note) = event.get_data::<EvNote>() {
                        let kind = if event.get_type() == EventType::Noteoff || note.velocity == 0 {
                            MidiMessageKind::NoteOff
                        } else {
                            MidiMessageKind::NoteOn
                        };
                        let event_nanos = elapsed_nanos(clock_origin);
                        let (callback_frame, callback_nanos) = counters.clock_snapshot();
                        let message = MidiMessage {
                            audio_frame: mapper.to_audio_frame(
                                callback_frame,
                                callback_nanos,
                                event_nanos,
                            ),
                            kind,
                            channel: note.channel,
                            pitch: note.note,
                            velocity: note.velocity,
                        };
                        let _ = ingress.send(message);
                    }
                }
                EventType::ClientStart
                | EventType::ClientExit
                | EventType::PortStart
                | EventType::PortExit
                | EventType::PortChange => {
                    status.topology_generation.fetch_add(1, Ordering::Relaxed);
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn subscribe_to_announcements(seq: &Seq, client: i32, port: i32) {
    let Ok(subscription) = PortSubscribe::empty() else {
        return;
    };
    subscription.set_sender(Addr::system_announce());
    subscription.set_dest(Addr { client, port });
    let _ = seq.subscribe_port(&subscription);
}

fn elapsed_nanos(origin: Instant) -> u64 {
    u64::try_from(origin.elapsed().as_nanos()).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamps_map_forward_and_backward_from_callback_clock() {
        let mapper = TimestampMapper::new(48_000);
        assert_eq!(
            mapper.to_audio_frame(4_800, 100_000_000, 110_000_000),
            5_280
        );
        assert_eq!(mapper.to_audio_frame(4_800, 100_000_000, 90_000_000), 4_320);
    }
}
