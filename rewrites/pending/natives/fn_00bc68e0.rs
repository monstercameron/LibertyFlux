// original: 0x00bc68e0 IS_CAR_IN_GARAGE_AREA
/// Script native `IS_CAR_IN_GARAGE_AREA` (hash 0x005868E2).
///
/// Forwards two script arguments (a garage id and a vehicle handle) to
/// /// the engine and stores the low byte of its answer
/// /// (zero-extended) into the return slot.
export!(cdecl, rw_00bc68e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
