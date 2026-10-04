// original: 0x00bb9510 TASK_COMBAT_HATED_TARGETS_IN_AREA
/// Script native `TASK_COMBAT_HATED_TARGETS_IN_AREA` (hash 0x06B840F1).
///
/// Reads five script arguments (a character handle and four float area
/// bounds) and calls the engine with eight arguments: the handle, the
/// four bounds, then the first three bounds repeated. The original builds
/// the whole outgoing argument area with moves into reserved stack space
/// (no pushes), which is why the repetition is visible in the call; the
/// rewrite expresses the same eight forwarded words directly. Floats are
/// copied as raw bits, so the forward is bit-exact.
export!(cdecl, rw_00bb9510(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let a0 = *args;
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        let a3 = *args.add(3);
        let a4 = *args.add(4);
        callee_cdecl!(1, u32, a0, a1, a2, a3, a4, a1, a2, a3)
    }
});
