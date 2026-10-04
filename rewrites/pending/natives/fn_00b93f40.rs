// original: 0x00b93f40 CHEAT_HAPPENED_RECENTLY
/// Script native `CHEAT_HAPPENED_RECENTLY` (hash 0x7488454D).
///
/// Forwards two script arguments to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b93f40(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
