// original: 0x00ba2880 SET_SENSE_RANGE
/// Script native `SET_SENSE_RANGE` (hash 0x44D56F66).
///
/// Forwards two script arguments (a character handle and a float
/// bit-pattern for the range) to the engine. The float is copied as raw
/// bits, so the forward is bit-exact. No return slot is written.
export!(cdecl, rw_00ba2880(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
