// original: 0x00bd3810 ADD_SPHERE
/// Script native `ADD_SPHERE` (hash 0x42252652).
///
/// Forwards five script arguments to the engine: four float bit-patterns
/// (centre coordinates and radius) and one integer. Stores the engine's
/// full 32-bit answer into the return slot.
export!(cdecl, rw_00bd3810(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
