// original: 0x00986090 audEmitter_slot_empty_check (proposed)

/// Inverted slot test through the shared emitter object: reports whether a
/// slot's flag word is zero.
///
/// Forwards `index` to the slot flag test (0x009863F0) with `this` fixed to
/// the shared object at `SHARED_EMITTER`, and returns 1 when that test
/// reports 0 (the slot word is zero), else 0. Only the low byte of the
/// callee's answer is read.
/// Original: cdecl, one stack word (caller cleans; the inner call pops only
/// its own pushed copy), return in `al`.
lf_checker_rt::export!(cdecl, rw_00986090(index: u32) -> u32 {
    const SHARED_EMITTER: u32 = 0x12389e0;
    const SLOT_TEST: u32 = 1;
    unsafe {
        let ans: u32 = lf_checker_rt::callee_thiscall!(
            SLOT_TEST,
            u32,
            lf_checker_rt::relocated(SHARED_EMITTER),
            index
        );
        ((ans as u8) == 0) as u32
    }
});
