// original: 0x00CDB1D0 CTaskSimpleNMBalance::vf23

/// Refresh a balance task's behaviour flags when its timer lapsed.
///
/// Adds the 16-bit word at `+0x20` to the word at `+0x24` (wrapping) and
/// compares the tick counter global against the sum, unsigned: when the tick
/// is past the sum, the behaviour query (patched stdcall callee) runs with
/// the two incoming words, and a nonzero low byte of its answer rewrites the
/// mode word at `+0x1c` (masked set, low byte forced to 4), sets bit 2 of the
/// flag byte at `+0x14` and rewrites the state word at `+0xc0` (masked set).
/// Returns the last value produced on its path: the sum, the query answer, or
/// the state word.
///
/// Original: 0x00CDB1D0 (thiscall, two stack words, callee pops).
lf_checker_rt::export!(thiscall, rw_00cdb1d0(this: u32, a1: u32, a2: u32) -> u32 {
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
        const TIMER_LO: u32 = 0x20;
        const TIMER_HI: u32 = 0x24;
        const FLAGS: u32 = 0x14;
        const MODE: u32 = 0x1c;
        const STATE: u32 = 0xc0;
        const MODE_KEEP: u32 = 0xFF0845FF;
        const MODE_SET: u32 = 0x00084500;
        const MODE_BYTE: u8 = 4;
        const STATE_KEEP: u32 = 0xFFF2FFFF;
        const STATE_SET: u32 = 0x00020000;

        let lo = ((this + TIMER_LO) as *const u16).read_unaligned() as u32;
        let sum = lo.wrapping_add(rd32(this + TIMER_HI));
        let tick = lf_checker_rt::global::<u32>(TICK_SLOT).read();
        if tick <= sum {
            return sum;
        }
        let ans = lf_checker_rt::callee_stdcall!(QUERY, u32, a1, a2);
        if (ans & 0xFF) == 0 {
            return ans;
        }
        ((this + FLAGS) as *mut u8).write(((this + FLAGS) as *const u8).read() | 4);
        let mode = (rd32(this + MODE) & MODE_KEEP) | MODE_SET;
        wr32(this + MODE, mode);
        ((this + MODE) as *mut u8).write(MODE_BYTE);
        let state = (rd32(this + STATE) & STATE_KEEP) | STATE_SET;
        wr32(this + STATE, state);
        state
    }
});
