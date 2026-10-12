// original: 0x00CDB890 CTaskSimpleNMHighFall::vf20

/// Arm a high-fall task and refresh its child on query miss.
///
/// Runs message query 7 (patched cdecl callee) against the payload at
/// `a2 + 0xc`; a zero answer sets the armed byte at `+0x40`. The child helper
/// (patched thiscall callee) then always runs on the child block at `+0x60`
/// with the payload pointer twice, and its answer is returned. The first
/// incoming word is not read.
///
/// Original: 0x00CDB890 (thiscall, two stack words, callee pops).
lf_checker_rt::export!(thiscall, rw_00cdb890(this: u32, a1: u32, a2: u32) -> u32 {
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
        const CHILD: u32 = 2;
        const MSG: u32 = 0x00EDCDBC;
        const CHILD_OFF: u32 = 0x60;
        const ARMED: u32 = 0x40;

        let payload = a2.wrapping_add(0x0c);
        let ans = lf_checker_rt::callee_cdecl!(QUERY, u32, MSG, payload, 7);
        if ans == 0 {
            ((this + ARMED) as *mut u8).write(1);
        }
        lf_checker_rt::callee_thiscall!(CHILD, u32, this + CHILD_OFF, a2, a2)
    }
});
