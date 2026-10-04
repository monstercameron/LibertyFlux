// original: 0x00bde340 GET_WATER_HEIGHT
/// Script native `GET_WATER_HEIGHT` (hash 0x2BB9620F).
///
/// Forwards four script arguments (three float bit-patterns for a position
/// and one integer) to the engine and stores the low byte of its answer
/// (zero-extended) into the return slot.
export!(cdecl, rw_00bde340(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
