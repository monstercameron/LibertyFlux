// original: 0x00bd7160 COMPARE_TWO_DATES
/// Script native `COMPARE_TWO_DATES` (hash 0x116D009A).
///
/// Forwards four script arguments (two packed dates) to the engine and
/// stores the full 32-bit answer into the return slot.
export!(cdecl, rw_00bd7160(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const u32) as *mut u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
        );
        *slot = answer;
        answer
    }
});
