// original: 0x00b9f610 HAS_CHAR_ANIM_FINISHED
/// Script native `HAS_CHAR_ANIM_FINISHED` (hash 0x53F34027).
///
/// Forwards three script arguments to the engine and stores the low byte of
/// its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9f610(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
