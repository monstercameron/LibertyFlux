// original: 0x00b9e480 CHECK_NM_FEEDBACK
/// Script native `CHECK_NM_FEEDBACK` (hash 0x7C4C63EF).
///
/// Forwards three script arguments to the engine and stores the low byte of
/// its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9e480(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
