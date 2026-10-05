// original: 0x00877810 rage::crmtComposerOptimized::vf1

/// Reset a composer object to its freshly-created state.
///
/// `this` points to a composer object. A helper is run first (through the
/// patched call slot), then the pending list (head `+0x80`, tail `+0x84`),
/// the counters at `+0x90`/`+0x94` and the immediate-mode flag at `+0x98`
/// are cleared, and the free-node cache at `+0x88` is pointed at the inner
/// block (`+0x0c`) plus 0x1C. The inner pointer itself is left alone.
///
/// Original: 0x00877810 (thiscall, no stack arguments; EAX ends holding the
/// new free-node pointer, which the contract compares).
lf_checker_rt::export!(thiscall, rw_00877810(this: u32) -> u32 {
    unsafe {
        const INNER_OFF: u32 = 0x0c;
        const HEAD_OFF: u32 = 0x80;
        const TAIL_OFF: u32 = 0x84;
        const FREE_OFF: u32 = 0x88;
        const COUNT0_OFF: u32 = 0x90;
        const COUNT1_OFF: u32 = 0x94;
        const IMMED_OFF: u32 = 0x98;
        const FREE_BIAS: u32 = 0x1c;
        const HELPER: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        lf_checker_rt::callee_thiscall!(HELPER, u32, this);
        let free = rd32(this.wrapping_add(INNER_OFF)).wrapping_add(FREE_BIAS);
        wr8(this.wrapping_add(IMMED_OFF), 0);
        wr32(this.wrapping_add(TAIL_OFF), 0);
        wr32(this.wrapping_add(HEAD_OFF), 0);
        wr32(this.wrapping_add(FREE_OFF), free);
        wr32(this.wrapping_add(COUNT0_OFF), 0);
        wr32(this.wrapping_add(COUNT1_OFF), 0);
        free
    }
});
