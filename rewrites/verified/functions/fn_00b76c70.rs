// original: 0x00b76c70 state_gated_accumulate (proposed)

/// Add `delta` to the accumulator at `this+0x990`, but only when the state
/// byte at `this+0xbc8` is 0 or 1; any other state leaves memory untouched.
///
/// Returns `delta` when the add runs. Otherwise the original returns with
/// only al set to the state byte, so the low return byte is the state and
/// the upper bytes are the caller's leftover eax (compared low byte only).
///
/// Original: 0x00b76c70 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b76c70(this: u32, delta: u32) -> u32 {
    unsafe {
        const STATE_OFF: u32 = 0xbc8;
        const ACCUM_OFF: u32 = 0x990;
        let state = ((this + STATE_OFF) as *const u8).read();
        if state == 1 || state == 0 {
            let slot = (this + ACCUM_OFF) as *mut u32;
            let cur = slot.read_unaligned();
            slot.write_unaligned(cur.wrapping_add(delta));
            delta
        } else {
            state as u32
        }
    }
});
