// original: 0x0087c0a0 rage::crmtNodeAddSubtract::vf2
/// Scale one source sample and accumulate it into this node's value.
///
/// Reads the unsigned index at `src+0x24`, loads the float at
/// `src+index*4`, multiplies it by the factor at `this+0x24` and adds the
/// accumulator at `this+0x20`, storing the sum back to `this+0x20`. The
/// multiply runs before the add, in the original's operand order (pinned
/// against reassociation so NaN signs and payloads match bit for bit).
/// Returns the index.
///
/// Original: thiscall/1, no calls. The index is unsigned (scaled by 4 with
/// no sign extension).
export!(thiscall, rw_0087c0a0(this: u32, src: u32) -> u32 {
    /// Index word in the source block; sample stride is 4 bytes.
    const IDX_OFF: u32 = 0x24;
    /// Scale factor in this node.
    const FACTOR_OFF: u32 = 0x24;
    /// Accumulator in this node, read and rewritten.
    const ACCUM_OFF: u32 = 0x20;
    #[inline(always)]
    fn mul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    #[inline(always)]
    fn add(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }
    unsafe {
        let idx = ((src + IDX_OFF) as *const u32).read_unaligned();
        let sample = ((src + idx.wrapping_mul(4)) as *const f32).read_unaligned();
        let factor = ((this + FACTOR_OFF) as *const f32).read_unaligned();
        let accum = ((this + ACCUM_OFF) as *const f32).read_unaligned();
        ((this + ACCUM_OFF) as *mut f32).write_unaligned(add(mul(sample, factor), accum));
        idx
    }
});
