// original: 0x00CDB690 CTaskSimpleNMFallDown::vf20

/// Latch the tick counter into a fall-down task on query miss.
///
/// Runs message query 7 (patched cdecl callee) against the payload at
/// `a1 + 0xc`; a zero answer copies the tick counter global into the latch
/// word at `+0x48`. Returns the query answer.
///
/// Original: 0x00CDB690 (thiscall, two stack words of which the second is not
/// read, callee pops).
lf_checker_rt::export!(thiscall, rw_00cdb690(this: u32, a1: u32, a2: u32) -> u32 {
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
        const MSG: u32 = 0x00EDCD34;
        const TICK_SLOT: u32 = 0x011735B4;
        const LATCH: u32 = 0x48;

        let ans = lf_checker_rt::callee_cdecl!(QUERY, u32, MSG, a1.wrapping_add(0x0c), 7);
        if ans == 0 {
            wr32(this + LATCH, lf_checker_rt::global::<u32>(TICK_SLOT).read());
        }
        ans
    }
});
