// original: 0x0088fa10 audio_accumulate_ratio_value
/// Accumulates a curve-shaped completion ratio into `this+0x28`.
///
/// When the end has been reached (or was never set), shapes 0.0 through
/// the curve helper and records that the range is done; otherwise shapes
/// `(end - pos) / (end - start)`. Adds the answer to the accumulator.
/// Returns 0 only when the range is done and the one-shot flag at byte
/// `0x3A` bit 3 is set; otherwise 1.
export!(thiscall, rw_0088fa10(this_ptr: *mut u8, pos: u32) -> u8 {
    unsafe {
        let w = |off: usize| *(this_ptr.add(off) as *const u32);
        let end = w(0x8C);
        let start = w(0x88);
        let mut done = false;
        let ratio: f32;
        if end <= start || pos >= end {
            done = true;
            ratio = 0.0;
        } else {
            let num = end.wrapping_sub(pos);
            let den = end.wrapping_sub(start);
            ratio = (num as f32) / (den as f32);
        }
        let ans: f32 = callee_cdecl!(1, f32, ratio.to_bits());
        let acc = this_ptr.add(0x28) as *mut f32;
        *acc = *acc + ans;
        if done && *this_ptr.add(0x3A) & 8 != 0 {
            return 0;
        }
        1
    }
});
