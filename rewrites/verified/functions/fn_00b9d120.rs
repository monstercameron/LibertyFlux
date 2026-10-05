// original: 0x00b9d120 NativeImpl_REGISTER_PLAYER_RESPAWN_COORDS_2

/// Registers player respawn coordinates from a selector and a triple.
///
/// Passes `sel` plus a frame buffer of [x, y, z, scratch] (the fourth
/// word is frame scratch, zero under the contract's `stack_fill`) through
/// the 2-argument `REG` call on the global object `OBJ`. Returns whatever
/// the callee returns.
///
/// The buffer pointer is a skipped call argument with snapshot-verified
/// contents (see `narrowed`).
///
/// Original: 0x00B9D120 (cdecl, four stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b9d120(sel: u32, x: u32, y: u32, z: u32) -> u32 {
    const OBJ: u32 = 0x01177A80;
    const REG: u32 = 1;
    let mut buf = [x, y, z, 0];
    lf_checker_rt::callee_thiscall!(
        REG,
        u32,
        lf_checker_rt::relocated(OBJ),
        sel,
        buf.as_mut_ptr() as u32
    )
});
