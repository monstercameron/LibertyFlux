// original: 0x00cbaa10 CTaskSimpleSlideToCoord::vf13
/// Commit a slide update to its sink (vf13, 1 call).
///
/// Calls the five-argument sink with the block at `this + 0xC0` and the
/// coordinate words at `[this + 0xE0]` and `[this + 0xE4]` (thiscall, two
/// stack arguments; both unread). The original also passes two slots it
/// reads half-overlapping its own return address and below-stack scratch;
/// those values depend on caller internals the rewrite cannot observe, so
/// the rewrite passes zero and the contract skips them. The callee is
/// intercepted by the checker. No reads or writes of its own, no return.
lf_checker_rt::export!(thiscall, rw_00cbaa10(this: u32, _a0: u32, _a1: u32) -> u32 {
    unsafe {
        /// Block offset passed as the first sink argument.
        const BLOCK_OFF: u32 = 0xC0;
        /// Coordinate words passed as the fourth and fifth arguments.
        const FIRST_OFF: u32 = 0xE0;
        const SECOND_OFF: u32 = 0xE4;
        /// Callee id of the sink.
        const SINK: u32 = 1;
        let f0 = ((this + FIRST_OFF) as *const u32).read_unaligned();
        let f1 = ((this + SECOND_OFF) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_cdecl!(
            SINK, u32, this.wrapping_add(BLOCK_OFF), 0, 0, f0, f1);
        0
    }
});
