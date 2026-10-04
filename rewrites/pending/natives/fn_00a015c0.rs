// original: 0x00a015c0 IS_OBJECT_PLAYING_ANIM
/// Script native `IS_OBJECT_PLAYING_ANIM` (hash 0x4D2E58D5).
///
/// Forwards three script arguments (an object handle and two animation
/// identifiers) to the engine and stores the low byte of its answer
/// (zero-extended) into the return slot.
export!(cdecl, rw_00a015c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
