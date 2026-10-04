// original: 0x00b949f0 IS_POINT_OBSCURED_BY_A_MISSION_ENTITY
/// Script native `IS_POINT_OBSCURED_BY_A_MISSION_ENTITY` (hash 0x7FBC713E).
///
/// Forwards six float script arguments to the engine as raw bit-patterns
/// and stores the low byte of its answer (zero-extended) into the return
/// slot. The floats are copied bit-exact; no arithmetic is done here.
export!(cdecl, rw_00b949f0(ctx: *const u8) -> u32 {
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
            *args.add(5),
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
