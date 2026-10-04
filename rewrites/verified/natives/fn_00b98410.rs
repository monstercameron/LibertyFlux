// original: 0x00b98410 HAS_RELOADED_WITH_MOTION_CONTROL
/// Script native `HAS_RELOADED_WITH_MOTION_CONTROL` (hash 0x08C6502C).
///
/// Forwards two script arguments (a character handle and a weapon slot) to
/// the engine and stores the low byte of its answer (zero-extended) into
/// the return slot.
export!(cdecl, rw_00b98410(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
