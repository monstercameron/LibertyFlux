// original: 0x00ba2180 SET_INFORM_RESPECTED_FRIENDS
/// Script native `SET_INFORM_RESPECTED_FRIENDS` (hash 0x509F236D).
///
/// Forwards three script arguments to the engine: an integer, one float
/// bit-pattern and an integer. The float is copied as raw bits. No return
/// slot is written.
export!(cdecl, rw_00ba2180(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
