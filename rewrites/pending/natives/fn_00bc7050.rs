// original: 0x00bc7050 MARK_CAR_AS_NO_LONGER_NEEDED
/// Script native `MARK_CAR_AS_NO_LONGER_NEEDED` (hash 0x20C76FD1).
///
/// Vehicle handle.
/// No return slot is written.
///
/// The handler returns the engine's answer in EAX (the register
/// is untouched after the call); the rewrite does the same.
export!(cdecl, rw_00bc7050(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
