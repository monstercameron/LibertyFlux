// original: 0x0088f950 audio_accumulate_scaled_value
/// Accumulates a range-checked, curve-shaped value into `this+0x28`.
///
/// Rejects empty ranges and out-of-range positions (returns 0); positions
/// below the window return 1 unchanged. Otherwise maps the position into
/// 0..1, passes it through the curve helper, adds the answer to the
/// accumulator at `this+0x28`, and returns 1.
export!(thiscall, rw_0088f950(this_ptr: *mut u8, pos: u32) -> u8 {
    unsafe {
        let w = |off: usize| *(this_ptr.add(off) as *const u32);
        let window = w(0x44);
        if window == 0 {
            return 0;
        }
        let span = w(0x6C).wrapping_sub(w(0x58));
        // Both range checks are signed (jle/jg): reject only a positive
        // span that exceeds the window.
        if (span as i32) > 0 && (span as i32) > (window as i32) {
            return 0;
        }
        let start = w(0x60);
        let rel = pos.wrapping_sub(w(0x84));
        if rel >= start.wrapping_sub(span).wrapping_add(window) {
            return 0;
        }
        if rel < start {
            return 1;
        }
        let num = rel.wrapping_sub(start).wrapping_add(span);
        let mut ratio = (num as f32) / (window as f32);
        if ratio < 0.0 {
            ratio = 0.0;
        } else if ratio > 1.0 {
            ratio = 1.0;
        }
        let ans: f32 = callee_cdecl!(1, f32, ratio.to_bits());
        let acc = this_ptr.add(0x28) as *mut f32;
        *acc = *acc + ans;
        1
    }
});
