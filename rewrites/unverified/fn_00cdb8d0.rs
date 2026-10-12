// original: 0x00CDB8D0 CTaskSimpleNMOnFire::vf20

/// Arm an on-fire task on query miss.
///
/// Runs message query 7 (patched cdecl callee) against the payload at
/// `a1 + 0xc`; a zero answer sets bit 1 of the flag word at `+0x28` and the
/// armed byte at `+0x40`. Returns the query answer.
///
/// Original: 0x00CDB8D0 (thiscall, two stack words of which the second is not
/// read, callee pops).
lf_checker_rt::export!(thiscall, rw_00cdb8d0(this: u32, a1: u32, a2: u32) -> u32 {
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
        const MSG: u32 = 0x00EDB228;

        let ans = lf_checker_rt::callee_cdecl!(QUERY, u32, MSG, a1.wrapping_add(0x0c), 7);
        if ans == 0 {
            wr32(this + 0x28, rd32(this + 0x28) | 2);
            ((this + 0x40) as *mut u8).write(1);
        }
        ans
    }
});
