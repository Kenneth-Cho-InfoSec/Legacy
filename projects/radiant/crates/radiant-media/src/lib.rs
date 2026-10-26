//! Decoded, planar audio media used by Radiant's offline engine.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct AudioBuffer {
    sample_rate: u32,
    channels: Vec<Vec<f32>>,
}

impl AudioBuffer {
    pub fn new(sample_rate: u32, channels: Vec<Vec<f32>>) -> Result<Self, MediaError> {
        if sample_rate == 0 || channels.is_empty() {
            return Err(MediaError::InvalidBuffer);
        }
        let frames = channels[0].len();
        if channels.iter().any(|channel| channel.len() != frames) {
            return Err(MediaError::InvalidBuffer);
        }
        Ok(Self {
            sample_rate,
            channels,
        })
    }

    #[must_use]
    pub const fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    #[must_use]
    pub fn channel_count(&self) -> usize {
        self.channels.len()
    }

    #[must_use]
    pub fn frames(&self) -> usize {
        self.channels[0].len()
    }

    #[must_use]
    pub fn channel(&self, channel: usize) -> Option<&[f32]> {
        self.channels.get(channel).map(Vec::as_slice)
    }

    #[must_use]
    pub fn sample(&self, channel: usize, frame: usize) -> f32 {
        let channel = channel.min(self.channels.len() - 1);
        self.channels[channel].get(frame).copied().unwrap_or(0.0)
    }

    #[must_use]
    pub fn silence(sample_rate: u32, channels: usize, frames: usize) -> Self {
        Self {
            sample_rate,
            channels: vec![vec![0.0; frames]; channels.max(1)],
        }
    }

    #[must_use]
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss
    )]
    pub fn resample_linear(&self, target_rate: u32) -> Self {
        if target_rate == self.sample_rate || self.frames() == 0 {
            return self.clone();
        }
        let output_frames = scaled_frame_count(self.frames(), self.sample_rate, target_rate);
        let ratio = f64::from(self.sample_rate) / f64::from(target_rate);
        let channels = self
            .channels
            .iter()
            .map(|channel| {
                (0..output_frames)
                    .map(|output_frame| {
                        #[allow(clippy::cast_precision_loss)]
                        let source_position = output_frame as f64 * ratio;
                        let left = source_position.floor() as usize;
                        let right = (left + 1).min(channel.len() - 1);
                        #[allow(clippy::cast_possible_truncation)]
                        let fraction = (source_position - left as f64) as f32;
                        channel[left] + (channel[right] - channel[left]) * fraction
                    })
                    .collect()
            })
            .collect();
        Self {
            sample_rate: target_rate,
            channels,
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
fn scaled_frame_count(frames: usize, source_rate: u32, target_rate: u32) -> usize {
    let frames = u128::try_from(frames).unwrap_or(u128::MAX);
    let numerator = frames.saturating_mul(u128::from(target_rate));
    let rounded = numerator.saturating_add(u128::from(source_rate) / 2) / u128::from(source_rate);
    usize::try_from(rounded).unwrap_or(usize::MAX)
}

pub fn decode(path: impl AsRef<Path>) -> Result<AudioBuffer, MediaError> {
    let path = path.as_ref();
    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("wav" | "wave") => decode_wav(path),
        Some("flac") => decode_flac(path),
        _ => Err(MediaError::UnsupportedFormat(path.to_path_buf())),
    }
}

fn decode_wav(path: &Path) -> Result<AudioBuffer, MediaError> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    let channel_count = usize::from(spec.channels);
    if channel_count == 0 {
        return Err(MediaError::InvalidBuffer);
    }
    let mut channels = vec![Vec::new(); channel_count];
    match spec.sample_format {
        hound::SampleFormat::Float => {
            for (index, sample) in reader.samples::<f32>().enumerate() {
                channels[index % channel_count].push(sanitize(sample?));
            }
        }
        hound::SampleFormat::Int => {
            let scale = 2.0_f64.powi(i32::from(spec.bits_per_sample) - 1);
            for (index, sample) in reader.samples::<i32>().enumerate() {
                let value = f64::from(sample?) / scale;
                #[allow(clippy::cast_possible_truncation)]
                channels[index % channel_count].push(sanitize(value as f32));
            }
        }
    }
    AudioBuffer::new(spec.sample_rate, channels)
}

fn decode_flac(path: &Path) -> Result<AudioBuffer, MediaError> {
    let input = BufReader::new(File::open(path)?);
    let mut reader = claxon::FlacReader::new(input)?;
    let info = reader.streaminfo();
    let sample_rate = info.sample_rate;
    let channel_count = usize::try_from(info.channels).map_err(|_| MediaError::InvalidBuffer)?;
    if channel_count == 0 {
        return Err(MediaError::InvalidBuffer);
    }
    let scale = 2.0_f64.powi(i32::try_from(info.bits_per_sample).unwrap_or(32) - 1);
    let mut channels = vec![Vec::new(); channel_count];
    for (index, sample) in reader.samples().enumerate() {
        let value = f64::from(sample?) / scale;
        #[allow(clippy::cast_possible_truncation)]
        channels[index % channel_count].push(sanitize(value as f32));
    }
    AudioBuffer::new(sample_rate, channels)
}

pub fn write_wav(path: impl AsRef<Path>, audio: &AudioBuffer) -> Result<(), MediaError> {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: audio.sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create(path, spec)?;
    for frame in 0..audio.frames() {
        let left = sanitize(audio.sample(0, frame));
        let right = sanitize(audio.sample(usize::from(audio.channel_count() > 1), frame));
        writer.write_sample(left)?;
        writer.write_sample(right)?;
    }
    writer.finalize()?;
    Ok(())
}

#[must_use]
pub fn sanitize(sample: f32) -> f32 {
    if !sample.is_finite() || (sample != 0.0 && sample.abs() < f32::MIN_POSITIVE) {
        0.0
    } else {
        sample.clamp(-1.0, 1.0)
    }
}

/// Min/max envelope for drawing long files without retaining decoded audio in
/// the UI. Peaks are derived data and can always be rebuilt.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct WaveformPeaks {
    pub buckets: u32,
    pub channels: Vec<Vec<Peak>>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Peak {
    pub min: f32,
    pub max: f32,
}

impl WaveformPeaks {
    #[must_use]
    pub fn from_audio(audio: &AudioBuffer, requested_buckets: u32) -> Self {
        let frames = audio.frames();
        let buckets = requested_buckets
            .max(1)
            .min(u32::try_from(frames.max(1)).unwrap_or(u32::MAX));
        let bucket_count = usize::try_from(buckets).unwrap_or(usize::MAX);
        let channels = (0..audio.channel_count())
            .map(|channel| {
                (0..bucket_count)
                    .map(|bucket| {
                        let start = bucket.saturating_mul(frames) / bucket_count;
                        let end = ((bucket + 1).saturating_mul(frames) / bucket_count)
                            .max(start + 1)
                            .min(frames);
                        let (mut min, mut max) = (0.0_f32, 0.0_f32);
                        for frame in start..end {
                            let sample = audio.sample(channel, frame);
                            min = min.min(sample);
                            max = max.max(sample);
                        }
                        Peak { min, max }
                    })
                    .collect()
            })
            .collect();
        Self { buckets, channels }
    }
}

#[derive(Debug, Error)]
pub enum MediaError {
    #[error("audio buffer has invalid dimensions or sample rate")]
    InvalidBuffer,
    #[error("unsupported audio format: {0}")]
    UnsupportedFormat(std::path::PathBuf),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Wav(#[from] hound::Error),
    #[error(transparent)]
    Flac(#[from] claxon::Error),
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn wav_round_trip_preserves_shape_and_signal() {
        let directory = tempdir().expect("temporary directory");
        let path = directory.path().join("round-trip.wav");
        let audio = AudioBuffer::new(48_000, vec![vec![0.0, 0.25, -0.5], vec![0.5, -0.25, 0.0]])
            .expect("audio");
        write_wav(&path, &audio).expect("write wav");
        let decoded = decode(&path).expect("decode wav");
        assert_eq!(decoded.sample_rate(), 48_000);
        assert_eq!(decoded.channel_count(), 2);
        assert_eq!(decoded.frames(), 3);
        assert!((decoded.sample(0, 2) - -0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn linear_resampling_preserves_duration_and_endpoints() {
        let audio = AudioBuffer::new(2, vec![vec![0.0, 1.0]]).expect("audio");
        let resampled = audio.resample_linear(4);
        assert_eq!(resampled.frames(), 4);
        assert!(resampled.sample(0, 0).abs() < f32::EPSILON);
        assert!((resampled.sample(0, 2) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn invalid_and_denormal_samples_are_contained() {
        assert!(sanitize(f32::NAN).abs() < f32::EPSILON);
        assert!(sanitize(f32::INFINITY).abs() < f32::EPSILON);
        assert!(sanitize(f32::MIN_POSITIVE / 2.0).abs() < f32::EPSILON);
    }
}
