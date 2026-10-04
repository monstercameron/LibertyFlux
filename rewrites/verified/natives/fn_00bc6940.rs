// original: 0x00bc6940 IS_CAR_MODEL
/// Script native `IS_CAR_MODEL` (hash 0x03D16145).
///
/// Forwards two script arguments (a car handle and a model hash) to the
/// engine and stores the low byte of the engine answer (zero-extended)
/// into the return slot.
export!(cdecl, rw_00bc6940(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
