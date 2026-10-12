// original: 0x00CDB230 CTaskSimpleNMBrace::vf23

/// Refresh a brace task's mode word when its timer lapsed.
///
/// Same gate as the balance task: the tick counter global past the wrapping
/// sum of the 16-bit word at `+0x20` and the word at `+0x24` (unsigned) runs
/// the behaviour query (patched stdcall callee) with the two incoming words,
/// and a nonzero low byte of its answer rewrites the mode word at `+0x1c`
/// (masked set, low byte forced to 4) and sets bit 2 of the flag byte at
/// `+0x14`. Returns the sum, the query answer, or the mode word.
///
/// Original: 0x00CDB230 (thiscall, two stack words, callee pops).
lf_checker_rt::export!(thiscall, rw_00cdb230(this: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const QUERY: u32 = 1;
        const TICK_SLOT: u32 = 0x011735B4;
        const MODE_KEEP: u32 = 0xFF0845FF;
        const MODE_SET: u32 = 0x00084500;

        let lo = ((this + 0x20) as *const u16).read_unaligned() as u32;
        let sum = lo.wrapping_add(rd32(this + 0x24));
        let tick = lf_checker_rt::global::<u32>(TICK_SLOT).read();
        if tick <= sum {
            return sum;
        }
        let ans = lf_checker_rt::callee_stdcall!(QUERY, u32, a1, a2);
        if (ans & 0xFF) == 0 {
            return ans;
        }
        ((this + 0x14) as *mut u8).write(((this + 0x14) as *const u8).read() | 4);
        let mode = (rd32(this + 0x1c) & MODE_KEEP) | MODE_SET;
        wr32(this + 0x1c, mode);
        ((this + 0x1c) as *mut u8).write(4);
        mode
    }
});
