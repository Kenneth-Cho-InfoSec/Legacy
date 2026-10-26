//! Allocation-free mono delay primitive for graph compensation.

#[derive(Clone, Debug)]
pub struct DelayLine {
    buffer: Vec<f32>,
    cursor: usize,
    delay: usize,
}

impl DelayLine {
    #[must_use]
    pub fn new(delay_frames: usize) -> Self {
        Self {
            buffer: vec![0.0; delay_frames.max(1)],
            cursor: 0,
            delay: delay_frames,
        }
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: vec![0.0; capacity.max(1)],
            cursor: 0,
            delay: 0,
        }
    }

    /// Change delay using existing storage. Returns false if the requested
    /// delay exceeds the prepared capacity.
    pub fn set_delay(&mut self, delay_frames: usize) -> bool {
        if delay_frames >= self.buffer.len() {
            return false;
        }
        if self.delay != delay_frames {
            self.delay = delay_frames;
            self.clear();
        }
        true
    }

    #[must_use]
    pub const fn delay(&self) -> usize {
        self.delay
    }

    pub fn process(&mut self, samples: &mut [f32]) {
        if self.delay == 0 {
            return;
        }
        for sample in samples {
            let read = (self.cursor + self.buffer.len() - self.delay) % self.buffer.len();
            let delayed = self.buffer[read];
            self.buffer[self.cursor] = *sample;
            *sample = delayed;
            self.cursor = (self.cursor + 1) % self.buffer.len();
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0.0);
        self.cursor = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delays_and_preserves_block_boundaries() {
        let mut delay = DelayLine::new(2);
        let mut first = [1.0, 2.0];
        delay.process(&mut first);
        assert!(first.iter().all(|sample| sample.abs() < f32::EPSILON));
        let mut second = [3.0, 4.0];
        delay.process(&mut second);
        assert!((second[0] - 1.0).abs() < f32::EPSILON);
        assert!((second[1] - 2.0).abs() < f32::EPSILON);
    }
}
