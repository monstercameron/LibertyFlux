// original: 0x00bc7010 LOCK_CAR_DOORS
/// Script native `LOCK_CAR_DOORS` (hash 0x6702757C).
///
/// Forwards a car handle and a lock-state value to the engine. No return
/// slot is written.
export!(cdecl, rw_00bc7010(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
        )
    }
});
