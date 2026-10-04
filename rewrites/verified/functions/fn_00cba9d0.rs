// original: 0x00cba9d0 CTaskSimpleMoveSlideToCoord::vf13
/// Commit a slide-to-coordinate update to its sink (vf13, 1 call).
///
/// Calls the five-argument sink with the block at `this + 0x20` and the
/// coordinate words at `[this + 0x44]` and `[this + 0x40]` (thiscall, two
/// stack arguments; both unread). The original also passes two slots it
/// reads half-overlapping its own return address and below-stack scratch;
/// those values depend on caller internals the rewrite cannot observe, so
/// the rewrite passes zero and the contract skips them. The callee is
/// intercepted by the checker. No reads or writes of its own, no return.
lf_checker_rt::export!(thiscall, rw_00cba9d0(this: u32, _a0: u32, _a1: u32) -> u32 {
    unsafe {
        /// Block offset passed as the first sink argument.
        const BLOCK_OFF: u32 = 0x20;
        /// Coordinate words passed as the fourth and fifth arguments.
        const Y_OFF: u32 = 0x44;
        const Z_OFF: u32 = 0x40;
        /// Callee id of the sink.
        const SINK: u32 = 1;
        let fy = ((this + Y_OFF) as *const u32).read_unaligned();
        let fz = ((this + Z_OFF) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_cdecl!(
            SINK, u32, this.wrapping_add(BLOCK_OFF), 0, 0, fy, fz);
        0
    }
});
