// original: 0x00bc4fd0 ADD_STUCK_CAR_CHECK
/// Script native `ADD_STUCK_CAR_CHECK` (hash 0x03A01B12).
///
/// Forwards three script arguments to the engine: two integers and one float
/// bit-pattern (copied as raw bits, so the forward is bit-exact). No return
/// slot is written.
export!(cdecl, rw_00bc4fd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
