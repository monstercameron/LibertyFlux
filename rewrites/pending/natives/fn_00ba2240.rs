// original: 0x00ba2240 SET_NM_MESSAGE_FLOAT
/// Script native `SET_NM_MESSAGE_FLOAT` (hash 0x6CE00370).
///
/// Forwards two script arguments (a natural-motion message id and a float
/// bit-pattern value) to the engine. The float is copied as raw bits, so the
/// forward is bit-exact. No return slot is written.
export!(cdecl, rw_00ba2240(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
