// original: 0x00655150 rage::ptxSprite::vf15

/// Drain `arg`'s sprite list into this sprite's pool when `arg` is non-null
/// and non-empty, tail-calling the pool drain with the pool header at
/// `this + 0x184`. An empty list returns at once: null for a null `arg`,
/// `arg` itself when its head word is zero. Thiscall, one stack argument.
lf_checker_rt::export!(thiscall, rw_00655150(this: u32, arg: u32) -> u32 {
    unsafe {
        const POOL_HEADER: u32 = 0x184;
        const DRAIN_CALLEE: u32 = 1;
        if arg == 0 {
            return 0;
        }
        let head = (arg as *const u32).read_unaligned();
        if head == 0 {
            return arg;
        }
        let pool =
            ((this.wrapping_add(POOL_HEADER)) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(DRAIN_CALLEE, u32, pool, arg)
    }
});
