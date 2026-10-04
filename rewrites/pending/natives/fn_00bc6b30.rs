// original: 0x00bc6b30 IS_CAR_TYRE_BURST
/// Script native `IS_CAR_TYRE_BURST` (hash 0x1DF623F9).
///
/// Forwards two script arguments (a vehicle handle and a wheel index) to the engine and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bc6b30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
