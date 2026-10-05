// original: 0x00b9b0b0 NativeImpl_ADD_SPAWN_BLOCKING_DISC

/// Adds a spawn-blocking disc from five float arguments.
///
/// Copies the first three floats (`x`, `y`, `z`) into an aligned frame
/// buffer of [x, y, z, scratch] (the fourth word is frame scratch, zero
/// under the contract's `stack_fill`) and passes a pointer to it through
/// the single `ADD` call on the global object `OBJ`. The remaining two
/// floats are staged in dead scratch below the buffer and never read.
/// Returns whatever the callee returns.
///
/// The buffer pointer is a skipped call argument with snapshot-verified
/// contents (see `narrowed`).
///
/// Original: 0x00B9B0B0 (cdecl, five stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b9b0b0(x: u32, y: u32, z: u32, r0: u32, r1: u32) -> u32 {
    const OBJ: u32 = 0x01177A80;
    const ADD: u32 = 1;
    let _ = (r0, r1);
    let mut buf = [x, y, z, 0];
    lf_checker_rt::callee_thiscall!(
        ADD,
        u32,
        lf_checker_rt::relocated(OBJ),
        buf.as_mut_ptr() as u32
    )
});
