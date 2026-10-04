// original: 0x00acfd10 audio_gated_forward
/// Audio gate-and-forward for one listener slot (original 0xACFD10).
///
/// `this` is the audio object, `a1` a parameter block, `a2` a forwarded
/// threshold word, `a0`/`a3`/`a4` opaque words passed through to the engine.
/// Two independent gated forwards run in sequence, both addressed to the
/// engine singleton:
/// 1. If `|this.gain|` exceeds `a1.range`, forward a clamped excess scaled by
///    `a1.scale`, plus a weight derived from `this.level` against the global
///    `[lo, hi]` band (0 below, 1 above, linear inside).
/// 2. If the length of `this.offset` exceeds `a1.radius`, forward a clamped
///    overshoot scaled by `a1.falloff`, plus `a2`.
/// Returns the second forward's answer, or `a4` when it is skipped. The
/// first forward's answer is discarded.
export!(thiscall, rw_b45_fd10(this: *mut u8, a0: u32, a1: *const u8, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        let cap = *global::<f32>(0xFE88E8);
        let engine = relocated(0x16D9F58);
        let mut out = a4;
        let raw_gain = *(this.add(0x130) as *const u32);
        let abs_mask = *global::<u32>(0xFE8F80);
        let gain = f32::from_bits(raw_gain & abs_mask);
        let range = *(a1.add(0x20) as *const f32);
        if gain > range {
            let excess = (gain - range) * *(a1.add(0x28) as *const f32);
            let excess = if cap > excess { excess } else { cap };
            let level = *(this.add(0x8C) as *const f32);
            let hi = *global::<f32>(0xECA808);
            let weight = if level > hi {
                cap
            } else {
                let lo = *global::<f32>(0xECA7FC);
                if level > lo {
                    (level - lo) / (hi - lo)
                } else {
                    0.0
                }
            };
            let _: u32 = callee_thiscall!(1, u32, engine, this as u32, a0, a4, a1 as u32,
                excess.to_bits(), a2, weight.to_bits(), a3);
        }
        let dx = *(this.add(0x110) as *const f32);
        let dy = *(this.add(0x114) as *const f32);
        let dz = *(this.add(0x118) as *const f32);
        let mut dist2 = dx * dx;
        dist2 += dy * dy;
        dist2 += dz * dz;
        let radius = *(a1.add(0x30) as *const f32);
        let r2 = radius * radius;
        if dist2 > r2 {
            let over = (dist2.sqrt() - radius) * *(a1.add(0x38) as *const f32);
            let over = if cap > over { over } else { cap };
            out = callee_thiscall!(2, u32, engine, this as u32, a0, a4, a1 as u32,
                over.to_bits(), a2, a3);
        }
        out
    }
});
