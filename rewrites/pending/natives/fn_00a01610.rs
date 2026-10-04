// original: 0x00a01610 IS_OBJECT_TOUCHING_OBJECT
/// Script native `IS_OBJECT_TOUCHING_OBJECT` (hash 0x6A2E514F).
///
/// Forwards two script arguments (two object handles) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00a01610(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
