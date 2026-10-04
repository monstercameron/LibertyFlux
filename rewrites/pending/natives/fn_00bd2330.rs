// original: 0x00bd2330 IS_EXPLOSION_IN_SPHERE
/// Script native `IS_EXPLOSION_IN_SPHERE` (hash 0x47A77D2E).
///
/// Forwards five script arguments to the engine: an integer followed by
/// four float bit-patterns (sphere centre and radius). Stores the low byte
/// of the engine answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd2330(ctx: *const u8) -> u32 {
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
        *slot = answer & 0xFF;
        slot as u32
    }
});
