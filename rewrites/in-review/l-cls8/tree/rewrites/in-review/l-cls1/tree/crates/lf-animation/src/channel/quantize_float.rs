//! A quantized float channel: integer keys scaled into floats.
//!
//! Only one slot of `crAnimChannelQuantizeFloat` is lifted: the evaluator
//! that scales two raw integer samples and blends them. The key store, the
//! extractor, the samplers built on them and the size are not lifted (the
//! samplers and size need the extractor callee or are unverified), so
//! this type holds only the scale and bias the lifted slot reads.

/// Scale and bias of a quantized float channel; the keys are not modelled yet.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct QuantizeFloat {
    /// Multiplier applied to each raw sample.
    scale: f32,
    /// Offset added after scaling.
    bias: f32,
}

impl QuantizeFloat {
    /// A channel fragment with `scale` and `bias`.
    #[must_use]
    pub const fn new(scale: f32, bias: f32) -> Self {
        Self { scale, bias }
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
