// original: 0x009a37a0 radio_volume_update
/// Updates a radio emitter volume and forwards it to the member at 0x4c.
///
/// Scales a global rate by a global factor, truncates it to an integer
/// threshold, and scales the signed word at offset 0x98 by a global gain.
/// When the base at offset 0x14 plus a global bias exceeds the threshold,
/// the scaled value is forwarded as is; otherwise a second global span is
/// added and, when that still exceeds the threshold with a nonzero span, the
/// forwarded value is one minus the fractional position within the span,
/// times the scaled value; in the remaining case zero is forwarded. The
/// original computes the unsigned-to-float conversions through an x87 double
/// fixup table and the truncation through a chop-mode fistp; the rewrite uses
/// the equivalent Rust conversions, which round identically. The word at
/// offset 0x24 is cleared on entry and exit.
export!(thiscall, rw_009a37a0(this: u32) -> () {
    unsafe {
        const RATE: u32 = 0x115dbf4;
        const FACTOR: u32 = 0xfe8c58;
        const BIAS: u32 = 0x1038e50;
        const GAIN: u32 = 0x1038e4c;
        const SPAN: u32 = 0x1038e54;
        const ONE: u32 = 0xfe88e8;
        let tune = *((this.wrapping_add(0x98)) as *const i16) as f32;
        let base = *((this.wrapping_add(0x14)) as *const u32);
        *((this.wrapping_add(0x24)) as *mut u32) = 0;
        let f0 = *global::<f32>(RATE) * *global::<f32>(FACTOR);
        let bias = *global::<u32>(BIAS);
        let scaled = tune * *global::<f32>(GAIN);
        let limit = (f0 as i64) as u32;
        let v: f32;
        if base.wrapping_add(bias) > limit {
            v = scaled;
        } else {
            let span = *global::<u32>(SPAN);
            if base.wrapping_add(span).wrapping_add(bias) <= limit || span == 0 {
                v = 0.0;
            } else {
                let num = limit.wrapping_sub(base).wrapping_sub(bias) as f32;
                let den = span as f32;
                v = (*global::<f32>(ONE) - num / den) * scaled;
            }
        }
        callee_thiscall!(1, u32, this.wrapping_add(0x4c), v.to_bits());
        *((this.wrapping_add(0x24)) as *mut u32) = 0;
    }
});
