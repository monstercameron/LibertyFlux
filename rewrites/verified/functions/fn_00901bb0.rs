// original: 0x00901bb0 input_emit_quad (proposed)
/// Forward three arguments to the six-argument emitter with fixed tags.
///
/// Calls the emitter callee with `(0, 4, a2, a0, 4, a1)` (pushed in
/// reverse) and returns its answer. Cdecl with three stack words.
export!(cdecl, rw_00901bb0(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const EMIT_ID: u32 = 1;
        callee_cdecl!(EMIT_ID, u32, 0u32, 4u32, a2, a0, 4u32, a1)
    }
});
