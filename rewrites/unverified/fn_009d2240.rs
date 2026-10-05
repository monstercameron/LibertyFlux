// original: 0x009D2240 stream_flagvec_seed (proposed)
//
/// Seeds a member vector with a fixed value sequence.
///
/// Resets the member vector at `this + 0x2c` (callee 1 with `0`), then appends six values
/// (0x80000000, 0, 1, 2, 4, 8) through the append helper (callee 2), each passed by address
/// through a frame slot (the values are observed via the call snapshot, so
/// the proof is the exact append sequence). All callee returns are ignored.
/// Returns nothing meaningful (`eax` is untouched incoming-register
/// passthrough). Thiscall, no arguments.
lf_checker_rt::export!(thiscall, rw_009D2240(this: u32) -> u32 {
    unsafe {
        const RESET: u32 = 1;
        const APPEND: u32 = 2;
        const VEC: u32 = 0x2c;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            RESET, u32, this.wrapping_add(VEC), 0);
        for v in [0x80000000, 0, 1, 2, 4, 8] {
            let mut slot: u32 = v;
            let _: u32 = lf_checker_rt::callee_thiscall!(
                APPEND, u32, this.wrapping_add(VEC), (&mut slot as *mut u32) as u32);
        }
        0
    }
});
