//! Sample-rate-independent linear parameter smoothing.

#[derive(Clone, Copy, Debug)]
pub struct SmoothedValue {
    current: f32,
    target: f32,
    remaining: u32,
}

impl SmoothedValue {
    #[must_use]
    pub const fn new(value: f32) -> Self {
        Self {
            current: value,
            target: value,
            remaining: 0,
        }
    }

    pub fn set_target(&mut self, target: f32, frames: u32) {
        self.target = target;
        self.remaining = frames;
        if frames == 0 {
            self.current = target;
        }
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn advance(&mut self) -> f32 {
        if self.remaining > 0 {
            self.current += (self.target - self.current) / self.remaining as f32;
            self.remaining -= 1;
        } else {
            self.current = self.target;
        }
        self.current
    }

    #[must_use]
    pub const fn value(self) -> f32 {
        self.current
    }

    #[must_use]
    pub const fn target(self) -> f32 {
        self.target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reaches_target_without_a_step() {
        let mut value = SmoothedValue::new(0.0);
        value.set_target(1.0, 4);
        let samples: Vec<_> = (0..4).map(|_| value.advance()).collect();
        assert!(samples.windows(2).all(|window| window[0] <= window[1]));
        assert!((value.advance() - 1.0).abs() < f32::EPSILON);
    }
}
