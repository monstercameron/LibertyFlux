//! A quantized float channel: integer keys scaled into floats.
//!
//! Two slots of `crAnimChannelQuantizeFloat` are lifted: the evaluator
//! that scales two raw integer samples and blends them, and the storage
//! size. The key store, the extractor and the samplers built on them are
//! not lifted (the samplers need the extractor callee), so this type
//! holds only the scale, the bias and the two counts the lifted slots
//! read.

/// Scale, bias and counts of a quantized float channel; the keys are not
/// modelled yet.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct QuantizeFloat {
    /// Multiplier applied to each raw sample.
    scale: f32,
    /// Offset added after scaling.
    bias: f32,
    /// Number of samples.
    count: u32,
    /// Per-sample units the size multiplies the count by.
    width: u32,
}

impl QuantizeFloat {
    /// A channel fragment with `scale` and `bias` and zero counts.
    #[must_use]
    pub const fn new(scale: f32, bias: f32) -> Self {
        Self {
            scale,
            bias,
            count: 0,
            width: 0,
        }
    }

    /// A channel fragment with `scale`, `bias`, `count` and `width`.
    #[must_use]
    pub const fn with_counts(mut self, count: u32, width: u32) -> Self {
        self.count = count;
        self.width = width;
        self
    }

    /// Byte size of the 32-bit storage form: `count * width` rounded
    /// down to whole 32-bit words, times four, plus a 32-byte header
    /// when any low bits remain, else a 28-byte header, wrapping exactly
    /// like the original.
    #[must_use]
    pub const fn storage_size(self) -> u32 {
        let n = self.count.wrapping_mul(self.width);
        let words = n >> 5;
        let header = if (n & 31) != 0 { 0x20u32 } else { 0x1cu32 };
        words.wrapping_mul(4).wrapping_add(header)
    }

    /// Scales two raw integer samples (`sample as f32 * scale + bias`
    /// each, in that order) and blends them at `t`: the difference times
    /// `t` plus the lower value, widened to `f64` exactly (the original
    /// returns it in floating point at double width).
    #[must_use]
    pub fn eval_scaled(self, a: u64, b: u64, t: f32) -> f64 {
        // Separate multiply and add, as verified: not a fused operation.
        let v0 = (a as f32) * self.scale + self.bias;
        let v1 = (b as f32) * self.scale + self.bias;
        f64::from((v1 - v0) * t + v0)
    }
}
