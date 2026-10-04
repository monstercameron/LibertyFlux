// original: 0x005e7ec0 LOAD_WEB_PAGE
/// Script native handler `LOAD_WEB_PAGE` (hash 0x78C17971).
///
/// Forwards script arguments 0..1 to the engine worker and returns its answer.
export!(cdecl, rw_005e7ec0(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let answer: u32 = callee_cdecl!(1, u32, a0, a1);
        answer
    }
});
