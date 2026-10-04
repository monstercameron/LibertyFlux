// original: 0x00ba2810 SET_SCRIPTED_ANIM_SEAT_OFFSET
/// Script native `SET_SCRIPTED_ANIM_SEAT_OFFSET` (hash 0x718939EF).
///
/// Forwards two script arguments to the engine: an integer and a float
/// bit-pattern. No return slot is written.
export!(cdecl, rw_00ba2810(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
