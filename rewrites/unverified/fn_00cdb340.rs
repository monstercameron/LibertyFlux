// original: 0x00CDB340 nmsub_mark_query_miss

/// Mark a NaturalMotion child done when its message query misses.
///
/// Runs message query 11 (patched cdecl callee) against the payload at
/// `a1 + 0xc`; a zero answer sets bit 12 of the flag word at `+0x60`. A
/// nonzero answer runs query 12 instead, whose zero answer sets the same bit.
/// Returns the deciding answer.
///
/// Original: 0x00CDB340 (thiscall, two stack words of which the second is not
/// read, callee pops). Name proposed: no merged symbol."""

lf_checker_rt::export!(thiscall, rw_00cdb340(this: u32, a1: u32, a2: u32) -> u32 {
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
        const QUERY2: u32 = 2;
        const MSG_A: u32 = 0x00EDCF34;
        const MSG_B: u32 = 0x00EDCF40;
        const PAYLOAD_OFF: u32 = 0x0c;
        const FLAG_WORD: u32 = 0x60;
        const DONE_BIT: u32 = 0x1000;

        let payload = a1.wrapping_add(PAYLOAD_OFF);
        let first = lf_checker_rt::callee_cdecl!(QUERY, u32, MSG_A, payload, 0x0b);
        if first == 0 {
            wr32(this + FLAG_WORD, rd32(this + FLAG_WORD) | DONE_BIT);
            return 0;
        }
        let second = lf_checker_rt::callee_cdecl!(QUERY2, u32, MSG_B, payload, 0x0c);
        if second == 0 {
            wr32(this + FLAG_WORD, rd32(this + FLAG_WORD) | DONE_BIT);
        }
        second
    }
});
