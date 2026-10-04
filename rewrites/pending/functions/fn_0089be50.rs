// original: 0x0089be50 aud_blend_compute
/// Blends two counter-based levels and normalizes the engine's answer.
///
/// Converts two unsigned counters to float, adds each live float trim,
/// runs the engine blender over the pair, then caps the result: answers
/// above one are replaced by one's reciprocal share, anything else (or an
/// unordered comparison) yields exactly one. Returns the float in ST0.
export!(thiscall, rw_0089be50(this: *const u8) -> f64 {
    unsafe {
        let mut first = *(this.add(0xB4) as *const u32) as f32;
        let trim1 = *(this.add(0xC4) as *const u32);
        if trim1 != 0 {
            first += *(trim1 as *const f32);
        }
        let mut second = *(this.add(0xB8) as *const u32) as f32;
        let trim2 = *(this.add(0xC8) as *const u32);
        if trim2 != 0 {
            second += *(trim2 as *const f32);
        }
        let blended: f32 = callee_cdecl!(1, f32, first.to_bits(), second.to_bits());
        let cap = *global::<f32>(0x00FE88E8);
        if blended > cap {
            (cap / blended) as f64
        } else {
            1.0f64
        }
    }
});

