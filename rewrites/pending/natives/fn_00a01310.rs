// original: 0x00a01310 IS_MONEY_PICKUP_AT_COORDS
/// Script native `IS_MONEY_PICKUP_AT_COORDS` (hash 0x43167C6E).
///
/// Forwards three script arguments (a coordinate triple) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
///
/// The coordinates are floats, forwarded as raw bits.
export!(cdecl, rw_00a01310(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
