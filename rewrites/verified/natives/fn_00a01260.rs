// original: 0x00a01260 IS_ANY_PICKUP_AT_COORDS
/// Script native `IS_ANY_PICKUP_AT_COORDS` (hash 0x75DC4737).
///
/// Forwards three float bit-patterns (coordinates) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00a01260(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
