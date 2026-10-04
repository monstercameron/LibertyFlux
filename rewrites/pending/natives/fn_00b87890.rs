// original: 0x00b87890 SET_DANCE_SHAKE_ACTIVE_THIS_UPDATE
/// Script native `SET_DANCE_SHAKE_ACTIVE_THIS_UPDATE` (hash 0x1E880709).
///
/// Forwards one float script argument to the engine as raw bits, so the forward is bit-exact.
export!(cdecl, rw_00b87890(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
