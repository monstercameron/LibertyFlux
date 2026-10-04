// original: 0x00bd0e60 GET_IS_PROJECTILE_TYPE_IN_AREA
/// Script native `GET_IS_PROJECTILE_TYPE_IN_AREA` (hash 0x7B2E70F3).
///
/// Forwards seven script words (six area-bound float bit patterns plus a
/// projectile type id) to the engine and stores the low byte of its answer
/// (zero-extended) into the return slot.
export!(cdecl, rw_00bd0e60(ctx: *const u8) -> u32 {
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
            *args.add(6)
        );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
