// original: 0x00b94820 IS_BIT_SET
/// Script native `IS_BIT_SET` (hash 0x5373544E).
///
/// Forwards two script arguments (an address and a bit index) to the engine
/// and stores the low byte of its answer (zero-extended) into the return
/// slot.
export!(cdecl, rw_00b94820(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
