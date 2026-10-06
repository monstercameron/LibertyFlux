// original: 0x00901770 input_emit_full (proposed)
/// Forward five arguments to the six-argument emitter with a -1 tag.
///
/// Calls the emitter callee with `(a0, a1, -1, a2, a3, a4)` (pushed in
/// reverse) and returns its answer. Cdecl with five stack words.
export!(cdecl, rw_00901770(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const EMIT_ID: u32 = 1;
        const TAG: u32 = 0xFFFFFFFF;
        callee_cdecl!(EMIT_ID, u32, a0, a1, TAG, a2, a3, a4)
    }
});
