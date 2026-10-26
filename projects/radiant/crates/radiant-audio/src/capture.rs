//! Native input capture with a preallocated callback-to-writer queue.

use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream, StreamConfig};
use rtrb::{Consumer, RingBuffer};

use crate::AudioError;

pub struct NativeCapture {
    _stream: Stream,
    consumer: Arc<Mutex<Consumer<f32>>>,
    pub sample_rate: u32,
    pub channels: u16,
}

impl NativeCapture {
    pub fn start() -> Result<Self, AudioError> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or(AudioError::NoInputDevice)?;
        let supported = device
            .default_input_config()
            .map_err(|error| AudioError::Stream(error.to_string()))?;
        if supported.sample_format() != SampleFormat::F32 {
            return Err(AudioError::UnsupportedInputConfiguration);
        }
        let config: StreamConfig = supported.clone().into();
        let channels = config.channels;
        let sample_rate = config.sample_rate;
        let (mut producer, consumer) = RingBuffer::new(48_000 * usize::from(channels) * 4);
        let consumer = Arc::new(Mutex::new(consumer));
        let stream = device
            .build_input_stream(
                &config,
                move |input: &[f32], _| {
                    for &sample in input {
                        let _ = producer.push(sample);
                    }
                },
                move |_error| {},
                None,
            )
            .map_err(|error| AudioError::Stream(error.to_string()))?;
        stream
            .play()
            .map_err(|error| AudioError::Stream(error.to_string()))?;
        Ok(Self {
            _stream: stream,
            consumer,
            sample_rate,
            channels,
        })
    }

    /// Drains samples on a non-real-time writer thread.
    pub fn drain(&self, destination: &mut Vec<f32>, maximum_samples: usize) {
        let Ok(mut consumer) = self.consumer.lock() else {
            return;
        };
        for _ in 0..maximum_samples {
            let Ok(sample) = consumer.pop() else {
                break;
            };
            destination.push(sample);
        }
    }
}
