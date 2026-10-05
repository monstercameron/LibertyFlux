// original: 0x00b9b060 NativeImpl_ADD_SPAWN_BLOCKING_AREA

/// Adds a spawn-blocking area from four float arguments.
///
/// Copies the four floats (`x`, `y`, `z`, `r`) into an aligned frame
/// buffer of [x, y, z, scratch] (the fourth word is frame scratch, zero
/// under the contract's `stack_fill`) and passes a pointer to it through
/// the single `ADD` call on the global object `OBJ`. The radius word is
/// also staged twice in dead scratch below the buffer. Returns whatever
/// the callee returns.
///
/// The buffer pointer is a skipped call argument with snapshot-verified
/// contents (see `narrowed`).
///
/// Original: 0x00B9B060 (cdecl, four stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b9b060(x: u32, y: u32, z: u32, r: u32) -> u32 {
    const OBJ: u32 = 0x01177A80;
    const ADD: u32 = 1;
    let _ = r;
    let mut buf = [x, y, z, 0];
    lf_checker_rt::callee_thiscall!(
        ADD,
        u32,
        lf_checker_rt::relocated(OBJ),
        buf.as_mut_ptr() as u32
    )
});
