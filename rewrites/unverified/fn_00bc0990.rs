// original: 0x00bc0990 task_request_83_clamped (proposed)

/// Request task kind 0x83: clamp a level to [0, LIMIT], pick a mode from
/// two bits of a selector word, then forward both with a float word to
/// the shared dispatcher. The clamp keeps NaN bit-identical (an unordered
/// comparison keeps the input) and passes -0.0 through. Mode is 6 when
/// bit 9 is set, else 4 when bit 11 is set, else 3. Forwards (TAG=0x83,
/// modebits, level, retaddr, mode, fbits, extra, modebits, level); the
/// fourth word re-pushes the caller's return address and is skipped by
/// the proof. Returns the shifted selector when bit 9 is set, else 4.
///
/// Original: 0x00BC0990 (cdecl, 7 stack words).
lf_checker_rt::export!(cdecl, rw_00bc0990(a0: u32, a1: u32, a2: u32, fbits: u32, extra: u32, modebits: u32, level: u32) -> u32 {
    const TAG: u32 = 0x83;
    const BIT_HI: u32 = 1 << 9;
    const BIT_LO: u32 = 1 << 11;
    const LIMIT: u32 = 0xFE88E8; // file VA of the clamp limit
    let limit: f32 = unsafe { (lf_checker_rt::relocated(LIMIT) as *const f32).read_unaligned() };
    let x: f32 = f32::from_bits(level);
    // Ordered comparisons: NaN fails both and falls through unchanged.
    let clamped: f32 = if x < 0.0 { 0.0 } else if x > limit { limit } else { x };
    let mode: u32 = if modebits & BIT_HI != 0 { 6 } else if modebits & BIT_LO != 0 { 4 } else { 3 };
    let retv: u32 = if modebits & BIT_HI != 0 { modebits >> 9 } else { 4 };
    lf_checker_rt::callee_cdecl!(1, u32, TAG, modebits, clamped.to_bits(), 0, mode, fbits, extra, modebits, clamped.to_bits());
    retv
});
