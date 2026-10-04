// original: 0x00b9fff0 IS_GROUP_MEMBER
/// Script native `IS_GROUP_MEMBER` (hash 0x674D6F8E).
///
/// Forwards two script arguments (handles) to the engine and stores the
/// low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b9fff0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
