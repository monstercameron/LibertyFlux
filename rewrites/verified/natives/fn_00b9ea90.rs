// original: 0x00b9ea90 FIRE_PED_WEAPON
/// Script native `FIRE_PED_WEAPON` (hash 0x25BB7D67).
///
/// Forwards a character handle and three float coordinates to the engine as seven words: the handle followed by the coordinate triple twice (the handler copies the triple into two adjacent stack slots). No return slot is written.
export!(cdecl, rw_00b9ea90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(1),
            *args.add(2),
            *args.add(3),
        )
    }
});
