// original: 0x00bb94c0 TASK_COMBAT_HATED_TARGETS_AROUND_CHAR
/// Script native `TASK_COMBAT_HATED_TARGETS_AROUND_CHAR` (hash 0x127669D3).
///
/// Forwards two script arguments to the engine: a character handle and one
/// float bit-pattern (the search radius). No return slot is written.
export!(cdecl, rw_00bb94c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
